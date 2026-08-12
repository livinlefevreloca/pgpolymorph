//! Conversion trait for mapping between IR and external formats.
//!
//! Implement this trait on a format marker type in a plugin crate. The marker's
//! [`PgMorph::Native`] associated type names the native representation used in
//! both directions.
//!
//! ```ignore
//! pub struct JsonFormat;
//!
//! impl PgMorph for JsonFormat {
//!     type Native = Vec<serde_json::Value>;
//!     type Error = JsonError;
//!
//!     fn from_pg_batch(
//!         schema: &Schema,
//!         batch: &PgBatch,
//!     ) -> Result<Self::Native, Self::Error> {
//!         // transform batch rows/columns into JSON
//!     }
//!
//!     fn to_pg_batch(
//!         native: &Self::Native,
//!         schema: &Schema,
//!     ) -> Result<PgBatch, Self::Error> {
//!         // transform JSON into batch rows/columns
//!     }
//! }
//! ```

use crate::schema::Schema;
use crate::value::PgBatch;

/// Bidirectional conversion between [`PgBatch`] IR and a native format representation.
///
/// Format plugin crates implement this on a marker type (e.g. `JsonFormat`) and export
/// that type for use as a compile-time plugin selector in outer crates.
pub trait PgMorph {
    /// Native type used for both encode and decode.
    type Native;
    /// Error type for conversion failures in either direction.
    type Error;

    /// Convert from IR to the native format.
    fn from_pg_batch(
        schema: &Schema,
        batch: &PgBatch,
    ) -> std::result::Result<Self::Native, Self::Error>;

    /// Convert from the native format to IR.
    fn to_pg_batch(
        native: &Self::Native,
        schema: &Schema,
    ) -> std::result::Result<PgBatch, Self::Error>;
}
