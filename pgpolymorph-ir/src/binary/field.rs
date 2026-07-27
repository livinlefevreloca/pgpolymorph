//! Field envelope: `int32` length prefix + optional payload.

use crate::binary::be;
use crate::binary::constants;

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

    pub fn read_field(&mut self) -> crate::error::Result<FieldCell<'a>> {
        let len = be::read_be_i32(self.read_bytes(constants::COPY_FIELD_LEN_BYTES)?)
            .expect("read_bytes guarantees exact length") as i64;
        if len == i64::from(constants::COPY_FIELD_NULL) {
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
        Ok(self.read_bytes(constants::U8_BYTES)?[0])
    }

    pub fn read_i16(&mut self) -> crate::error::Result<i16> {
        Ok(be::read_be_i16(self.read_bytes(constants::I16_BYTES)?)
            .expect("read_bytes guarantees exact length"))
    }

    pub fn read_i32(&mut self) -> crate::error::Result<i32> {
        Ok(be::read_be_i32(self.read_bytes(constants::I32_BYTES)?)
            .expect("read_bytes guarantees exact length"))
    }

    pub fn read_i64(&mut self) -> crate::error::Result<i64> {
        Ok(be::read_be_i64(self.read_bytes(constants::I64_BYTES)?)
            .expect("read_bytes guarantees exact length"))
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
        if self.remaining() < constants::I16_BYTES {
            return Err(crate::error::Error::UnexpectedEof {
                expected: constants::I16_BYTES,
                available: self.remaining(),
            });
        }
        Ok(be::read_be_i16(&self.data[self.pos..self.pos + constants::I16_BYTES])
            .expect("length checked"))
    }
}
