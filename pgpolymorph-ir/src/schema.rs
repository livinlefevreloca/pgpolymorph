//! Schema types describing column names and PostgreSQL types.

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PgType {
    Bool,
    Bytea,
    Char,
    Int2,
    Int4,
    Int8,
    Float4,
    Float8,
    Text,
    Json,
    Jsonb,
    Date,
    Time,
    Timestamp,
    Timestamptz,
    Timetz,
    Interval,
    Numeric,
    Uuid,
    Money,
    Oid,
    Name,
    Array(Box<PgType>),
}

impl PgType {
    pub fn oid(&self) -> Option<u32> {
        Some(match self {
            PgType::Bool => 16,
            PgType::Bytea => 17,
            PgType::Char => 18,
            PgType::Int2 => 21,
            PgType::Int4 => 23,
            PgType::Int8 => 20,
            PgType::Text => 25,
            PgType::Json => 114,
            PgType::Jsonb => 3802,
            PgType::Float4 => 700,
            PgType::Float8 => 701,
            PgType::Date => 1082,
            PgType::Time => 1083,
            PgType::Timestamp => 1114,
            PgType::Timestamptz => 1184,
            PgType::Timetz => 1266,
            PgType::Interval => 1186,
            PgType::Numeric => 1700,
            PgType::Uuid => 2950,
            PgType::Money => 790,
            PgType::Oid => 26,
            PgType::Name => 19,
            PgType::Array(_) => return None,
        })
    }

    pub fn from_oid(oid: u32) -> Option<Self> {
        Some(match oid {
            16 => PgType::Bool,
            17 => PgType::Bytea,
            18 => PgType::Char,
            20 => PgType::Int8,
            21 => PgType::Int2,
            23 => PgType::Int4,
            25 => PgType::Text,
            114 => PgType::Json,
            3802 => PgType::Jsonb,
            700 => PgType::Float4,
            701 => PgType::Float8,
            1082 => PgType::Date,
            1083 => PgType::Time,
            1114 => PgType::Timestamp,
            1184 => PgType::Timestamptz,
            1266 => PgType::Timetz,
            1186 => PgType::Interval,
            1700 => PgType::Numeric,
            2950 => PgType::Uuid,
            790 => PgType::Money,
            26 => PgType::Oid,
            19 => PgType::Name,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Column {
    pub name: String,
    pub ty: PgType,
    pub nullable: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Schema {
    pub columns: Vec<Column>,
}
