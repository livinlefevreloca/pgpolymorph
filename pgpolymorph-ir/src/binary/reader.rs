//! Incremental COPY binary blob parser.

use crate::binary::constants::{self, FOOTER_SENTINEL};
use crate::binary::field::{FieldCell, FieldReader};
use crate::binary::header::PgBinaryHeader;
use crate::error::{Error, Result};
use crate::schema::Schema;

pub(crate) struct PgBinaryReader<'a> {
    reader: FieldReader<'a>,
    schema: &'a Schema,
    finished: bool,
}

impl<'a> PgBinaryReader<'a> {
    pub fn new(schema: &'a Schema, data: &'a [u8]) -> Result<Self> {
        if schema.columns.len() > i16::MAX as usize {
            return Err(Error::TooManyColumns {
                max: i16::MAX,
                got: schema.columns.len(),
            });
        }
        let (_, offset) = PgBinaryHeader::parse(data)?;
        Ok(Self {
            reader: FieldReader::new(&data[offset..]),
            schema,
            finished: false,
        })
    }

    pub fn next_tuple_raw(&mut self) -> Result<Option<Vec<FieldCell<'a>>>> {
        if self.finished {
            return Ok(None);
        }

        if self.reader.remaining() < constants::COPY_FIELD_COUNT_BYTES {
            return Err(Error::UnexpectedEof {
                expected: constants::COPY_FIELD_COUNT_BYTES,
                available: self.reader.remaining(),
            });
        }

        if self.try_consume_footer()? {
            return Ok(None);
        }

        let field_count = self.reader.read_i16()?;
        let expected = i16::try_from(self.schema.columns.len()).expect("checked in new");
        if field_count != expected {
            return Err(Error::FieldCountMismatch {
                expected,
                got: field_count,
            });
        }

        let mut fields = Vec::with_capacity(field_count as usize);
        for _ in 0..field_count {
            fields.push(self.reader.read_field()?);
        }
        Ok(Some(fields))
    }

    pub fn ensure_finished(&mut self) -> Result<()> {
        if self.finished {
            return Ok(());
        }
        if self.reader.remaining() == 0 {
            return Err(Error::UnexpectedEof {
                expected: constants::COPY_FIELD_COUNT_BYTES,
                available: 0,
            });
        }
        // Caller received `Some(last_row)` and skipped the final `next_tuple_raw` that
        // would have consumed the footer sentinel.
        if !self.try_consume_footer()? {
            return Err(Error::InvalidFooter);
        }
        Ok(())
    }

    fn try_consume_footer(&mut self) -> Result<bool> {
        if self.reader.peek_i16()? == FOOTER_SENTINEL {
            self.reader.read_i16()?;
            self.finished = true;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
