//! PostgreSQL array binary format decode/encode.

use crate::binary::{be, FieldCell, FieldReader, wire};
use crate::error::{Error, Result};
use crate::schema::PgType;
use crate::codec::decode::decode_array_element;
use crate::codec::encode::encode_array_element;
use crate::codec::oid::oid_to_pg_type;
use crate::value::types;
use crate::value::Value;

pub(crate) fn decode_array(
    array_ty: &PgType,
    cell: &FieldCell<'_>,
    column: &str,
) -> Result<Value> {
    let element_ty = match array_ty {
        PgType::Array(inner) => inner.as_ref(),
        _ => {
            return Err(Error::InvalidPayload {
                column: column.to_string(),
                ty: array_ty.clone(),
                reason: "not an array type",
            });
        }
    };

    if cell.is_null {
        return Ok(Value::Null);
    }

    let payload = cell.payload;
    let body = strip_optional_total_len(payload)?;

    let mut reader = FieldReader::new(body);
    let ndim = reader.read_i32()?;
    if ndim < wire::ARRAY_MIN_NDIM {
        return Err(Error::InvalidPayload {
            column: column.to_string(),
            ty: array_ty.clone(),
            reason: "array ndim must be at least 1",
        });
    }

    let has_nulls = reader.read_i32()?;
    let element_oid = reader.read_i32()? as u32;
    let wire_element_ty = oid_to_pg_type(element_oid).ok_or_else(|| {
        Error::UnsupportedType(PgType::Array(Box::new(element_ty.clone())))
    })?;

    if !array_element_types_compatible(element_ty, &wire_element_ty) {
        return Err(Error::InvalidPayload {
            column: column.to_string(),
            ty: array_ty.clone(),
            reason: "array element OID does not match schema",
        });
    }

    let mut dimensions = Vec::with_capacity(ndim as usize);
    let mut total_elements = 1i64;
    for _ in 0..ndim {
        let length = reader.read_i32()?;
        let lower_bound = reader.read_i32()?;
        if length < 0 {
            return Err(Error::InvalidPayload {
                column: column.to_string(),
                ty: array_ty.clone(),
                reason: "negative array dimension length",
            });
        }
        total_elements = total_elements
            .checked_mul(length as i64)
            .ok_or_else(|| Error::InvalidPayload {
                column: column.to_string(),
                ty: array_ty.clone(),
                reason: "array element count overflow",
            })?;
        dimensions.push(types::ArrayDimension {
            length,
            lower_bound,
        });
    }

    let total_elements = usize::try_from(total_elements).map_err(|_| Error::InvalidPayload {
        column: column.to_string(),
        ty: array_ty.clone(),
        reason: "array element count overflow",
    })?;

    if has_nulls != wire::ARRAY_HAS_NULLS_FALSE && has_nulls != wire::ARRAY_HAS_NULLS_TRUE {
        return Err(Error::InvalidPayload {
            column: column.to_string(),
            ty: array_ty.clone(),
            reason: "invalid array has_nulls flag",
        });
    }

    let mut elements = Vec::with_capacity(total_elements);
    for _ in 0..total_elements {
        elements.push(decode_array_element(element_ty, &reader.read_field()?)?);
    }

    Ok(Value::Array(types::PgArray::new(
        element_ty.clone(),
        dimensions,
        elements,
    )))
}

fn strip_optional_total_len(payload: &[u8]) -> Result<&[u8]> {
    if payload.len() < wire::ARRAY_TOTAL_LEN {
        return Err(Error::InvalidPayload {
            column: String::new(),
            ty: PgType::Array(Box::new(PgType::Int4)),
            reason: "array payload too short",
        });
    }
    let total_len = be::i32(&payload[..wire::ARRAY_TOTAL_LEN]).unwrap_or(0) as usize;
    if total_len == payload.len().saturating_sub(wire::ARRAY_TOTAL_LEN) {
        Ok(&payload[wire::ARRAY_TOTAL_LEN..])
    } else {
        Ok(payload)
    }
}

fn array_element_types_compatible(schema_ty: &PgType, wire_ty: &PgType) -> bool {
    matches!(
        (schema_ty, wire_ty),
        (PgType::Bool, PgType::Bool)
            | (PgType::Bytea, PgType::Bytea)
            | (PgType::Char, PgType::Char)
            | (PgType::Int2, PgType::Int2)
            | (PgType::Int4, PgType::Int4)
            | (PgType::Int8, PgType::Int8)
            | (PgType::Float4, PgType::Float4)
            | (PgType::Float8, PgType::Float8)
            | (PgType::Text, PgType::Text)
            | (PgType::Json, PgType::Json)
            | (PgType::Jsonb, PgType::Jsonb)
            | (PgType::Date, PgType::Date)
            | (PgType::Time, PgType::Time)
            | (PgType::Timestamp, PgType::Timestamp)
            | (PgType::Timestamptz, PgType::Timestamptz)
            | (PgType::Timetz, PgType::Timetz)
            | (PgType::Interval, PgType::Interval)
            | (PgType::Numeric, PgType::Numeric)
            | (PgType::Uuid, PgType::Uuid)
            | (PgType::Money, PgType::Money)
            | (PgType::Oid, PgType::Oid)
            | (PgType::Name, PgType::Name)
    )
}

pub(crate) fn encode_array(value: &Value, array_ty: &PgType) -> Result<Vec<u8>> {
    let element_ty = match array_ty {
        PgType::Array(inner) => inner.as_ref(),
        _ => {
            return Err(Error::TypeMismatch {
                column: String::new(),
                expected: array_ty.clone(),
                got: value_variant(value),
            });
        }
    };

    let Value::Array(array) = value else {
        return Err(Error::TypeMismatch {
            column: String::new(),
            expected: array_ty.clone(),
            got: value_variant(value),
        });
    };

    if &array.element_type != element_ty {
        return Err(Error::TypeMismatch {
            column: String::new(),
            expected: array_ty.clone(),
            got: value_variant(value),
        });
    }

    let ndim = array.dimensions.len();
    if ndim == 0 {
        return Err(Error::InvalidPayload {
            column: String::new(),
            ty: array_ty.clone(),
            reason: "array must have at least one dimension",
        });
    }

    let mut expected_count = 1i64;
    for dim in &array.dimensions {
        expected_count = expected_count
            .checked_mul(dim.length as i64)
            .ok_or_else(|| Error::InvalidPayload {
                column: String::new(),
                ty: array_ty.clone(),
                reason: "array element count overflow",
            })?;
    }
    if array.elements.len() as i64 != expected_count {
        return Err(Error::InvalidPayload {
            column: String::new(),
            ty: array_ty.clone(),
            reason: "array element count does not match dimensions",
        });
    }

    let has_nulls = if array.elements.iter().any(Value::is_null) {
        wire::ARRAY_HAS_NULLS_TRUE
    } else {
        wire::ARRAY_HAS_NULLS_FALSE
    };
    let element_oid = element_ty
        .oid()
        .ok_or_else(|| Error::UnsupportedType(array_ty.clone()))?;

    let mut body = Vec::new();
    write_i32(&mut body, ndim as i32);
    write_i32(&mut body, has_nulls);
    write_i32(&mut body, element_oid as i32);
    for dim in &array.dimensions {
        write_i32(&mut body, dim.length);
        write_i32(&mut body, dim.lower_bound);
    }

    for element in &array.elements {
        match encode_array_element(element, element_ty)? {
            ArrayElementEncoding::Null => write_i32(&mut body, wire::COPY_FIELD_NULL),
            ArrayElementEncoding::Payload(payload) => {
                write_i32(&mut body, payload.len() as i32);
                body.extend_from_slice(&payload);
            }
        }
    }

    if ndim >= wire::ARRAY_MULTI_DIM_NDIM {
        let mut buf = Vec::with_capacity(wire::ARRAY_TOTAL_LEN + body.len());
        write_i32(&mut buf, body.len() as i32);
        buf.extend_from_slice(&body);
        Ok(buf)
    } else {
        Ok(body)
    }
}

pub(crate) enum ArrayElementEncoding {
    Null,
    Payload(Vec<u8>),
}

fn write_i32(buf: &mut Vec<u8>, value: i32) {
    buf.extend_from_slice(&value.to_be_bytes());
}

fn value_variant(value: &Value) -> String {
    crate::value::value_variant_name(value).to_string()
}
