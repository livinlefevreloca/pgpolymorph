//! Intermediate representation of decoded PostgreSQL values.

pub mod types;

pub use types::{ArrayDimension, NumericSign};

/// A single decoded column value. Each PostgreSQL type maps to a dedicated IR struct.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(types::PgBool),
    Bytea(types::PgBytea),
    Char(types::PgChar),
    Int2(types::PgInt2),
    Int4(types::PgInt4),
    Int8(types::PgInt8),
    Float4(types::PgFloat4),
    Float8(types::PgFloat8),
    Text(types::PgText),
    Name(types::PgName),
    Json(types::PgJson),
    Jsonb(types::PgJsonb),
    Date(types::PgDate),
    Time(types::PgTime),
    Timestamp(types::PgTimestamp),
    Timestamptz(types::PgTimestamptz),
    Timetz(types::PgTimetz),
    Interval(types::PgInterval),
    Numeric(types::PgNumeric),
    Uuid(types::PgUuid),
    Money(types::PgMoney),
    Oid(types::PgOid),
    Array(types::PgArray),
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

    pub fn as_bool(&self) -> Option<&types::PgBool> {
        match self {
            Value::Bool(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_bytea(&self) -> Option<&types::PgBytea> {
        match self {
            Value::Bytea(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_char(&self) -> Option<&types::PgChar> {
        match self {
            Value::Char(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int2(&self) -> Option<&types::PgInt2> {
        match self {
            Value::Int2(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int4(&self) -> Option<&types::PgInt4> {
        match self {
            Value::Int4(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_int8(&self) -> Option<&types::PgInt8> {
        match self {
            Value::Int8(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_float4(&self) -> Option<&types::PgFloat4> {
        match self {
            Value::Float4(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_float8(&self) -> Option<&types::PgFloat8> {
        match self {
            Value::Float8(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_text(&self) -> Option<&types::PgText> {
        match self {
            Value::Text(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_name(&self) -> Option<&types::PgName> {
        match self {
            Value::Name(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_json(&self) -> Option<&types::PgJson> {
        match self {
            Value::Json(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_jsonb(&self) -> Option<&types::PgJsonb> {
        match self {
            Value::Jsonb(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_date(&self) -> Option<&types::PgDate> {
        match self {
            Value::Date(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_time(&self) -> Option<&types::PgTime> {
        match self {
            Value::Time(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timestamp(&self) -> Option<&types::PgTimestamp> {
        match self {
            Value::Timestamp(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timestamptz(&self) -> Option<&types::PgTimestamptz> {
        match self {
            Value::Timestamptz(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_timetz(&self) -> Option<&types::PgTimetz> {
        match self {
            Value::Timetz(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_interval(&self) -> Option<&types::PgInterval> {
        match self {
            Value::Interval(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_numeric(&self) -> Option<&types::PgNumeric> {
        match self {
            Value::Numeric(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_uuid(&self) -> Option<&types::PgUuid> {
        match self {
            Value::Uuid(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_money(&self) -> Option<&types::PgMoney> {
        match self {
            Value::Money(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_oid(&self) -> Option<&types::PgOid> {
        match self {
            Value::Oid(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_array(&self) -> Option<&types::PgArray> {
        match self {
            Value::Array(v) => Some(v),
            _ => None,
        }
    }
}
