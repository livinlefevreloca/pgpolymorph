use crate::binary::FieldReader;
use crate::binary::constants;
use crate::error::{Error, Result};
use crate::schema::PgType;

/// Sign component of a PostgreSQL `numeric` value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NumericSign {
    Positive,
    Negative,
    NaN,
}

/// Arbitrary-precision decimal in PostgreSQL binary numeric layout.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PgNumeric {
    pub ndigits: i16,
    pub weight: i16,
    pub sign: NumericSign,
    pub dscale: i16,
    /// Base-10000 magnitude limbs, most significant first.
    pub digits: Vec<i16>,
}

impl PgNumeric {
    pub fn new(
        ndigits: i16,
        weight: i16,
        sign: NumericSign,
        dscale: i16,
        digits: Vec<i16>,
    ) -> Self {
        Self {
            ndigits,
            weight,
            sign,
            dscale,
            digits,
        }
    }

    pub fn zero() -> Self {
        Self {
            ndigits: 0,
            weight: 0,
            sign: NumericSign::Positive,
            dscale: 0,
            digits: Vec::new(),
        }
    }
}

const NUMERIC_POS: i16 = 0x0000;
const NUMERIC_NEG: i16 = 0x4000;
const NUMERIC_NAN: i16 = 0xC000u16 as i16;

pub(crate) fn decode_numeric(payload: &[u8], column: &str, ty: &PgType) -> Result<PgNumeric> {
    if payload.is_empty() {
        return Err(invalid_payload(column, ty, "numeric payload too short"));
    }

    // Some fixtures use a truncated all-zero header for numeric zero.
    if payload.len() < constants::NUMERIC_HEADER_BYTES {
        if payload.iter().all(|&b| b == 0) {
            return Ok(PgNumeric::zero());
        }
        return Err(invalid_payload(column, ty, "numeric payload too short"));
    }

    let mut reader = FieldReader::new(payload);
    let ndigits = reader.read_i16()?;
    let weight = reader.read_i16()?;
    let sign = decode_sign(reader.read_i16()?, column, ty)?;
    let dscale = reader.read_i16()?;

    let digit_count = ndigits.max(0) as usize;
    let mut digits = Vec::with_capacity(digit_count);
    for _ in 0..digit_count {
        digits.push(reader.read_i16()?);
    }

    if reader.remaining() != 0 {
        return Err(invalid_payload(column, ty, "numeric payload length mismatch"));
    }

    Ok(PgNumeric::new(ndigits, weight, sign, dscale, digits))
}

pub(crate) fn encode_numeric(value: &PgNumeric) -> Vec<u8> {
    let mut buf = Vec::with_capacity(constants::NUMERIC_HEADER_BYTES + value.digits.len() * constants::NUMERIC_DIGIT_BYTES);
    buf.extend_from_slice(&value.ndigits.to_be_bytes());
    buf.extend_from_slice(&value.weight.to_be_bytes());
    buf.extend_from_slice(&encode_sign(value.sign).to_be_bytes());
    buf.extend_from_slice(&value.dscale.to_be_bytes());
    for digit in &value.digits {
        buf.extend_from_slice(&digit.to_be_bytes());
    }
    buf
}

fn decode_sign(raw: i16, column: &str, ty: &PgType) -> Result<NumericSign> {
    match raw {
        NUMERIC_POS => Ok(NumericSign::Positive),
        NUMERIC_NEG => Ok(NumericSign::Negative),
        NUMERIC_NAN => Ok(NumericSign::NaN),
        _ => Err(invalid_payload(column, ty, "invalid numeric sign")),
    }
}

fn encode_sign(sign: NumericSign) -> i16 {
    match sign {
        NumericSign::Positive => NUMERIC_POS,
        NumericSign::Negative => NUMERIC_NEG,
        NumericSign::NaN => NUMERIC_NAN,
    }
}

fn invalid_payload(column: &str, ty: &PgType, reason: &'static str) -> Error {
    Error::InvalidPayload {
        column: column.to_string(),
        ty: ty.clone(),
        reason,
    }
}
