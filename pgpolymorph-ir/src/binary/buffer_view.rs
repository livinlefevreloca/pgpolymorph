//! Cursor over a byte slice with consumed-byte tracking.

use crate::binary::be;
use crate::binary::constants;
use crate::error::{Error, Result};

pub(crate) struct BufferView<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> BufferView<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, pos: 0 }
    }

    pub fn remaining(&self) -> usize {
        self.data.len().saturating_sub(self.pos)
    }

    pub fn consumed(&self) -> usize {
        self.pos
    }

    pub fn read_bytes(&mut self, n: usize) -> Result<&'a [u8]> {
        if self.pos + n > self.data.len() {
            return Err(Error::UnexpectedEof {
                expected: n,
                available: self.remaining(),
            });
        }
        let slice = &self.data[self.pos..self.pos + n];
        self.pos += n;
        Ok(slice)
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_bytes(constants::U8_BYTES)?[0])
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(be::read_be_i16(self.read_bytes(constants::I16_BYTES)?)
            .expect("read_bytes guarantees exact length"))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(be::read_be_i32(self.read_bytes(constants::I32_BYTES)?)
            .expect("read_bytes guarantees exact length"))
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        Ok(be::read_be_i64(self.read_bytes(constants::I64_BYTES)?)
            .expect("read_bytes guarantees exact length"))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(be::read_be_u32(self.read_bytes(constants::I32_BYTES)?)
            .expect("read_bytes guarantees exact length"))
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(be::read_be_f32(self.read_bytes(constants::FLOAT4_PAYLOAD_BYTES)?)
            .expect("read_bytes guarantees exact length"))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(be::read_be_f64(self.read_bytes(constants::FLOAT8_PAYLOAD_BYTES)?)
            .expect("read_bytes guarantees exact length"))
    }

    /// Return all bytes from the current position to the end.
    pub fn read_rest(&mut self) -> Result<&'a [u8]> {
        let rest = &self.data[self.pos..];
        self.pos = self.data.len();
        Ok(rest)
    }

    pub fn peek_i16(&self) -> Result<i16> {
        if self.remaining() < constants::I16_BYTES {
            return Err(Error::UnexpectedEof {
                expected: constants::I16_BYTES,
                available: self.remaining(),
            });
        }
        Ok(be::read_be_i16(&self.data[self.pos..self.pos + constants::I16_BYTES])
            .expect("length checked"))
    }
}
