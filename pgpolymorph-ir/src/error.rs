//! Error types for COPY binary decode and encode.

use crate::schema::PgType;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("malformed COPY binary input: {reason}")]
    MalformedInput { reason: &'static str },

    #[error("unknown array element OID in column {column}: {element_oid}")]
    UnknownArrayElementOid {
        column: String,
        element_oid: u32,
    },

    #[error("invalid array dimension length in column {column}: got {got}")]
    InvalidArrayDimensionLength { column: String, got: i32 },

    #[error("invalid COPY binary magic bytes")]
    InvalidMagic,

    #[error("invalid COPY binary header: {reason}")]
    InvalidHeader { reason: &'static str },

    #[error("unexpected end of input: expected {expected} bytes, available {available}")]
    UnexpectedEof {
        expected: usize,
        available: usize,
    },

    #[error("invalid COPY binary footer")]
    InvalidFooter,

    #[error("field count mismatch: expected {expected}, got {got}")]
    FieldCountMismatch { expected: i16, got: i16 },

    #[error("too many columns for COPY binary format: max {max}, got {got}")]
    TooManyColumns { max: i16, got: usize },

    #[error("field payload too large: {len} bytes")]
    FieldTooLarge { len: i64 },

    #[error("unexpected NULL in non-nullable column {column}")]
    UnexpectedNull { column: String },

    #[error("type mismatch in column {column}: expected {expected:?}, got {got}")]
    TypeMismatch {
        column: String,
        expected: PgType,
        got: String,
    },

    #[error("invalid payload in column {column} for type {ty:?}: {reason}")]
    InvalidPayload {
        column: String,
        ty: PgType,
        reason: &'static str,
    },

    #[error("array element OID mismatch in column {column}: expected {expected:?}, got OID {element_oid}")]
    ArrayElementOidMismatch {
        column: String,
        expected: PgType,
        element_oid: u32,
    },

    #[error("invalid array has_nulls flag in column {column}: got {got}")]
    InvalidArrayHasNulls { column: String, got: i32 },

    #[error("unsupported type {0:?}")]
    UnsupportedType(PgType),

    #[error("row value count mismatch: expected {expected}, got {got}")]
    SchemaRowLengthMismatch { expected: usize, got: usize },
}
