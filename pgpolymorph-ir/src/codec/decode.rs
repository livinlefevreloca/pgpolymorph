//! Decode PostgreSQL binary field payloads into IR [`Value`]s.

use crate::binary::{be, FieldCell, FieldReader, PG_DATE_EPOCH_OFFSET_DAYS, PG_TIMESTAMP_EPOCH_OFFSET_US, wire};
use crate::error::{Error, Result};
use crate::schema::PgType;
use crate::codec::array::decode_array;
use crate::value::types::{self, decode_numeric};
use crate::value::Value;

pub(crate) fn decode_field(
    ty: &PgType,
    cell: &FieldCell<'_>,
    column: &str,
    nullable: bool,
) -> Result<Value> {
    if cell.is_null {
        if nullable {
            return Ok(Value::Null);
        }
        return Err(Error::UnexpectedNull {
            column: column.to_string(),
        });
    }

    if matches!(ty, PgType::Array(_)) {
        return decode_array(ty, cell, column);
    }

    decode_scalar(ty, cell.payload, column)
}

pub(crate) fn decode_array_element(ty: &PgType, cell: &FieldCell<'_>) -> Result<Value> {
    if cell.is_null {
        return Ok(Value::Null);
    }
    decode_scalar(ty, cell.payload, "")
}

fn decode_scalar(ty: &PgType, payload: &[u8], column: &str) -> Result<Value> {
    match ty {
        PgType::Bool => decode_bool(payload, column, ty),
        PgType::Bytea => Ok(Value::Bytea(types::PgBytea::new(payload.to_vec()))),
        PgType::Char => Ok(Value::Char(types::PgChar::new(read_i16(
            payload, wire::CHAR, column, ty,
        )?))),
        PgType::Int2 => Ok(Value::Int2(types::PgInt2::new(read_i16(
            payload, wire::INT2, column, ty,
        )?))),
        PgType::Int4 => Ok(Value::Int4(types::PgInt4::new(read_i32(
            payload, wire::INT4, column, ty,
        )?))),
        PgType::Int8 => Ok(Value::Int8(types::PgInt8::new(read_i64(
            payload, wire::INT8, column, ty,
        )?))),
        PgType::Float4 => Ok(Value::Float4(types::PgFloat4::new(read_f32(
            payload, wire::FLOAT4, column, ty,
        )?))),
        PgType::Float8 => Ok(Value::Float8(types::PgFloat8::new(read_f64(
            payload, wire::FLOAT8, column, ty,
        )?))),
        PgType::Text => Ok(Value::Text(types::PgText::new(utf8_text(
            payload, ty, column,
        )?))),
        PgType::Name => Ok(Value::Name(types::PgName::new(utf8_text(
            payload, ty, column,
        )?))),
        PgType::Json => Ok(Value::Json(types::PgJson::new(utf8_text(
            payload, ty, column,
        )?))),
        PgType::Jsonb => decode_jsonb(payload, column, ty),
        PgType::Date => decode_date(payload, column, ty),
        PgType::Time => Ok(Value::Time(types::PgTime::new(read_i64(
            payload, wire::TIME, column, ty,
        )?))),
        PgType::Timestamp => decode_timestamp(payload, column, ty),
        PgType::Timestamptz => decode_timestamptz(payload, column, ty),
        PgType::Timetz => decode_timetz(payload, column, ty),
        PgType::Interval => decode_interval(payload, column, ty),
        PgType::Numeric => Ok(Value::Numeric(decode_numeric(payload, column, ty)?)),
        PgType::Uuid => decode_uuid(payload, column, ty),
        PgType::Money => Ok(Value::Money(types::PgMoney::new(read_i64(
            payload, wire::MONEY, column, ty,
        )?))),
        PgType::Oid => Ok(Value::Oid(types::PgOid::new(read_u32(
            payload, wire::OID, column, ty,
        )?))),
        PgType::Array(_) => Err(Error::UnsupportedType(ty.clone())),
    }
}

fn decode_bool(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let payload = exact(payload, wire::BOOL, column, ty)?;
    Ok(Value::Bool(types::PgBool::new(payload[0] != 0)))
}

fn decode_jsonb(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    if payload.len() < wire::JSONB_VERSION {
        return Err(invalid_payload(column, ty, "jsonb payload too short"));
    }
    let mut reader = FieldReader::new(payload);
    let version = reader.read_u8()?;
    let json = utf8_text(reader.read_rest()?, ty, column)?;
    Ok(Value::Jsonb(types::PgJsonb::new(version, json)))
}

fn decode_date(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let wire = read_i32(payload, wire::DATE, column, ty)?;
    Ok(Value::Date(types::PgDate::new(
        wire + PG_DATE_EPOCH_OFFSET_DAYS,
    )))
}

fn decode_timestamp(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let wire = read_i64(payload, wire::TIMESTAMP, column, ty)?;
    Ok(Value::Timestamp(types::PgTimestamp::new(
        wire + PG_TIMESTAMP_EPOCH_OFFSET_US,
    )))
}

fn decode_timestamptz(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let wire = read_i64(payload, wire::TIMESTAMPTZ, column, ty)?;
    Ok(Value::Timestamptz(types::PgTimestamptz::new(
        wire + PG_TIMESTAMP_EPOCH_OFFSET_US,
    )))
}

fn decode_timetz(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let mut reader = FieldReader::new(exact(payload, wire::TIMETZ, column, ty)?);
    Ok(Value::Timetz(types::PgTimetz::new(
        reader.read_i64()?,
        reader.read_i32()?,
    )))
}

fn decode_interval(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let mut reader = FieldReader::new(exact(payload, wire::INTERVAL, column, ty)?);
    Ok(Value::Interval(types::PgInterval::new(
        reader.read_i64()?,
        reader.read_i32()?,
        reader.read_i32()?,
    )))
}

fn decode_uuid(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let bytes = be::fixed::<{ wire::UUID }>(exact(payload, wire::UUID, column, ty)?)
        .ok_or_else(|| invalid_payload(column, ty, "invalid uuid payload"))?;
    Ok(Value::Uuid(types::PgUuid::new(bytes)))
}

fn exact<'a>(payload: &'a [u8], len: usize, column: &str, ty: &PgType) -> Result<&'a [u8]> {
    if payload.len() != len {
        return Err(invalid_payload(column, ty, "unexpected payload length"));
    }
    Ok(payload)
}

fn read_i16(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<i16> {
    be::i16(exact(payload, len, column, ty)?)
        .ok_or_else(|| invalid_payload(column, ty, "invalid i16 payload"))
}

fn read_i32(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<i32> {
    be::i32(exact(payload, len, column, ty)?)
        .ok_or_else(|| invalid_payload(column, ty, "invalid i32 payload"))
}

fn read_i64(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<i64> {
    be::i64(exact(payload, len, column, ty)?)
        .ok_or_else(|| invalid_payload(column, ty, "invalid i64 payload"))
}

fn read_u32(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<u32> {
    be::u32(exact(payload, len, column, ty)?)
        .ok_or_else(|| invalid_payload(column, ty, "invalid u32 payload"))
}

fn read_f32(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<f32> {
    be::f32(exact(payload, len, column, ty)?)
        .ok_or_else(|| invalid_payload(column, ty, "invalid f32 payload"))
}

fn read_f64(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<f64> {
    be::f64(exact(payload, len, column, ty)?)
        .ok_or_else(|| invalid_payload(column, ty, "invalid f64 payload"))
}

fn utf8_text(payload: &[u8], ty: &PgType, column: &str) -> Result<String> {
    std::str::from_utf8(payload)
        .map(|s| s.to_string())
        .map_err(|_| invalid_payload(column, ty, "invalid UTF-8"))
}

fn invalid_payload(column: &str, ty: &PgType, reason: &'static str) -> Error {
    Error::InvalidPayload {
        column: column.to_string(),
        ty: ty.clone(),
        reason,
    }
}
