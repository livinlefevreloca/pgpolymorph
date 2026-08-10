//! Bidirectional converter between PostgreSQL `FORMAT binary` COPY blobs and typed IR.
//!
//! See `SPEC.md` in the crate root for binary layout, IR design, and validation rules.

mod binary;
pub mod error;
pub mod schema;
pub mod traits;
mod codec;
pub mod value;

pub use binary::COPY_MAGIC;
pub use error::{Error, Result};
pub use schema::{Column, PgType, Schema};
pub use traits::{FromPgBatch, ToPgBatch};
pub use value::{
    pgtypes::{
        ArrayDimension, NumericSign, PgArray, PgBool, PgBytea, PgChar, PgDate, PgFloat4, PgFloat8,
        PgInt2, PgInt4, PgInt8, PgInterval, PgJson, PgJsonb, PgMoney, PgName, PgNumeric, PgOid,
        PgText, PgTime, PgTimestamp, PgTimestamptz, PgTimetz, PgUuid, PgVarchar,
    },
    pg_value_variant_name, PgBatch, PgRow, PgValue,
};

use binary::{BufferView, EncodedField, FieldCell, PgBinaryReader, PgBinaryWriter};
use codec::{FieldDecoder, FieldEncoder};

/// Decode a PostgreSQL `FORMAT binary` COPY blob into typed IR using the given schema.
pub fn decode(schema: &Schema, binary: &[u8]) -> Result<PgBatch> {
    let mut rows = Vec::new();
    for row in Decoder::new(schema, binary)? {
        rows.push(row?);
    }
    Ok(PgBatch { rows })
}

/// Encode typed IR into a PostgreSQL `FORMAT binary` COPY blob using the given schema.
pub fn encode(schema: &Schema, batch: &PgBatch) -> Result<Vec<u8>> {
    let mut encoder = Encoder::new(schema);
    for row in &batch.rows {
        encoder.write_row(row)?;
    }
    encoder.finish()
}

/// Incremental decoder for large in-memory COPY binary blobs.
pub struct Decoder<'a> {
    reader: PgBinaryReader<'a>,
    schema: &'a Schema,
}

impl<'a> Decoder<'a> {
    /// Create a decoder positioned after the COPY header.
    pub fn new(schema: &'a Schema, binary: &'a [u8]) -> Result<Self> {
        Ok(Self {
            reader: PgBinaryReader::new(schema, BufferView::new(binary))?,
            schema,
        })
    }

    fn decode_row(&self, cells: Vec<FieldCell<'_>>) -> Result<PgRow> {
        let mut values = Vec::with_capacity(self.schema.columns.len());
        for (column, cell) in self.schema.columns.iter().zip(cells) {
            values.push(
                FieldDecoder::new(&column.name, &column.ty, column.nullable).decode(cell)?,
            );
        }
        Ok(PgRow { values })
    }
}

impl<'a> Iterator for Decoder<'a> {
    type Item = Result<PgRow>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.next_tuple_raw() {
            Ok(Some(cells)) => Some(self.decode_row(cells)),
            Ok(None) => {
                if let Err(err) = self.reader.ensure_finished() {
                    Some(Err(err))
                } else {
                    None
                }
            }
            Err(err) => Some(Err(err)),
        }
    }
}

/// Incremental encoder that builds a COPY binary blob row-at-a-time.
pub struct Encoder<'a> {
    schema: &'a Schema,
    writer: PgBinaryWriter,
    finished: bool,
}

impl<'a> Encoder<'a> {
    /// Create an encoder with the COPY header already written.
    pub fn new(schema: &'a Schema) -> Self {
        Self {
            schema,
            writer: PgBinaryWriter::new(),
            finished: false,
        }
    }

    /// Append one row to the output blob.
    pub fn write_row(&mut self, row: &PgRow) -> Result<()> {
        if self.finished {
            return Err(Error::InvalidFooter);
        }
        self.write_encoded_row(row)
    }

    /// Finalize the blob with the footer sentinel.
    pub fn finish(mut self) -> Result<Vec<u8>> {
        if !self.finished {
            self.writer.write_footer();
            self.finished = true;
        }
        Ok(self.writer.finish())
    }

    fn write_encoded_row(&mut self, row: &PgRow) -> Result<()> {
        if row.values.len() != self.schema.columns.len() {
            return Err(Error::SchemaRowLengthMismatch {
                expected: self.schema.columns.len(),
                got: row.values.len(),
            });
        }

        if self.schema.columns.len() > i16::MAX as usize {
            return Err(Error::TooManyColumns {
                max: i16::MAX,
                got: self.schema.columns.len(),
            });
        }

        let fields = self
            .schema
            .columns
            .iter()
            .zip(row.values.iter())
            .map(|(column, value)| {
                FieldEncoder::new(&column.name, &column.ty, column.nullable).encode(value)
            })
            .collect::<Result<Vec<EncodedField>>>()?;

        self.writer
            .write_tuple(self.schema.columns.len() as i16, fields)
    }
}
