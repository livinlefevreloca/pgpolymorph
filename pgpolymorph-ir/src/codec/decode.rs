//! Decode PostgreSQL binary field payloads into IR [`PgValue`]s.

use crate::binary::{
    constants, BufferView, FieldCell, PG_DATE_EPOCH_OFFSET_DAYS, PG_TIMESTAMP_EPOCH_OFFSET_US,
};
use crate::codec::array::decode_array;
use crate::error::{Error, Result};
use crate::schema::PgType;
use crate::value::pgtypes::{self, decode_numeric};
use crate::value::PgValue;

pub(crate) struct FieldDecoder<'a> {
    column: &'a str,
    ty: &'a PgType,
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
            if self.nullable {
                return Ok(PgValue::Null);
            }
            return Err(Error::UnexpectedNull {
                column: self.column.to_string(),
            });
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

    pub(crate) fn column(&self) -> &str {
        self.column
    }

    pub(crate) fn ty(&self) -> &PgType {
        self.ty
    }

    fn decode_scalar(&self, mut view: BufferView<'_>) -> Result<PgValue> {
        match self.ty {
            PgType::Bool => self.decode_bool(&mut view),
            PgType::Bytea => Ok(PgValue::Bytea(pgtypes::PgBytea::new(
                view.read_rest()?.to_vec(),
            ))),
            PgType::Char => Ok(PgValue::Char(pgtypes::PgChar::new(
                self.read_i16(&mut view, constants::CHAR_PAYLOAD_BYTES)?,
            ))),
            PgType::Int2 => Ok(PgValue::Int2(pgtypes::PgInt2::new(
                self.read_i16(&mut view, constants::INT2_PAYLOAD_BYTES)?,
            ))),
            PgType::Int4 => Ok(PgValue::Int4(pgtypes::PgInt4::new(
                self.read_i32(&mut view, constants::INT4_PAYLOAD_BYTES)?,
            ))),
            PgType::Int8 => Ok(PgValue::Int8(pgtypes::PgInt8::new(
                self.read_i64(&mut view, constants::INT8_PAYLOAD_BYTES)?,
            ))),
            PgType::Float4 => Ok(PgValue::Float4(pgtypes::PgFloat4::new(
                self.read_f32(&mut view, constants::FLOAT4_PAYLOAD_BYTES)?,
            ))),
            PgType::Float8 => Ok(PgValue::Float8(pgtypes::PgFloat8::new(
                self.read_f64(&mut view, constants::FLOAT8_PAYLOAD_BYTES)?,
            ))),
            PgType::Text => Ok(PgValue::Text(pgtypes::PgText::new(self.utf8_view(&mut view)?))),
            PgType::Name => Ok(PgValue::Name(pgtypes::PgName::new(self.utf8_view(&mut view)?))),
            PgType::Json => Ok(PgValue::Json(pgtypes::PgJson::new(self.utf8_view(&mut view)?))),
            PgType::Jsonb => self.decode_jsonb(&mut view),
            PgType::Date => self.decode_date(&mut view),
            PgType::Time => Ok(PgValue::Time(pgtypes::PgTime::new(
                self.read_i64(&mut view, constants::TIME_PAYLOAD_BYTES)?,
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
                self.read_i64(&mut view, constants::MONEY_PAYLOAD_BYTES)?,
            ))),
            PgType::Oid => Ok(PgValue::Oid(pgtypes::PgOid::new(
                self.read_u32(&mut view, constants::OID_PAYLOAD_BYTES)?,
            ))),
            PgType::Array(_) => Err(Error::UnsupportedType(self.ty.clone())),
        }
    }

    fn decode_bool(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        Ok(PgValue::Bool(pgtypes::PgBool::new(
            self.read_u8(view, constants::BOOL_PAYLOAD_BYTES)? != 0,
        )))
    }

    fn decode_jsonb(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        // A jsonb payload is at least one format-version byte followed by JSON bytes.
        self.ensure_min_remaining(view, constants::JSONB_VERSION_BYTES, "jsonb payload too short")?;
        let version = view.read_u8()?;
        let json = self.utf8_bytes(view.read_rest()?)?;
        Ok(PgValue::Jsonb(pgtypes::PgJsonb::new(version, json)))
    }

    fn decode_date(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        let pg_days = self.read_i32(view, constants::DATE_PAYLOAD_BYTES)?;
        Ok(PgValue::Date(pgtypes::PgDate::new(
            pg_days + PG_DATE_EPOCH_OFFSET_DAYS,
        )))
    }

    fn decode_timestamp(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        let pg_micros = self.read_i64(view, constants::TIMESTAMP_PAYLOAD_BYTES)?;
        Ok(PgValue::Timestamp(pgtypes::PgTimestamp::new(
            pg_micros + PG_TIMESTAMP_EPOCH_OFFSET_US,
        )))
    }

    fn decode_timestamptz(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        let pg_micros = self.read_i64(view, constants::TIMESTAMPTZ_PAYLOAD_BYTES)?;
        Ok(PgValue::Timestamptz(pgtypes::PgTimestamptz::new(
            pg_micros + PG_TIMESTAMP_EPOCH_OFFSET_US,
        )))
    }

    fn decode_timetz(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        self.ensure_exact_remaining(view, constants::TIMETZ_PAYLOAD_BYTES)?;
        Ok(PgValue::Timetz(pgtypes::PgTimetz::new(
            view.read_i64()?,
            view.read_i32()?,
        )))
    }

    fn decode_interval(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        self.ensure_exact_remaining(view, constants::INTERVAL_PAYLOAD_BYTES)?;
        Ok(PgValue::Interval(pgtypes::PgInterval::new(
            view.read_i64()?,
            view.read_i32()?,
            view.read_i32()?,
        )))
    }

    fn decode_uuid(&self, view: &mut BufferView<'_>) -> Result<PgValue> {
        self.ensure_exact_remaining(view, constants::UUID_PAYLOAD_BYTES)?;
        Ok(PgValue::Uuid(pgtypes::PgUuid::new(
            view.read_fixed::<{ constants::UUID_PAYLOAD_BYTES }>()?,
        )))
    }

    fn ensure_exact_remaining(&self, view: &BufferView<'_>, len: usize) -> Result<()> {
        if view.remaining() != len {
            return Err(self.invalid_payload("unexpected payload length"));
        }
        Ok(())
    }

    fn read_u8(&self, view: &mut BufferView<'_>, len: usize) -> Result<u8> {
        self.ensure_exact_remaining(view, len)?;
        view.read_u8()
    }

    fn read_i16(&self, view: &mut BufferView<'_>, len: usize) -> Result<i16> {
        self.ensure_exact_remaining(view, len)?;
        view.read_i16()
    }

    fn read_i32(&self, view: &mut BufferView<'_>, len: usize) -> Result<i32> {
        self.ensure_exact_remaining(view, len)?;
        view.read_i32()
    }

    fn read_i64(&self, view: &mut BufferView<'_>, len: usize) -> Result<i64> {
        self.ensure_exact_remaining(view, len)?;
        view.read_i64()
    }

    fn read_u32(&self, view: &mut BufferView<'_>, len: usize) -> Result<u32> {
        self.ensure_exact_remaining(view, len)?;
        view.read_u32()
    }

    fn read_f32(&self, view: &mut BufferView<'_>, len: usize) -> Result<f32> {
        self.ensure_exact_remaining(view, len)?;
        view.read_f32()
    }

    fn read_f64(&self, view: &mut BufferView<'_>, len: usize) -> Result<f64> {
        self.ensure_exact_remaining(view, len)?;
        view.read_f64()
    }

    fn utf8_view(&self, view: &mut BufferView<'_>) -> Result<String> {
        self.utf8_bytes(view.read_rest()?)
    }

    fn utf8_bytes(&self, bytes: &[u8]) -> Result<String> {
        std::str::from_utf8(bytes)
            .map(|s| s.to_string())
            .map_err(|_| self.invalid_payload("invalid UTF-8"))
    }

    pub(crate) fn invalid_payload(&self, reason: &'static str) -> Error {
        Error::InvalidPayload {
            column: self.column.to_string(),
            ty: self.ty.clone(),
            reason,
        }
    }
}
