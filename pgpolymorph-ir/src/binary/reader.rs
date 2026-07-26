//! Incremental COPY binary blob parser.

use crate::binary::constants::FOOTER_SENTINEL;
use crate::binary::field::{FieldCell, FieldReader};
use crate::binary::header::CopyHeader;
use crate::binary::wire;
use crate::error::{Error, Result};
use crate::schema::Schema;

pub(crate) struct CopyReader<'a> {
    reader: FieldReader<'a>,
    schema: &'a Schema,
    finished: bool,
}

impl<'a> CopyReader<'a> {
    pub fn new(schema: &'a Schema, data: &'a [u8]) -> Result<Self> {
        let (_, offset) = CopyHeader::parse(data)?;
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

        if self.reader.remaining() < wire::COPY_FIELD_COUNT {
            return Err(Error::UnexpectedEof {
                expected: wire::COPY_FIELD_COUNT,
                available: self.reader.remaining(),
            });
        }

        let marker = self.reader.peek_i16()?;
        if marker == FOOTER_SENTINEL {
            self.reader.read_i16()?;
            self.finished = true;
            return Ok(None);
        }

        let field_count = self.reader.read_i16()?;
        let expected = i16::try_from(self.schema.columns.len()).unwrap_or(i16::MAX);
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
                expected: wire::COPY_FIELD_COUNT,
                available: 0,
            });
        }
        let marker = self.reader.read_i16()?;
        if marker != FOOTER_SENTINEL {
            return Err(Error::InvalidFooter);
        }
        self.finished = true;
        Ok(())
    }
}
