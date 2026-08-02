//! Cursor over a byte slice — all binary parsing goes through here.

use crate::binary::constants;
use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    pub fn peek_bytes(&self, n: usize) -> Result<&'a [u8]> {
        if self.remaining() < n {
            return Err(Error::UnexpectedEof {
                expected: n,
                available: self.remaining(),
            });
        }
        Ok(&self.data[self.pos..self.pos + n])
    }

    pub fn read_fixed<const N: usize>(&mut self) -> Result<[u8; N]> {
        self.read_bytes(N)?
            .try_into()
            .map_err(|_| Error::UnexpectedEof {
                expected: N,
                available: N.saturating_sub(1),
            })
    }

    pub fn read_u8(&mut self) -> Result<u8> {
        Ok(self.read_fixed::<{ constants::U8_BYTES }>()?[0])
    }

    pub fn read_i16(&mut self) -> Result<i16> {
        Ok(i16::from_be_bytes(
            self.read_fixed::<{ constants::I16_BYTES }>()?,
        ))
    }

    pub fn read_i32(&mut self) -> Result<i32> {
        Ok(i32::from_be_bytes(
            self.read_fixed::<{ constants::I32_BYTES }>()?,
        ))
    }

    pub fn read_i64(&mut self) -> Result<i64> {
        Ok(i64::from_be_bytes(
            self.read_fixed::<{ constants::I64_BYTES }>()?,
        ))
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        Ok(u32::from_be_bytes(
            self.read_fixed::<{ constants::I32_BYTES }>()?,
        ))
    }

    pub fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_be_bytes(
            self.read_fixed::<{ constants::F32_BYTES }>()?,
        ))
    }

    pub fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_be_bytes(
            self.read_fixed::<{ constants::F64_BYTES }>()?,
        ))
    }

    /// Advance by `n` bytes and return a view over that span from offset zero.
    pub fn to_remaining_view(&mut self, n: usize) -> Result<BufferView<'a>> {
        Ok(BufferView::new(self.read_bytes(n)?))
    }

    /// Return all bytes from the current position to the end.
    pub fn read_rest(&mut self) -> Result<&'a [u8]> {
        let rest = &self.data[self.pos..];
        self.pos = self.data.len();
        Ok(rest)
    }

    pub fn peek_i16(&self) -> Result<i16> {
        Ok(i16::from_be_bytes(
            self.peek_bytes(constants::I16_BYTES)?
                .try_into()
                .map_err(|_| Error::UnexpectedEof {
                    expected: constants::I16_BYTES,
                    available: self.remaining(),
                })?,
        ))
    }

    pub fn peek_i32(&self) -> Result<i32> {
        Ok(i32::from_be_bytes(
            self.peek_bytes(constants::I32_BYTES)?
                .try_into()
                .map_err(|_| Error::UnexpectedEof {
                    expected: constants::I32_BYTES,
                    available: self.remaining(),
                })?,
        ))
    }
}
