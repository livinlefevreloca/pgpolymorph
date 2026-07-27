//! Conversion traits for mapping between IR and external formats.
//!
//! Implementors receive a full [`CopyBatch`] and produce a native representation,
//! or accept a native representation and produce IR.
//!
//! ```ignore
//! impl FromCopyBatch for MyOutput {
//!     type Output = MyOutput;
//!     type Error = MyError;
//!     fn from_copy_batch(schema: &Schema, batch: &CopyBatch) -> Result<Self::Output, Self::Error> {
//!         // transform batch rows/columns into MyOutput
//!     }
//! }
//! ```

use crate::schema::Schema;
use crate::value::CopyBatch;

/// Convert from IR to a native format representation.
pub trait FromCopyBatch {
    type Output;
    type Error;
    fn from_copy_batch(
        schema: &Schema,
        batch: &CopyBatch,
    ) -> std::result::Result<Self::Output, Self::Error>;
}

/// Convert from a native format representation to IR.
pub trait ToCopyBatch {
    type Error;
    fn to_copy_batch(
        &self,
        schema: &Schema,
    ) -> std::result::Result<CopyBatch, Self::Error>;
}
