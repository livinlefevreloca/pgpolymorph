//! Big-endian primitive reads from byte slices.

/// Read a big-endian `i16` from exactly two bytes.
pub(crate) fn i16(bytes: &[u8]) -> Option<i16> {
    Some(i16::from_be_bytes(bytes.try_into().ok()?))
}

/// Read a big-endian `i32` from exactly four bytes.
pub(crate) fn i32(bytes: &[u8]) -> Option<i32> {
    Some(i32::from_be_bytes(bytes.try_into().ok()?))
}

/// Read a big-endian `i64` from exactly eight bytes.
pub(crate) fn i64(bytes: &[u8]) -> Option<i64> {
    Some(i64::from_be_bytes(bytes.try_into().ok()?))
}

/// Read a big-endian `u32` from exactly four bytes.
pub(crate) fn u32(bytes: &[u8]) -> Option<u32> {
    Some(u32::from_be_bytes(bytes.try_into().ok()?))
}

/// Read a big-endian `f32` from exactly four bytes.
pub(crate) fn f32(bytes: &[u8]) -> Option<f32> {
    Some(f32::from_be_bytes(bytes.try_into().ok()?))
}

/// Read a big-endian `f64` from exactly eight bytes.
pub(crate) fn f64(bytes: &[u8]) -> Option<f64> {
    Some(f64::from_be_bytes(bytes.try_into().ok()?))
}

/// Read a fixed-size array from the front of `bytes`.
pub(crate) fn fixed<const N: usize>(bytes: &[u8]) -> Option<[u8; N]> {
    bytes.try_into().ok()
}
