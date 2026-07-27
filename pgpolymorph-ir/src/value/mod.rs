//! Intermediate representation of decoded PostgreSQL values.

pub mod pgtypes;

pub use pgtypes::{ArrayDimension, NumericSign};

/// A single decoded column value. Each PostgreSQL type maps to a dedicated IR struct.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
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
pub struct Row {
    pub values: Vec<Value>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CopyBatch {
    pub rows: Vec<Row>,
}

pub fn value_variant_name(value: &Value) -> &'static str {
    match value {
        Value::Null => "Null",
        Value::Bool(_) => "Bool",
        Value::Bytea(_) => "Bytea",
        Value::Char(_) => "Char",
        Value::Int2(_) => "Int2",
        Value::Int4(_) => "Int4",
        Value::Int8(_) => "Int8",
        Value::Float4(_) => "Float4",
        Value::Float8(_) => "Float8",
        Value::Text(_) => "Text",
        Value::Name(_) => "Name",
        Value::Json(_) => "Json",
        Value::Jsonb(_) => "Jsonb",
        Value::Date(_) => "Date",
        Value::Time(_) => "Time",
        Value::Timestamp(_) => "Timestamp",
        Value::Timestamptz(_) => "Timestamptz",
        Value::Timetz(_) => "Timetz",
        Value::Interval(_) => "Interval",
        Value::Numeric(_) => "Numeric",
        Value::Uuid(_) => "Uuid",
        Value::Money(_) => "Money",
        Value::Oid(_) => "Oid",
        Value::Array(_) => "Array",
    }
}

impl Value {
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn as_bool(&self) -> Option<&pgtypes::PgBool> {
        match self {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bytea(&self) -> Option<&pgtypes::PgBytea> {
        match self {
            Value::Bytea(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_char(&self) -> Option<&pgtypes::PgChar> {
        match self {
            Value::Char(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int2(&self) -> Option<&pgtypes::PgInt2> {
        match self {
            Value::Int2(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int4(&self) -> Option<&pgtypes::PgInt4> {
        match self {
            Value::Int4(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int8(&self) -> Option<&pgtypes::PgInt8> {
        match self {
            Value::Int8(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_float4(&self) -> Option<&pgtypes::PgFloat4> {
        match self {
            Value::Float4(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_float8(&self) -> Option<&pgtypes::PgFloat8> {
        match self {
            Value::Float8(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&pgtypes::PgText> {
        match self {
            Value::Text(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_name(&self) -> Option<&pgtypes::PgName> {
        match self {
            Value::Name(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_json(&self) -> Option<&pgtypes::PgJson> {
        match self {
            Value::Json(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_jsonb(&self) -> Option<&pgtypes::PgJsonb> {
        match self {
            Value::Jsonb(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_date(&self) -> Option<&pgtypes::PgDate> {
        match self {
            Value::Date(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_time(&self) -> Option<&pgtypes::PgTime> {
        match self {
            Value::Time(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<&pgtypes::PgTimestamp> {
        match self {
            Value::Timestamp(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timestamptz(&self) -> Option<&pgtypes::PgTimestamptz> {
        match self {
            Value::Timestamptz(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timetz(&self) -> Option<&pgtypes::PgTimetz> {
        match self {
            Value::Timetz(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_interval(&self) -> Option<&pgtypes::PgInterval> {
        match self {
            Value::Interval(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_numeric(&self) -> Option<&pgtypes::PgNumeric> {
        match self {
            Value::Numeric(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_uuid(&self) -> Option<&pgtypes::PgUuid> {
        match self {
            Value::Uuid(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_money(&self) -> Option<&pgtypes::PgMoney> {
        match self {
            Value::Money(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_oid(&self) -> Option<&pgtypes::PgOid> {
        match self {
            Value::Oid(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&pgtypes::PgArray> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }
}
