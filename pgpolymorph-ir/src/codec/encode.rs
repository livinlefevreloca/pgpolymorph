//! Encode IR [`Value`]s into PostgreSQL binary field payloads.

use crate::binary::wire;
use crate::binary::EncodedField;
use crate::error::{Error, Result};
use crate::schema::PgType;
use crate::codec::array::{encode_array, ArrayElementEncoding};
use crate::value::types::encode_numeric;
use crate::value::Value;

pub(crate) fn encode_field(
    value: &Value,
    ty: &PgType,
    column: &str,
    nullable: bool,
) -> Result<EncodedField> {
    if matches!(value, Value::Null) {
        if nullable {
            return Ok(EncodedField::Null);
        }
        return Err(Error::UnexpectedNull {
            column: column.to_string(),
        });
    }

    if !value_matches_type(value, ty) {
        return Err(Error::TypeMismatch {
            column: column.to_string(),
            expected: ty.clone(),
            got: crate::value::value_variant_name(value).to_string(),
        });
    }

    if matches!(ty, PgType::Array(_)) {
        return Ok(EncodedField::NonNull(encode_array(value, ty)?));
    }

    Ok(EncodedField::NonNull(encode_scalar(value, ty)?))
}

pub(crate) fn encode_array_element(value: &Value, ty: &PgType) -> Result<ArrayElementEncoding> {
    if matches!(value, Value::Null) {
        return Ok(ArrayElementEncoding::Null);
    }
    if !value_matches_type(value, ty) {
        return Err(Error::TypeMismatch {
            column: String::new(),
            expected: ty.clone(),
            got: crate::value::value_variant_name(value).to_string(),
        });
    }
    Ok(ArrayElementEncoding::Payload(encode_scalar(value, ty)?))
}

fn encode_scalar(value: &Value, ty: &PgType) -> Result<Vec<u8>> {
    match (ty, value) {
        (PgType::Bool, Value::Bool(v)) => Ok(vec![u8::from(v.value)]),
        (PgType::Bytea, Value::Bytea(v)) => Ok(v.bytes.clone()),
        (PgType::Char, Value::Char(v)) => Ok(v.value.to_be_bytes().to_vec()),
        (PgType::Int2, Value::Int2(v)) => Ok(v.value.to_be_bytes().to_vec()),
        (PgType::Int4, Value::Int4(v)) => Ok(v.value.to_be_bytes().to_vec()),
        (PgType::Int8, Value::Int8(v)) => Ok(v.value.to_be_bytes().to_vec()),
        (PgType::Float4, Value::Float4(v)) => Ok(v.value.to_be_bytes().to_vec()),
        (PgType::Float8, Value::Float8(v)) => Ok(v.value.to_be_bytes().to_vec()),
        (PgType::Text, Value::Text(v)) => Ok(v.value.as_bytes().to_vec()),
        (PgType::Name, Value::Name(v)) => Ok(v.value.as_bytes().to_vec()),
        (PgType::Json, Value::Json(v)) => Ok(v.text.as_bytes().to_vec()),
        (PgType::Jsonb, Value::Jsonb(v)) => {
            let mut buf = Vec::with_capacity(wire::JSONB_VERSION + v.json.len());
            buf.push(v.version);
            buf.extend_from_slice(v.json.as_bytes());
            Ok(buf)
        }
        (PgType::Date, Value::Date(v)) => {
            use crate::binary::PG_DATE_EPOCH_OFFSET_DAYS;
            Ok((v.days - PG_DATE_EPOCH_OFFSET_DAYS).to_be_bytes().to_vec())
        }
        (PgType::Time, Value::Time(v)) => Ok(v.micros.to_be_bytes().to_vec()),
        (PgType::Timestamp, Value::Timestamp(v)) => {
            use crate::binary::PG_TIMESTAMP_EPOCH_OFFSET_US;
            Ok((v.micros - PG_TIMESTAMP_EPOCH_OFFSET_US)
                .to_be_bytes()
                .to_vec())
        }
        (PgType::Timestamptz, Value::Timestamptz(v)) => {
            use crate::binary::PG_TIMESTAMP_EPOCH_OFFSET_US;
            Ok((v.micros - PG_TIMESTAMP_EPOCH_OFFSET_US)
                .to_be_bytes()
                .to_vec())
        }
        (PgType::Timetz, Value::Timetz(v)) => {
            let mut buf = Vec::with_capacity(wire::TIMETZ);
            buf.extend_from_slice(&v.micros.to_be_bytes());
            buf.extend_from_slice(&v.tz_offset_secs.to_be_bytes());
            Ok(buf)
        }
        (PgType::Interval, Value::Interval(v)) => {
            let mut buf = Vec::with_capacity(wire::INTERVAL);
            buf.extend_from_slice(&v.micros.to_be_bytes());
            buf.extend_from_slice(&v.days.to_be_bytes());
            buf.extend_from_slice(&v.months.to_be_bytes());
            Ok(buf)
        }
        (PgType::Numeric, Value::Numeric(v)) => Ok(encode_numeric(v)),
        (PgType::Uuid, Value::Uuid(v)) => Ok(v.bytes.to_vec()),
        (PgType::Money, Value::Money(v)) => Ok(v.amount.to_be_bytes().to_vec()),
        (PgType::Oid, Value::Oid(v)) => Ok(v.value.to_be_bytes().to_vec()),
        _ => Err(Error::TypeMismatch {
            column: String::new(),
            expected: ty.clone(),
            got: crate::value::value_variant_name(value).to_string(),
        }),
    }
}

fn value_matches_type(value: &Value, ty: &PgType) -> bool {
    matches!(
        (ty, value),
        (PgType::Bool, Value::Bool(_))
            | (PgType::Bytea, Value::Bytea(_))
            | (PgType::Char, Value::Char(_))
            | (PgType::Int2, Value::Int2(_))
            | (PgType::Int4, Value::Int4(_))
            | (PgType::Int8, Value::Int8(_))
            | (PgType::Float4, Value::Float4(_))
            | (PgType::Float8, Value::Float8(_))
            | (PgType::Text, Value::Text(_))
            | (PgType::Name, Value::Name(_))
            | (PgType::Json, Value::Json(_))
            | (PgType::Jsonb, Value::Jsonb(_))
            | (PgType::Date, Value::Date(_))
            | (PgType::Time, Value::Time(_))
            | (PgType::Timestamp, Value::Timestamp(_))
            | (PgType::Timestamptz, Value::Timestamptz(_))
            | (PgType::Timetz, Value::Timetz(_))
            | (PgType::Interval, Value::Interval(_))
            | (PgType::Numeric, Value::Numeric(_))
            | (PgType::Uuid, Value::Uuid(_))
            | (PgType::Money, Value::Money(_))
            | (PgType::Oid, Value::Oid(_))
            | (PgType::Array(_), Value::Array(_))
    )
}
