//! Cursor over a byte slice — all binary parsing goes through here.

use crate::binary::big_endian_bytes::BigEndianBytes;
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

    fn read_bytes(&mut self, n: usize) -> Result<&'a [u8]> {
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

    pub(crate) fn peek_bytes(&self, n: usize) -> Result<&'a [u8]> {
        if self.remaining() < n {
            return Err(Error::UnexpectedEof {
                expected: n,
                available: self.remaining(),
            });
        }
        Ok(&self.data[self.pos..self.pos + n])
    }

    pub(crate) fn read_fixed<const N: usize>(&mut self) -> Result<[u8; N]> {
        self.read_bytes(N)?
            .try_into()
            .map_err(|_| Error::UnexpectedEof {
                expected: N,
                available: N.saturating_sub(1),
            })
    }

    pub(crate) fn read_be<T: BigEndianBytes>(&mut self) -> Result<T> {
        T::read_be(self)
    }

    pub(crate) fn read_be_exact<T: BigEndianBytes>(&mut self, len: usize) -> Result<T> {
        self.ensure_exact_remaining(len)?;
        T::read_be(self)
    }

    pub(crate) fn peek_be<T: BigEndianBytes>(&self) -> Result<T> {
        T::peek_be(self)
    }

    fn ensure_exact_remaining(&self, len: usize) -> Result<()> {
        if self.remaining() != len {
            return Err(Error::UnexpectedEof {
                expected: len,
                available: self.remaining(),
            });
        }
        Ok(())
    }

    pub(crate) fn read_utf8_remaining(&mut self) -> Result<String> {
        std::str::from_utf8(self.read_remaining()?)
            .map(|s| s.to_string())
            .map_err(|_| Error::UnexpectedEof {
                expected: 1,
                available: 0,
            })
    }

    fn project_view(&self, end_offset: usize) -> Result<BufferView<'a>> {
        if end_offset < self.pos {
            return Err(Error::UnexpectedEof {
                expected: self.pos - end_offset,
                available: 0,
            });
        }
        if end_offset > self.data.len() {
            return Err(Error::UnexpectedEof {
                expected: end_offset - self.pos,
                available: self.remaining(),
            });
        }
        Ok(BufferView::new(&self.data[self.pos..end_offset]))
    }

    /// Advance by `n` bytes and return a projected view over that span.
    pub(crate) fn read_n_and_project_view(&mut self, n: usize) -> Result<BufferView<'a>> {
        let end = self.pos.checked_add(n).ok_or(Error::MalformedInput {
            reason: "field span length overflow",
        })?;
        let view = self.project_view(end)?;
        self.pos = end;
        Ok(view)
    }

    /// Return all bytes from the current position to the end.
    pub(crate) fn read_remaining(&mut self) -> Result<&'a [u8]> {
        let rest = &self.data[self.pos..];
        self.pos = self.data.len();
        Ok(rest)
    }
}

impl BufferView<'static> {
    pub fn empty() -> Self {
        Self { data: &[], pos: 0 }
    }
}
