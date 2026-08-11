//! Big-endian scalar reads from a [`BufferView`].

use crate::binary::buffer_view::BufferView;
use crate::binary::constants;
use crate::error::{Error, Result};

pub(crate) trait BigEndianBytes: Copy {
    fn read_be(view: &mut BufferView<'_>) -> Result<Self>;
    fn peek_be(view: &BufferView<'_>) -> Result<Self>;
}

macro_rules! impl_big_endian_bytes {
    ($ty:ty, $size:expr) => {
        impl BigEndianBytes for $ty {
            fn read_be(view: &mut BufferView<'_>) -> Result<Self> {
                Ok(<$ty>::from_be_bytes(view.read_fixed::<$size>()?))
            }

            fn peek_be(view: &BufferView<'_>) -> Result<Self> {
                Ok(<$ty>::from_be_bytes(
                    view.peek_bytes($size)?
                        .try_into()
                        .map_err(|_| Error::UnexpectedEof {
                            expected: $size,
                            available: view.remaining(),
                        })?,
                ))
            }
        }
    };
}

impl BigEndianBytes for u8 {
    fn read_be(view: &mut BufferView<'_>) -> Result<Self> {
        Ok(view.read_fixed::<{ constants::U8_BYTES }>()?[0])
    }

    fn peek_be(view: &BufferView<'_>) -> Result<Self> {
        Ok(view.peek_bytes(constants::U8_BYTES)?[0])
    }
}

impl_big_endian_bytes!(i16, 2);
impl_big_endian_bytes!(i32, 4);
impl_big_endian_bytes!(i64, 8);
impl_big_endian_bytes!(u32, 4);
impl_big_endian_bytes!(f32, 4);
impl_big_endian_bytes!(f64, 8);
