//! Intermediate representation of decoded PostgreSQL values.

pub mod pgtypes;

pub use pgtypes::{ArrayDimension, NumericSign};

/// A single decoded column value. Each PostgreSQL type maps to a dedicated IR struct.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PgValue {
    Null,
    Bool(pgtypes::PgBool),
    Bytea(pgtypes::PgBytea),
    Char(pgtypes::PgChar),
    Int2(pgtypes::PgInt2),
    Int4(pgtypes::PgInt4),
    Int8(pgtypes::PgInt8),
    Float4(pgtypes::PgFloat4),
    Float8(pgtypes::PgFloat8),
    Text(pgtypes::PgText),
    Name(pgtypes::PgName),
    Json(pgtypes::PgJson),
    Jsonb(pgtypes::PgJsonb),
    Date(pgtypes::PgDate),
    Time(pgtypes::PgTime),
    Timestamp(pgtypes::PgTimestamp),
    Timestamptz(pgtypes::PgTimestamptz),
    Timetz(pgtypes::PgTimetz),
    Interval(pgtypes::PgInterval),
    Numeric(pgtypes::PgNumeric),
    Uuid(pgtypes::PgUuid),
    Money(pgtypes::PgMoney),
    Oid(pgtypes::PgOid),
    Array(pgtypes::PgArray),
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgRow {
    pub values: Vec<PgValue>,
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgBatch {
    pub rows: Vec<PgRow>,
}

pub fn pg_value_variant_name(value: &PgValue) -> &'static str {
    match value {
        PgValue::Null => "Null",
        PgValue::Bool(_) => "Bool",
        PgValue::Bytea(_) => "Bytea",
        PgValue::Char(_) => "Char",
        PgValue::Int2(_) => "Int2",
        PgValue::Int4(_) => "Int4",
        PgValue::Int8(_) => "Int8",
        PgValue::Float4(_) => "Float4",
        PgValue::Float8(_) => "Float8",
        PgValue::Text(_) => "Text",
        PgValue::Name(_) => "Name",
        PgValue::Json(_) => "Json",
        PgValue::Jsonb(_) => "Jsonb",
        PgValue::Date(_) => "Date",
        PgValue::Time(_) => "Time",
        PgValue::Timestamp(_) => "Timestamp",
        PgValue::Timestamptz(_) => "Timestamptz",
        PgValue::Timetz(_) => "Timetz",
        PgValue::Interval(_) => "Interval",
        PgValue::Numeric(_) => "Numeric",
        PgValue::Uuid(_) => "Uuid",
        PgValue::Money(_) => "Money",
        PgValue::Oid(_) => "Oid",
        PgValue::Array(_) => "Array",
    }
}

impl PgValue {
    pub fn is_null(&self) -> bool {
        matches!(self, PgValue::Null)
    }

    pub fn as_bool(&self) -> Option<&pgtypes::PgBool> {
        match self {
            PgValue::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bytea(&self) -> Option<&pgtypes::PgBytea> {
        match self {
            PgValue::Bytea(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_char(&self) -> Option<&pgtypes::PgChar> {
        match self {
            PgValue::Char(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int2(&self) -> Option<&pgtypes::PgInt2> {
        match self {
            PgValue::Int2(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int4(&self) -> Option<&pgtypes::PgInt4> {
        match self {
            PgValue::Int4(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int8(&self) -> Option<&pgtypes::PgInt8> {
        match self {
            PgValue::Int8(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_float4(&self) -> Option<&pgtypes::PgFloat4> {
        match self {
            PgValue::Float4(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_float8(&self) -> Option<&pgtypes::PgFloat8> {
        match self {
            PgValue::Float8(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&pgtypes::PgText> {
        match self {
            PgValue::Text(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_name(&self) -> Option<&pgtypes::PgName> {
        match self {
            PgValue::Name(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_json(&self) -> Option<&pgtypes::PgJson> {
        match self {
            PgValue::Json(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_jsonb(&self) -> Option<&pgtypes::PgJsonb> {
        match self {
            PgValue::Jsonb(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_date(&self) -> Option<&pgtypes::PgDate> {
        match self {
            PgValue::Date(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_time(&self) -> Option<&pgtypes::PgTime> {
        match self {
            PgValue::Time(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<&pgtypes::PgTimestamp> {
        match self {
            PgValue::Timestamp(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timestamptz(&self) -> Option<&pgtypes::PgTimestamptz> {
        match self {
            PgValue::Timestamptz(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timetz(&self) -> Option<&pgtypes::PgTimetz> {
        match self {
            PgValue::Timetz(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_interval(&self) -> Option<&pgtypes::PgInterval> {
        match self {
            PgValue::Interval(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_numeric(&self) -> Option<&pgtypes::PgNumeric> {
        match self {
            PgValue::Numeric(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_uuid(&self) -> Option<&pgtypes::PgUuid> {
        match self {
            PgValue::Uuid(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_money(&self) -> Option<&pgtypes::PgMoney> {
        match self {
            PgValue::Money(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_oid(&self) -> Option<&pgtypes::PgOid> {
        match self {
            PgValue::Oid(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&pgtypes::PgArray> {
        match self {
            PgValue::Array(v) => Some(v),
            _ => None,
        }
    }
}
