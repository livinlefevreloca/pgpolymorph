//! Encode IR [`PgValue`]s into PostgreSQL binary field payloads.

use crate::binary::{constants, PG_DATE_EPOCH_OFFSET_DAYS, PG_TIMESTAMP_EPOCH_OFFSET_US};
use crate::binary::EncodedField;
use crate::codec::array::{encode_array, ArrayElementEncoding};
use crate::error::{Error, Result};
use crate::schema::PgType;
use crate::value::pgtypes::encode_numeric;
use crate::value::PgValue;

pub(crate) struct FieldEncoder<'a> {
    pub(crate) column: &'a str,
    pub(crate) ty: &'a PgType,
    nullable: bool,
}

impl<'a> FieldEncoder<'a> {
    pub fn new(column: &'a str, ty: &'a PgType, nullable: bool) -> Self {
        Self {
            column,
            ty,
            nullable,
        }
    }

    pub fn encode(&self, value: &PgValue) -> Result<EncodedField> {
        if matches!(value, PgValue::Null) {
            if self.nullable {
                return Ok(EncodedField::Null);
            }
            return Err(Error::UnexpectedNull {
                column: self.column.to_string(),
            });
        }

        if !value_matches_type(value, self.ty) {
            return Err(self.type_mismatch(value));
        }

        if matches!(self.ty, PgType::Array(_)) {
            return Ok(EncodedField::NonNull(encode_array(self, value)?));
        }

        Ok(EncodedField::NonNull(self.encode_scalar(value)?))
    }

    pub fn encode_array_element(&self, value: &PgValue) -> Result<ArrayElementEncoding> {
        if matches!(value, PgValue::Null) {
            return Ok(ArrayElementEncoding::Null);
        }
        if !value_matches_type(value, self.ty) {
            return Err(self.type_mismatch(value));
        }
        Ok(ArrayElementEncoding::Payload(self.encode_scalar(value)?))
    }

    pub(crate) fn type_mismatch(&self, value: &PgValue) -> Error {
        Error::TypeMismatch {
            column: self.column.to_string(),
            expected: self.ty.clone(),
            got: crate::value::pg_value_variant_name(value).to_string(),
        }
    }

    pub(crate) fn invalid_payload(&self, reason: &'static str) -> Error {
        Error::InvalidPayload {
            column: self.column.to_string(),
            ty: self.ty.clone(),
            reason,
        }
    }

    fn encode_scalar(&self, value: &PgValue) -> Result<Vec<u8>> {
        match (self.ty, value) {
            (PgType::Bool, PgValue::Bool(v)) => Ok(vec![u8::from(v.value)]),
            (PgType::Bytea, PgValue::Bytea(v)) => Ok(v.bytes.clone()),
            (PgType::Char, PgValue::Char(v)) => Ok(v.value.to_be_bytes().to_vec()),
            (PgType::Int2, PgValue::Int2(v)) => Ok(v.value.to_be_bytes().to_vec()),
            (PgType::Int4, PgValue::Int4(v)) => Ok(v.value.to_be_bytes().to_vec()),
            (PgType::Int8, PgValue::Int8(v)) => Ok(v.value.to_be_bytes().to_vec()),
            (PgType::Float4, PgValue::Float4(v)) => Ok(v.value.to_be_bytes().to_vec()),
            (PgType::Float8, PgValue::Float8(v)) => Ok(v.value.to_be_bytes().to_vec()),
            (PgType::Text, PgValue::Text(v)) => Ok(v.value.as_bytes().to_vec()),
            (PgType::Varchar(schema_max), PgValue::Varchar(v)) => {
                if let Some(max_len) = *schema_max {
                    if v.value.chars().count() > max_len as usize {
                        return Err(self.invalid_payload("varchar value exceeds max length"));
                    }
                }
                Ok(v.value.as_bytes().to_vec())
            }
            (PgType::Name, PgValue::Name(v)) => Ok(v.value.as_bytes().to_vec()),
            (PgType::Json, PgValue::Json(v)) => Ok(v.text.as_bytes().to_vec()),
            (PgType::Jsonb, PgValue::Jsonb(v)) => {
                let mut buf = Vec::with_capacity(constants::JSONB_VERSION_BYTES + v.json.len());
                buf.push(v.version);
                buf.extend_from_slice(v.json.as_bytes());
                Ok(buf)
            }
            (PgType::Date, PgValue::Date(v)) => {
                Ok((v.days - PG_DATE_EPOCH_OFFSET_DAYS).to_be_bytes().to_vec())
            }
            (PgType::Time, PgValue::Time(v)) => Ok(v.micros.to_be_bytes().to_vec()),
            (PgType::Timestamp, PgValue::Timestamp(v)) => Ok((v.micros - PG_TIMESTAMP_EPOCH_OFFSET_US)
                .to_be_bytes()
                .to_vec()),
            (PgType::Timestamptz, PgValue::Timestamptz(v)) => {
                Ok((v.micros - PG_TIMESTAMP_EPOCH_OFFSET_US)
                    .to_be_bytes()
                    .to_vec())
            }
            (PgType::Timetz, PgValue::Timetz(v)) => {
                let mut buf = Vec::with_capacity(constants::TIMETZ_PAYLOAD_BYTES);
                buf.extend_from_slice(&v.micros.to_be_bytes());
                buf.extend_from_slice(&v.tz_offset_secs.to_be_bytes());
                Ok(buf)
            }
            (PgType::Interval, PgValue::Interval(v)) => {
                let mut buf = Vec::with_capacity(constants::INTERVAL_PAYLOAD_BYTES);
                buf.extend_from_slice(&v.micros.to_be_bytes());
                buf.extend_from_slice(&v.days.to_be_bytes());
                buf.extend_from_slice(&v.months.to_be_bytes());
                Ok(buf)
            }
            (PgType::Numeric, PgValue::Numeric(v)) => Ok(encode_numeric(&v)),
            (PgType::Uuid, PgValue::Uuid(v)) => Ok(v.bytes.to_vec()),
            (PgType::Money, PgValue::Money(v)) => Ok(v.amount.to_be_bytes().to_vec()),
            (PgType::Oid, PgValue::Oid(v)) => Ok(v.value.to_be_bytes().to_vec()),
            _ => Err(self.type_mismatch(value)),
        }
    }
}

fn value_matches_type(value: &PgValue, ty: &PgType) -> bool {
    match (ty, value) {
        (PgType::Bool, PgValue::Bool(_)) => true,
        (PgType::Bytea, PgValue::Bytea(_)) => true,
        (PgType::Char, PgValue::Char(_)) => true,
        (PgType::Int2, PgValue::Int2(_)) => true,
        (PgType::Int4, PgValue::Int4(_)) => true,
        (PgType::Int8, PgValue::Int8(_)) => true,
        (PgType::Float4, PgValue::Float4(_)) => true,
        (PgType::Float8, PgValue::Float8(_)) => true,
        (PgType::Text, PgValue::Text(_)) => true,
        (PgType::Varchar(schema_max), PgValue::Varchar(v)) => v.max_len == *schema_max,
        (PgType::Name, PgValue::Name(_)) => true,
        (PgType::Json, PgValue::Json(_)) => true,
        (PgType::Jsonb, PgValue::Jsonb(_)) => true,
        (PgType::Date, PgValue::Date(_)) => true,
        (PgType::Time, PgValue::Time(_)) => true,
        (PgType::Timestamp, PgValue::Timestamp(_)) => true,
        (PgType::Timestamptz, PgValue::Timestamptz(_)) => true,
        (PgType::Timetz, PgValue::Timetz(_)) => true,
        (PgType::Interval, PgValue::Interval(_)) => true,
        (PgType::Numeric, PgValue::Numeric(_)) => true,
        (PgType::Uuid, PgValue::Uuid(_)) => true,
        (PgType::Money, PgValue::Money(_)) => true,
        (PgType::Oid, PgValue::Oid(_)) => true,
        (PgType::Array(_), PgValue::Array(_)) => true,
        _ => false,
    }
}
