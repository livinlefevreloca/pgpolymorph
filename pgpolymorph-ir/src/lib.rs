//! Bidirectional converter between PostgreSQL `FORMAT binary` COPY blobs and typed IR.
//!
//! See `SPEC.md` in the crate root for wire layout, IR design, and validation rules.

mod binary;
pub mod error;
pub mod schema;
pub mod traits;
mod codec;
pub mod value;

pub use binary::COPY_MAGIC;
pub use error::{Error, Result};
pub use schema::{Column, PgType, Schema};
pub use traits::{FromIr, ToIr};
pub use value::{
    types::{
        ArrayDimension, NumericSign, PgArray, PgBool, PgBytea, PgChar, PgDate, PgFloat4, PgFloat8,
        PgInt2, PgInt4, PgInt8, PgInterval, PgJson, PgJsonb, PgMoney, PgName, PgNumeric, PgOid,
        PgText, PgTime, PgTimestamp, PgTimestamptz, PgTimetz, PgUuid,
    },
    value_variant_name, CopyBatch, Row, Value,
};

use binary::{write_footer, write_tuple, CopyReader, CopyWriter, EncodedField, FieldCell};
use codec::{decode_field, encode_field};
use value::Row as IrRow;

/// Decode a PostgreSQL `FORMAT binary` COPY blob into typed IR using the given schema.
pub fn decode(schema: &Schema, binary: &[u8]) -> Result<CopyBatch> {
    let mut reader = CopyReader::new(schema, binary)?;
    let mut rows = Vec::new();

    while let Some(cells) = reader.next_tuple_raw()? {
        rows.push(decode_row(schema, &cells)?);
    }

    reader.ensure_finished()?;
    Ok(CopyBatch { rows })
}

/// Encode typed IR into a PostgreSQL `FORMAT binary` COPY blob using the given schema.
pub fn encode(schema: &Schema, batch: &CopyBatch) -> Result<Vec<u8>> {
    let mut writer = CopyWriter::new();
    for row in &batch.rows {
        write_encoded_row(schema, &mut writer, row)?;
    }
    write_footer(&mut writer);
    Ok(writer.finish())
}

/// Incremental decoder for large in-memory COPY binary blobs.
pub struct Decoder<'a> {
    reader: CopyReader<'a>,
    schema: &'a Schema,
}

impl<'a> Decoder<'a> {
    /// Create a decoder positioned after the COPY header.
    pub fn new(schema: &'a Schema, binary: &'a [u8]) -> Result<Self> {
        Ok(Self {
            reader: CopyReader::new(schema, binary)?,
            schema,
        })
    }

    /// Decode the next row, or `None` after the footer sentinel.
    pub fn next_row(&mut self) -> Result<Option<IrRow>> {
        match self.reader.next_tuple_raw()? {
            Some(cells) => Ok(Some(decode_row(self.schema, &cells)?)),
            None => {
                self.reader.ensure_finished()?;
                Ok(None)
            }
        }
    }
}

/// Incremental encoder that builds a COPY binary blob row-at-a-time.
pub struct Encoder<'a> {
    schema: &'a Schema,
    writer: CopyWriter,
    finished: bool,
}

impl<'a> Encoder<'a> {
    /// Create an encoder with the COPY header already written.
    pub fn new(schema: &'a Schema) -> Self {
        Self {
            schema,
            writer: CopyWriter::new(),
            finished: false,
        }
    }

    /// Append one row to the output blob.
    pub fn write_row(&mut self, row: &IrRow) -> Result<()> {
        if self.finished {
            return Err(Error::InvalidFooter);
        }
        write_encoded_row(self.schema, &mut self.writer, row)
    }

    /// Finalize the blob with the footer sentinel.
    pub fn finish(mut self) -> Result<Vec<u8>> {
        if !self.finished {
            write_footer(&mut self.writer);
            self.finished = true;
        }
        Ok(self.writer.finish())
    }
}

fn decode_row(schema: &Schema, cells: &[FieldCell<'_>]) -> Result<IrRow> {
    let mut values = Vec::with_capacity(schema.columns.len());
    for (column, cell) in schema.columns.iter().zip(cells.iter()) {
        values.push(decode_field(
            &column.ty,
            cell,
            &column.name,
            column.nullable,
        )?);
    }
    Ok(IrRow { values })
}

fn write_encoded_row(schema: &Schema, writer: &mut CopyWriter, row: &IrRow) -> Result<()> {
    if row.values.len() != schema.columns.len() {
        return Err(Error::SchemaRowLengthMismatch {
            expected: schema.columns.len(),
            got: row.values.len(),
        });
    }

    let fields = schema
        .columns
        .iter()
        .zip(row.values.iter())
        .map(|(column, value)| {
            encode_field(value, &column.ty, &column.name, column.nullable)
        })
        .collect::<Result<Vec<EncodedField>>>()?;

    write_tuple(
        writer,
        i16::try_from(schema.columns.len()).unwrap_or(i16::MAX),
        fields,
    )
}
