pub(crate) mod buffer_view;
pub(crate) mod constants;
mod field;
mod header;
mod reader;
mod writer;

pub use constants::COPY_MAGIC;
pub(crate) use buffer_view::BufferView;
pub(crate) use constants::{PG_DATE_EPOCH_OFFSET_DAYS, PG_TIMESTAMP_EPOCH_OFFSET_US};
pub(crate) use field::{FieldCell, FieldReader};
pub(crate) use reader::PgBinaryReader;
pub(crate) use writer::{EncodedField, PgBinaryWriter};
