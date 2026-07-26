pub(crate) mod be;
mod constants;
mod field;
mod header;
mod reader;
mod writer;
pub(crate) mod wire;

pub use constants::COPY_MAGIC;
pub(crate) use constants::{PG_DATE_EPOCH_OFFSET_DAYS, PG_TIMESTAMP_EPOCH_OFFSET_US};
pub(crate) use field::{FieldCell, FieldReader};
pub(crate) use reader::CopyReader;
pub(crate) use writer::{write_footer, write_tuple, CopyWriter, EncodedField};
