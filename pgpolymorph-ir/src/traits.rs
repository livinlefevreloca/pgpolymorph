//! Conversion traits for mapping between IR and external formats.
//!
//! Implementors receive a full [`PgBatch`] and produce a native representation,
//! or accept a native representation and produce IR.
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
