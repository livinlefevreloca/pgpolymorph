//! Decode PostgreSQL binary field payloads into IR [`PgValue`]s.

use crate::binary::{
    constants, BigEndianBytes, BufferView, FieldCell, PG_DATE_EPOCH_OFFSET_DAYS,
    PG_TIMESTAMP_EPOCH_OFFSET_US,
};
use crate::codec::array::decode_array;
use crate::error::{Error, Result};
use crate::schema::PgType;
use crate::value::pgtypes::{self, decode_numeric};
use crate::value::PgValue;

pub(crate) struct FieldDecoder<'a> {
    pub(crate) column: &'a str,
    pub(crate) ty: &'a PgType,
    nullable: bool,
}

impl<'a> FieldDecoder<'a> {
    pub fn new(column: &'a str, ty: &'a PgType, nullable: bool) -> Self {
        Self {
            column,
            ty,
            nullable,
        }
    }

    pub fn decode(&self, cell: FieldCell<'_>) -> Result<PgValue> {
        if cell.is_null {
            if !self.nullable {
                return Err(Error::UnexpectedNull {
                    column: self.column.to_string(),
                });
            }
            return Ok(PgValue::Null);
        }

        match self.ty {
            PgType::Array(_) => decode_array(self, cell),
            _ => self.decode_scalar(cell.payload),
        }
    }

    pub fn decode_array_element(&self, cell: FieldCell<'_>) -> Result<PgValue> {
        if cell.is_null {
            return Ok(PgValue::Null);
        }
        self.decode_scalar(cell.payload)
    }

    pub(crate) fn ensure_min_remaining(
        &self,
        view: &BufferView<'_>,
        min: usize,
        reason: &'static str,
    ) -> Result<()> {
        if view.remaining() < min {
            return Err(self.invalid_payload(reason));
        }
        Ok(())
    }

    fn decode_scalar(&self, mut view: BufferView<'_>) -> Result<PgValue> {
        match self.ty {
            PgType::Bool => self.decode_bool(&mut view),
            PgType::Bytea => Ok(PgValue::Bytea(pgtypes::PgBytea::new(
                view.read_remaining()?.to_vec(),
            ))),
            // PostgreSQL internal `char` is a 2-byte big-endian integer (see `PgChar`).
            PgType::Char => Ok(PgValue::Char(pgtypes::PgChar::new(
                self.read_be_exact(&mut view, constants::CHAR_PAYLOAD_BYTES)?,
            ))),
            PgType::Int2 => Ok(PgValue::Int2(pgtypes::PgInt2::new(
                self.read_be_exact(&mut view, constants::INT2_PAYLOAD_BYTES)?,
            ))),
            PgType::Int4 => Ok(PgValue::Int4(pgtypes::PgInt4::new(
                self.read_be_exact(&mut view, constants::INT4_PAYLOAD_BYTES)?,
            ))),
            PgType::Int8 => Ok(PgValue::Int8(pgtypes::PgInt8::new(
                self.read_be_exact(&mut view, constants::INT8_PAYLOAD_BYTES)?,
            ))),
            PgType::Float4 => Ok(PgValue::Float4(pgtypes::PgFloat4::new(
                self.read_be_exact(&mut view, constants::FLOAT4_PAYLOAD_BYTES)?,
            ))),
            PgType::Float8 => Ok(PgValue::Float8(pgtypes::PgFloat8::new(
                self.read_be_exact(&mut view, constants::FLOAT8_PAYLOAD_BYTES)?,
            ))),
            PgType::Text => Ok(PgValue::Text(pgtypes::PgText::new(
                self.read_utf8_remaining(&mut view)?,
            ))),
            PgType::Varchar(max_len) => {
                let varchar = pgtypes::PgVarchar::new(self.read_utf8_remaining(&mut view)?, *max_len);
                self.validate_varchar(&varchar)?;
                Ok(PgValue::Varchar(varchar))
            }
            PgType::Name => Ok(PgValue::Name(pgtypes::PgName::new(
                self.read_utf8_remaining(&mut view)?,
            ))),
            PgType::Json => Ok(PgValue::Json(pgtypes::PgJson::new(
                self.read_utf8_remaining(&mut view)?,
            ))),
            PgType::Jsonb => self.decode_jsonb(&mut view),
            PgType::Date => self.decode_date(&mut view),
            PgType::Time => Ok(PgValue::Time(pgtypes::PgTime::new(
                self.read_be_exact(&mut view, constants::TIME_PAYLOAD_BYTES)?,
            ))),
            PgType::Timestamp => self.decode_timestamp(&mut view),
            PgType::Timestamptz => self.decode_timestamptz(&mut view),
            PgType::Timetz => self.decode_timetz(&mut view),
            PgType::Interval => self.decode_interval(&mut view),
            PgType::Numeric => Ok(PgValue::Numeric(decode_numeric(
                view,
                self.column,
                self.ty,
            )?)),
            PgType::Uuid => self.decode_uuid(&mut view),
            PgType::Money => Ok(PgValue::Money(pgtypes::PgMoney::new(
                self.read_be_exact(&mut view, constants::MONEY_PAYLOAD_BYTES)?,
            ))),
            PgType::Oid => Ok(PgValue::Oid(pgtypes::PgOid::new(
                self.read_be_exact(&mut view, constants::OID_PAYLOAD_BYTES)?,
            ))),
            PgType::Array(_) => Err(Error::UnsupportedType(self.ty.clone())),
        }
    }

    fn decode_bool(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        let byte: u8 = self.read_be_exact(view, constants::BOOL_PAYLOAD_BYTES)?;
        match byte {
            0 => Ok(PgValue::Bool(pgtypes::PgBool::new(false))),
            1 => Ok(PgValue::Bool(pgtypes::PgBool::new(true))),
            _ => Err(self.invalid_payload("invalid bool payload byte")),
        }
    }

    fn decode_jsonb(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        // First byte is the jsonb format version; remainder must be UTF-8 JSON.
        self.ensure_min_remaining(view, constants::JSONB_VERSION_BYTES, "jsonb payload too short")?;
        let version: u8 = self.read_be_exact(view, constants::JSONB_VERSION_BYTES)?;
        let json = self.read_utf8_remaining(view)?;
        Ok(PgValue::Jsonb(pgtypes::PgJsonb::new(version, json)))
    }

    fn decode_date(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        let pg_days: i32 = self.read_be_exact(view, constants::DATE_PAYLOAD_BYTES)?;
        Ok(PgValue::Date(pgtypes::PgDate::new(
            pg_days + PG_DATE_EPOCH_OFFSET_DAYS,
        )))
    }

    fn decode_timestamp(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        let pg_micros: i64 = self.read_be_exact(view, constants::TIMESTAMP_PAYLOAD_BYTES)?;
        Ok(PgValue::Timestamp(pgtypes::PgTimestamp::new(
            pg_micros + PG_TIMESTAMP_EPOCH_OFFSET_US,
        )))
    }

    fn decode_timestamptz(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        let pg_micros: i64 = self.read_be_exact(view, constants::TIMESTAMPTZ_PAYLOAD_BYTES)?;
        Ok(PgValue::Timestamptz(pgtypes::PgTimestamptz::new(
            pg_micros + PG_TIMESTAMP_EPOCH_OFFSET_US,
        )))
    }

    fn decode_timetz(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        if view.remaining() != constants::TIMETZ_PAYLOAD_BYTES {
            return Err(self.invalid_payload("unexpected payload length"));
        }
        Ok(PgValue::Timetz(pgtypes::PgTimetz::new(
            view.read_be()?,
            view.read_be()?,
        )))
    }

    fn decode_interval(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        if view.remaining() != constants::INTERVAL_PAYLOAD_BYTES {
            return Err(self.invalid_payload("unexpected payload length"));
        }
        Ok(PgValue::Interval(pgtypes::PgInterval::new(
            view.read_be()?,
            view.read_be()?,
            view.read_be()?,
        )))
    }

    fn decode_uuid(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        Ok(PgValue::Uuid(pgtypes::PgUuid::new(
            view.read_fixed::<{ constants::UUID_PAYLOAD_BYTES }>()
                .map_err(|_| self.invalid_payload("unexpected payload length"))?,
        )))
    }

    fn validate_varchar(&self, varchar: &pgtypes::PgVarchar) -> Result<()> {
        if let Some(max_len) = varchar.max_len {
            if varchar.value.chars().count() > max_len as usize {
                return Err(self.invalid_payload("varchar value exceeds max length"));
            }
        }
        Ok(())
    }

    fn read_be_exact<T: BigEndianBytes>(
        &self,
        view: &mut BufferView<'_>,
        len: usize,
    ) -> Result<T> {
        view.read_be_exact::<T>(len).map_err(|err| match err {
            Error::UnexpectedEof { .. } | Error::MalformedInput { .. } => {
                self.invalid_payload("unexpected payload length")
            }
            other => other,
        })
    }

    fn read_utf8_remaining(&self, view: &mut BufferView<'_>) -> Result<String> {
        match view.read_utf8_remaining() {
            Ok(value) => Ok(value),
            Err(Error::UnexpectedEof { .. }) => Err(self.invalid_payload("invalid UTF-8")),
            Err(err) => Err(err),
        }
    }

    pub(crate) fn invalid_payload(&self, reason: &'static str) -> Error {
        Error::InvalidPayload {
            column: self.column.to_string(),
            ty: self.ty.clone(),
            reason,
        }
    }
}
