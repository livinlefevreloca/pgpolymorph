//! Conversion traits for mapping between IR and external formats.
//!
//! Implementors receive fully typed IR values — e.g. [`Value::Int4`](crate::Value::Int4)
//! wraps [`PgInt4`](crate::PgInt4), not a bare `i32`. Use [`Value::as_int4`](crate::Value::as_int4)
//! or match on [`Value`] when converting a known column type.
//!
//! ```ignore
//! impl FromIr for i32 {
//!     type Error = MyError;
//!     fn from_ir(_schema: &Schema, column: &Column, value: &Value) -> Result<Self, Self::Error> {
//!         match value {
//!             Value::Null if column.nullable => Err(MyError::Null),
//!             Value::Int4(v) => Ok(v.value),
//!             other => Err(MyError::TypeMismatch(other)),
//!         }
//!     }
//! }
//! ```

use crate::schema::{Column, Schema};
use crate::value::Value;

/// Convert from IR to a native format representation.
pub trait FromIr: Sized {
    type Error;
    fn from_ir(
        schema: &Schema,
        column: &Column,
        value: &Value,
    ) -> std::result::Result<Self, Self::Error>;
}

/// Convert from a native format representation to IR.
pub trait ToIr {
    type Error;
    fn to_ir(
        &self,
        schema: &Schema,
        column: &Column,
    ) -> std::result::Result<Value, Self::Error>;
}
