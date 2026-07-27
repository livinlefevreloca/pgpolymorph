//! Decode PostgreSQL binary field payloads into IR [`Value`]s.

use crate::binary::{
    be, constants, FieldCell, FieldReader, PG_DATE_EPOCH_OFFSET_DAYS, PG_TIMESTAMP_EPOCH_OFFSET_US,
};
use crate::codec::array::decode_array;
use crate::error::{Error, Result};
use crate::schema::PgType;
use crate::value::pgtypes::{self, decode_numeric};
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

    match ty {
        PgType::Array(_) => decode_array(ty, cell, column),
        _ => decode_scalar(ty, cell.payload, column),
    }
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
        PgType::Bytea => Ok(Value::Bytea(pgtypes::PgBytea::new(payload.to_vec()))),
        PgType::Char => Ok(Value::Char(pgtypes::PgChar::new(read_i16(
            payload,
            constants::CHAR_PAYLOAD_BYTES,
            column,
            ty,
        )?))),
        PgType::Int2 => Ok(Value::Int2(pgtypes::PgInt2::new(read_i16(
            payload,
            constants::INT2_PAYLOAD_BYTES,
            column,
            ty,
        )?))),
        PgType::Int4 => Ok(Value::Int4(pgtypes::PgInt4::new(read_i32(
            payload,
            constants::INT4_PAYLOAD_BYTES,
            column,
            ty,
        )?))),
        PgType::Int8 => Ok(Value::Int8(pgtypes::PgInt8::new(read_i64(
            payload,
            constants::INT8_PAYLOAD_BYTES,
            column,
            ty,
        )?))),
        PgType::Float4 => Ok(Value::Float4(pgtypes::PgFloat4::new(read_f32(
            payload,
            constants::FLOAT4_PAYLOAD_BYTES,
            column,
            ty,
        )?))),
        PgType::Float8 => Ok(Value::Float8(pgtypes::PgFloat8::new(read_f64(
            payload,
            constants::FLOAT8_PAYLOAD_BYTES,
            column,
            ty,
        )?))),
        PgType::Text => Ok(Value::Text(pgtypes::PgText::new(utf8_text(
            payload, ty, column,
        )?))),
        PgType::Name => Ok(Value::Name(pgtypes::PgName::new(utf8_text(
            payload, ty, column,
        )?))),
        PgType::Json => Ok(Value::Json(pgtypes::PgJson::new(utf8_text(
            payload, ty, column,
        )?))),
        PgType::Jsonb => decode_jsonb(payload, column, ty),
        PgType::Date => decode_date(payload, column, ty),
        PgType::Time => Ok(Value::Time(pgtypes::PgTime::new(read_i64(
            payload,
            constants::TIME_PAYLOAD_BYTES,
            column,
            ty,
        )?))),
        PgType::Timestamp => decode_timestamp(payload, column, ty),
        PgType::Timestamptz => decode_timestamptz(payload, column, ty),
        PgType::Timetz => decode_timetz(payload, column, ty),
        PgType::Interval => decode_interval(payload, column, ty),
        PgType::Numeric => Ok(Value::Numeric(decode_numeric(payload, column, ty)?)),
        PgType::Uuid => decode_uuid(payload, column, ty),
        PgType::Money => Ok(Value::Money(pgtypes::PgMoney::new(read_i64(
            payload,
            constants::MONEY_PAYLOAD_BYTES,
            column,
            ty,
        )?))),
        PgType::Oid => Ok(Value::Oid(pgtypes::PgOid::new(read_u32(
            payload,
            constants::OID_PAYLOAD_BYTES,
            column,
            ty,
        )?))),
        PgType::Array(_) => Err(Error::UnsupportedType(ty.clone())),
    }
}

fn decode_bool(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let payload = exact(payload, constants::BOOL_PAYLOAD_BYTES, column, ty)?;
    Ok(Value::Bool(pgtypes::PgBool::new(payload[0] != 0)))
}

fn decode_jsonb(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    // A jsonb payload is at least one format-version byte followed by JSON bytes.
    if payload.len() < constants::JSONB_VERSION_BYTES {
        return Err(invalid_payload(column, ty, "jsonb payload too short"));
    }
    let mut reader = FieldReader::new(payload);
    let version = reader.read_u8()?;
    let json = utf8_text(reader.read_rest()?, ty, column)?;
    Ok(Value::Jsonb(pgtypes::PgJsonb::new(version, json)))
}

fn decode_date(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let pg_days = read_i32(payload, constants::DATE_PAYLOAD_BYTES, column, ty)?;
    Ok(Value::Date(pgtypes::PgDate::new(
        pg_days + PG_DATE_EPOCH_OFFSET_DAYS,
    )))
}

fn decode_timestamp(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let pg_micros = read_i64(payload, constants::TIMESTAMP_PAYLOAD_BYTES, column, ty)?;
    Ok(Value::Timestamp(pgtypes::PgTimestamp::new(
        pg_micros + PG_TIMESTAMP_EPOCH_OFFSET_US,
    )))
}

fn decode_timestamptz(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let pg_micros = read_i64(payload, constants::TIMESTAMPTZ_PAYLOAD_BYTES, column, ty)?;
    Ok(Value::Timestamptz(pgtypes::PgTimestamptz::new(
        pg_micros + PG_TIMESTAMP_EPOCH_OFFSET_US,
    )))
}

fn decode_timetz(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let mut reader = FieldReader::new(exact(payload, constants::TIMETZ_PAYLOAD_BYTES, column, ty)?);
    Ok(Value::Timetz(pgtypes::PgTimetz::new(
        reader.read_i64()?,
        reader.read_i32()?,
    )))
}

fn decode_interval(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let mut reader =
        FieldReader::new(exact(payload, constants::INTERVAL_PAYLOAD_BYTES, column, ty)?);
    Ok(Value::Interval(pgtypes::PgInterval::new(
        reader.read_i64()?,
        reader.read_i32()?,
        reader.read_i32()?,
    )))
}

fn decode_uuid(payload: &[u8], column: &str, ty: &PgType) -> Result<Value> {
    let bytes = be::read_be_fixed::<{ constants::UUID_PAYLOAD_BYTES }>(exact(
        payload,
        constants::UUID_PAYLOAD_BYTES,
        column,
        ty,
    )?)
    .ok_or_else(|| invalid_payload(column, ty, "invalid uuid payload"))?;
    Ok(Value::Uuid(pgtypes::PgUuid::new(bytes)))
}

fn exact<'a>(payload: &'a [u8], len: usize, column: &str, ty: &PgType) -> Result<&'a [u8]> {
    if payload.len() != len {
        return Err(invalid_payload(column, ty, "unexpected payload length"));
    }
    Ok(payload)
}

fn read_i16(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<i16> {
    Ok(be::read_be_i16(exact(payload, len, column, ty)?)
        .expect("exact length guarantees parse"))
}

fn read_i32(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<i32> {
    Ok(be::read_be_i32(exact(payload, len, column, ty)?)
        .expect("exact length guarantees parse"))
}

fn read_i64(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<i64> {
    Ok(be::read_be_i64(exact(payload, len, column, ty)?)
        .expect("exact length guarantees parse"))
}

fn read_u32(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<u32> {
    Ok(be::read_be_u32(exact(payload, len, column, ty)?)
        .expect("exact length guarantees parse"))
}

fn read_f32(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<f32> {
    Ok(be::read_be_f32(exact(payload, len, column, ty)?)
        .expect("exact length guarantees parse"))
}

fn read_f64(payload: &[u8], len: usize, column: &str, ty: &PgType) -> Result<f64> {
    Ok(be::read_be_f64(exact(payload, len, column, ty)?)
        .expect("exact length guarantees parse"))
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
