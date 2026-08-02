//! Decode PostgreSQL binary field payloads into IR [`PgValue`]s.

use crate::binary::{
    be, constants, BufferView, FieldCell, PG_DATE_EPOCH_OFFSET_DAYS, PG_TIMESTAMP_EPOCH_OFFSET_US,
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

    pub fn decode(&self, cell: &FieldCell<'_>) -> Result<PgValue> {
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

    pub fn decode_array_element(&self, cell: &FieldCell<'_>) -> Result<PgValue> {
        if cell.is_null {
            return Ok(PgValue::Null);
        }
        self.decode_scalar(cell.payload)
    }

    pub(crate) fn ensure_min_payload_len(
        &self,
        payload: &[u8],
        min: usize,
        reason: &'static str,
    ) -> Result<()> {
        if payload.len() < min {
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

    fn decode_scalar(&self, payload: &[u8]) -> Result<PgValue> {
        match self.ty {
            PgType::Bool => self.decode_bool(payload),
            PgType::Bytea => Ok(PgValue::Bytea(pgtypes::PgBytea::new(payload.to_vec()))),
            PgType::Char => Ok(PgValue::Char(pgtypes::PgChar::new(
                self.view(payload, constants::CHAR_PAYLOAD_BYTES)?.read_i16()?,
            ))),
            PgType::Int2 => Ok(PgValue::Int2(pgtypes::PgInt2::new(
                self.view(payload, constants::INT2_PAYLOAD_BYTES)?.read_i16()?,
            ))),
            PgType::Int4 => Ok(PgValue::Int4(pgtypes::PgInt4::new(
                self.view(payload, constants::INT4_PAYLOAD_BYTES)?.read_i32()?,
            ))),
            PgType::Int8 => Ok(PgValue::Int8(pgtypes::PgInt8::new(
                self.view(payload, constants::INT8_PAYLOAD_BYTES)?.read_i64()?,
            ))),
            PgType::Float4 => Ok(PgValue::Float4(pgtypes::PgFloat4::new(
                self.view(payload, constants::FLOAT4_PAYLOAD_BYTES)?.read_f32()?,
            ))),
            PgType::Float8 => Ok(PgValue::Float8(pgtypes::PgFloat8::new(
                self.view(payload, constants::FLOAT8_PAYLOAD_BYTES)?.read_f64()?,
            ))),
            PgType::Text => Ok(PgValue::Text(pgtypes::PgText::new(self.utf8_text(payload)?))),
            PgType::Name => Ok(PgValue::Name(pgtypes::PgName::new(self.utf8_text(payload)?))),
            PgType::Json => Ok(PgValue::Json(pgtypes::PgJson::new(self.utf8_text(payload)?))),
            PgType::Jsonb => self.decode_jsonb(payload),
            PgType::Date => self.decode_date(payload),
            PgType::Time => Ok(PgValue::Time(pgtypes::PgTime::new(
                self.view(payload, constants::TIME_PAYLOAD_BYTES)?.read_i64()?,
            ))),
            PgType::Timestamp => self.decode_timestamp(payload),
            PgType::Timestamptz => self.decode_timestamptz(payload),
            PgType::Timetz => self.decode_timetz(payload),
            PgType::Interval => self.decode_interval(payload),
            PgType::Numeric => Ok(PgValue::Numeric(decode_numeric(
                payload,
                self.column,
                self.ty,
            )?)),
            PgType::Uuid => self.decode_uuid(payload),
            PgType::Money => Ok(PgValue::Money(pgtypes::PgMoney::new(
                self.view(payload, constants::MONEY_PAYLOAD_BYTES)?.read_i64()?,
            ))),
            PgType::Oid => Ok(PgValue::Oid(pgtypes::PgOid::new(
                self.view(payload, constants::OID_PAYLOAD_BYTES)?.read_u32()?,
            ))),
            PgType::Array(_) => Err(Error::UnsupportedType(self.ty.clone())),
        }
    }

    fn decode_bool(&self, payload: &[u8]) -> Result<PgValue> {
        let payload = self.exact(payload, constants::BOOL_PAYLOAD_BYTES)?;
        Ok(PgValue::Bool(pgtypes::PgBool::new(payload[0] != 0)))
    }

    fn decode_jsonb(&self, payload: &[u8]) -> Result<PgValue> {
        // A jsonb payload is at least one format-version byte followed by JSON bytes.
        self.ensure_min_payload_len(payload, constants::JSONB_VERSION_BYTES, "jsonb payload too short")?;
        let mut view = BufferView::new(payload);
        let version = view.read_u8()?;
        let json = self.utf8_text(view.read_rest()?)?;
        Ok(PgValue::Jsonb(pgtypes::PgJsonb::new(version, json)))
    }

    fn decode_date(&self, payload: &[u8]) -> Result<PgValue> {
        let pg_days = self.view(payload, constants::DATE_PAYLOAD_BYTES)?.read_i32()?;
        Ok(PgValue::Date(pgtypes::PgDate::new(
            pg_days + PG_DATE_EPOCH_OFFSET_DAYS,
        )))
    }

    fn decode_timestamp(&self, payload: &[u8]) -> Result<PgValue> {
        let pg_micros = self.view(payload, constants::TIMESTAMP_PAYLOAD_BYTES)?.read_i64()?;
        Ok(PgValue::Timestamp(pgtypes::PgTimestamp::new(
            pg_micros + PG_TIMESTAMP_EPOCH_OFFSET_US,
        )))
    }

    fn decode_timestamptz(&self, payload: &[u8]) -> Result<PgValue> {
        let pg_micros = self.view(payload, constants::TIMESTAMPTZ_PAYLOAD_BYTES)?.read_i64()?;
        Ok(PgValue::Timestamptz(pgtypes::PgTimestamptz::new(
            pg_micros + PG_TIMESTAMP_EPOCH_OFFSET_US,
        )))
    }

    fn decode_timetz(&self, payload: &[u8]) -> Result<PgValue> {
        let mut view = BufferView::new(self.exact(payload, constants::TIMETZ_PAYLOAD_BYTES)?);
        Ok(PgValue::Timetz(pgtypes::PgTimetz::new(
            view.read_i64()?,
            view.read_i32()?,
        )))
    }

    fn decode_interval(&self, payload: &[u8]) -> Result<PgValue> {
        let mut view = BufferView::new(self.exact(payload, constants::INTERVAL_PAYLOAD_BYTES)?);
        Ok(PgValue::Interval(pgtypes::PgInterval::new(
            view.read_i64()?,
            view.read_i32()?,
            view.read_i32()?,
        )))
    }

    fn decode_uuid(&self, payload: &[u8]) -> Result<PgValue> {
        let bytes = be::read_be_fixed::<{ constants::UUID_PAYLOAD_BYTES }>(self.exact(
            payload,
            constants::UUID_PAYLOAD_BYTES,
        )?)
        .ok_or_else(|| self.invalid_payload("invalid uuid payload"))?;
        Ok(PgValue::Uuid(pgtypes::PgUuid::new(bytes)))
    }

    fn exact<'b>(&self, payload: &'b [u8], len: usize) -> Result<&'b [u8]> {
        if payload.len() != len {
            return Err(self.invalid_payload("unexpected payload length"));
        }
        Ok(payload)
    }

    fn view<'b>(&self, payload: &'b [u8], len: usize) -> Result<BufferView<'b>> {
        Ok(BufferView::new(self.exact(payload, len)?))
    }

    fn utf8_text(&self, payload: &[u8]) -> Result<String> {
        std::str::from_utf8(payload)
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
