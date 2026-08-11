//! Conversion traits for mapping between IR and external formats.
//!
//! Implement these traits on your output type: `impl FromPgBatch for MyOutput` takes
//! a [`PgBatch`] and produces `MyOutput`, and `impl ToPgBatch for MyInput` does the
//! reverse. Trait definitions live here; format-specific impls live in other crates.
//!
//! ```ignore
//! impl FromPgBatch for MyOutput {
//!     type Output = MyOutput;
//!     type Error = MyError;
//!     fn from_pg_batch(schema: &Schema, batch: &PgBatch) -> Result<Self::Output, Self::Error> {
//!         // transform batch rows/columns into MyOutput
//!     }
//! }
//! ```

use crate::schema::Schema;
use crate::value::PgBatch;

/// Convert from IR to a native format representation.
pub trait FromPgBatch {
    type Output;
    type Error;
    fn from_pg_batch(
        schema: &Schema,
        batch: &PgBatch,
    ) -> std::result::Result<Self::Output, Self::Error>;
}

/// Convert from a native format representation to IR.
pub trait ToPgBatch {
    type Error;
    fn to_pg_batch(
        &self,
        schema: &Schema,
    ) -> std::result::Result<PgBatch, Self::Error>;
}
