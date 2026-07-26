//! Field envelope: `int32` length prefix + optional payload.

use crate::binary::be;
use crate::binary::wire;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct FieldCell<'a> {
    pub is_null: bool,
    pub payload: &'a [u8],
}

pub(crate) struct FieldReader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> FieldReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn consumed(&self) -> usize {
        self.pos
    }

    pub fn read_field(&mut self) -> crate::error::Result<FieldCell<'a>> {
        let len =
            be::i32(self.read_bytes(wire::COPY_FIELD_LEN)?).ok_or_else(unexpected_eof)? as i64;
        if len == i64::from(wire::COPY_FIELD_NULL) {
            return Ok(FieldCell {
                is_null: true,
                payload: &[],
            });
        }
        if len < 0 {
            return Err(crate::error::Error::FieldTooLarge { len });
        }
        let payload = self.read_bytes(len as usize)?;
        Ok(FieldCell {
            is_null: false,
            payload,
        })
    }

    pub fn read_u8(&mut self) -> crate::error::Result<u8> {
        Ok(self.read_bytes(wire::U8)?[0])
    }

    pub fn read_i16(&mut self) -> crate::error::Result<i16> {
        be::i16(self.read_bytes(wire::I16)?).ok_or_else(unexpected_eof)
    }

    pub fn read_i32(&mut self) -> crate::error::Result<i32> {
        be::i32(self.read_bytes(wire::I32)?).ok_or_else(unexpected_eof)
    }

    pub fn read_i64(&mut self) -> crate::error::Result<i64> {
        be::i64(self.read_bytes(wire::I64)?).ok_or_else(unexpected_eof)
    }

    /// Return all bytes from the current position to the end.
    pub fn read_rest(&mut self) -> crate::error::Result<&'a [u8]> {
        let rest = &self.data[self.pos..];
        self.pos = self.data.len();
        Ok(rest)
    }

    pub fn read_bytes(&mut self, n: usize) -> crate::error::Result<&'a [u8]> {
        if self.pos + n > self.data.len() {
            return Err(crate::error::Error::UnexpectedEof {
                expected: n,
                available: self.remaining(),
            });
        }
        let slice = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }

    pub fn peek_i16(&self) -> crate::error::Result<i16> {
        if self.remaining() < wire::I16 {
            return Err(crate::error::Error::UnexpectedEof {
                expected: wire::I16,
                available: self.remaining(),
            });
        }
        be::i16(&self.data[self.pos..self.pos + wire::I16]).ok_or_else(unexpected_eof)
    }
}

fn unexpected_eof() -> crate::error::Error {
    crate::error::Error::UnexpectedEof {
        expected: 0,
        available: 0,
    }
}
