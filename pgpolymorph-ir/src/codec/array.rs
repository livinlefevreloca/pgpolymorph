//! PostgreSQL array binary format decode/encode.

use crate::binary::{be, constants, FieldCell, FieldReader};
use crate::codec::decode::FieldDecoder;
use crate::codec::encode::FieldEncoder;
use crate::error::{Error, Result};
use crate::schema::PgType;
use crate::value::pgtypes;
use crate::value::PgValue;

pub(crate) fn decode_array(decoder: &FieldDecoder<'_>, cell: &FieldCell<'_>) -> Result<PgValue> {
    let array_ty = decoder.ty();
    let element_ty = match array_ty {
        PgType::Array(inner) => inner.as_ref(),
        _ => {
            return Err(decoder.invalid_payload("not an array type"));
        }
    };

    if cell.is_null {
        return Ok(PgValue::Null);
    }

    let body = strip_optional_total_len(decoder, cell.payload)?;

    let mut reader = FieldReader::new(body);
    let ndim = reader.read_i32()?;
    if ndim < constants::ARRAY_MIN_NDIM {
        return Err(decoder.invalid_payload("array ndim must be at least 1"));
    }

    let has_nulls = reader.read_i32()?;
    let element_oid = reader.read_i32()? as u32;
    let payload_element_ty = PgType::from_oid(element_oid).ok_or_else(|| {
        Error::UnsupportedType(PgType::Array(Box::new(element_ty.clone())))
    })?;

    if element_ty != &payload_element_ty {
        return Err(Error::ArrayElementOidMismatch {
            column: decoder.column().to_string(),
            expected: element_ty.clone(),
            element_oid,
        });
    }

    let (dimensions, total_elements) = parse_dimensions(decoder, &mut reader, ndim)?;

    if has_nulls != constants::ARRAY_HAS_NULLS_FALSE
        && has_nulls != constants::ARRAY_HAS_NULLS_TRUE
    {
        return Err(Error::InvalidArrayHasNulls {
            column: decoder.column().to_string(),
            got: has_nulls,
        });
    }

    let element_decoder = FieldDecoder::new(decoder.column(), element_ty, true);
    let mut elements = Vec::with_capacity(total_elements);
    for _ in 0..total_elements {
        elements.push(element_decoder.decode_array_element(&reader.read_field()?)?);
    }

    Ok(PgValue::Array(pgtypes::PgArray::new(
        element_ty.clone(),
        dimensions,
        elements,
    )))
}

fn parse_dimensions(
    decoder: &FieldDecoder<'_>,
    reader: &mut FieldReader<'_>,
    ndim: i32,
) -> Result<(Vec<pgtypes::ArrayDimension>, usize)> {
    let mut dimensions = Vec::with_capacity(ndim as usize);
    let mut total_elements = 1i64;
    for _ in 0..ndim {
        let length = reader.read_i32()?;
        let lower_bound = reader.read_i32()?;
        if length < 0 {
            return Err(decoder.invalid_payload("negative array dimension length"));
        }
        total_elements = total_elements
            .checked_mul(length as i64)
            .ok_or_else(|| decoder.invalid_payload("array element count overflow"))?;
        dimensions.push(pgtypes::ArrayDimension {
            length,
            lower_bound,
        });
    }

    let total_elements = usize::try_from(total_elements)
        .map_err(|_| decoder.invalid_payload("array element count overflow"))?;

    Ok((dimensions, total_elements))
}

fn strip_optional_total_len<'a>(
    decoder: &FieldDecoder<'_>,
    payload: &'a [u8],
) -> Result<&'a [u8]> {
    decoder.ensure_min_payload_len(
        payload,
        constants::ARRAY_TOTAL_LEN_BYTES,
        "array payload too short",
    )?;
    let total_len =
        be::read_be_i32(&payload[..constants::ARRAY_TOTAL_LEN_BYTES]).unwrap_or(0) as usize;
    if total_len == payload.len().saturating_sub(constants::ARRAY_TOTAL_LEN_BYTES) {
        Ok(&payload[constants::ARRAY_TOTAL_LEN_BYTES..])
    } else {
        Ok(payload)
    }
}

pub(crate) fn encode_array(encoder: &FieldEncoder<'_>, value: &PgValue) -> Result<Vec<u8>> {
    let array_ty = encoder.ty();
    let element_ty = match array_ty {
        PgType::Array(inner) => inner.as_ref(),
        _ => return Err(encoder.type_mismatch(value)),
    };

    // Schema column is an array type, so the IR value must be `PgValue::Array`.
    let array = match value {
        PgValue::Array(a) => a,
        _ => return Err(encoder.type_mismatch(value)),
    };

    if &array.element_type != element_ty {
        return Err(encoder.type_mismatch(value));
    }

    let ndim = array.dimensions.len();
    if ndim == 0 {
        return Err(encoder.invalid_payload("array must have at least one dimension"));
    }

    let mut expected_count = 1i64;
    for dim in &array.dimensions {
        expected_count = expected_count
            .checked_mul(dim.length as i64)
            .ok_or_else(|| encoder.invalid_payload("array element count overflow"))?;
    }
    if array.elements.len() as i64 != expected_count {
        return Err(encoder.invalid_payload("array element count does not match dimensions"));
    }

    let has_nulls = if array.elements.iter().any(PgValue::is_null) {
        constants::ARRAY_HAS_NULLS_TRUE
    } else {
        constants::ARRAY_HAS_NULLS_FALSE
    };
    let element_oid = element_ty
        .oid()
        .ok_or_else(|| Error::UnsupportedType(array_ty.clone()))?;

    let element_encoder = FieldEncoder::new(encoder.column(), element_ty, true);

    let mut body = Vec::new();
    write_i32(&mut body, ndim as i32);
    write_i32(&mut body, has_nulls);
    write_i32(&mut body, element_oid as i32);
    for dim in &array.dimensions {
        write_i32(&mut body, dim.length);
        write_i32(&mut body, dim.lower_bound);
    }

    for element in &array.elements {
        match element_encoder.encode_array_element(element)? {
            ArrayElementEncoding::Null => write_i32(&mut body, constants::COPY_FIELD_NULL),
            ArrayElementEncoding::Payload(payload) => {
                write_i32(&mut body, payload.len() as i32);
                body.extend_from_slice(&payload);
            }
        }
    }

    if ndim >= constants::ARRAY_MULTI_DIM_NDIM {
        let mut buf = Vec::with_capacity(constants::ARRAY_TOTAL_LEN_BYTES + body.len());
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
