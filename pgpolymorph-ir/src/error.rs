//! Error types for COPY binary decode and encode.

use crate::schema::PgType;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum Error {
    #[error("invalid COPY binary magic bytes")]
    InvalidMagic,

    #[error("unexpected end of input: expected {expected} bytes, available {available}")]
    UnexpectedEof {
        expected: usize,
        available: usize,
    },

    #[error("invalid COPY binary footer")]
    InvalidFooter,

    #[error("field count mismatch: expected {expected}, got {got}")]
    FieldCountMismatch { expected: i16, got: i16 },

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

    #[error("unsupported type {0:?}")]
    UnsupportedType(PgType),

    #[error("row value count mismatch: expected {expected}, got {got}")]
    SchemaRowLengthMismatch { expected: usize, got: usize },
}
