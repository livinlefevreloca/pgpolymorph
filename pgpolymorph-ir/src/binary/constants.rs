//! COPY binary format constants and PostgreSQL binary payload sizes.
//!
//! All multi-byte scalars are big-endian in PostgreSQL binary format.

pub const COPY_MAGIC: &[u8; 11] = b"PGCOPY\n\xFF\r\n\x00";
pub(crate) const FOOTER_SENTINEL: i16 = -1;

/// Days between PostgreSQL date epoch (2000-01-01) and Unix epoch (1970-01-01).
pub(crate) const PG_DATE_EPOCH_OFFSET_DAYS: i32 = 10_957;

/// Microseconds between PostgreSQL timestamp epoch and Unix epoch.
pub(crate) const PG_TIMESTAMP_EPOCH_OFFSET_US: i64 = 946_684_800_000_000;

/// Width of a `uint8` / `bool` payload byte.
pub(crate) const U8_BYTES: usize = 1;

/// Width of an `int16` field (big-endian).
pub(crate) const I16_BYTES: usize = 2;

/// Width of an `int32` field (big-endian).
pub(crate) const I32_BYTES: usize = 4;

/// Width of an `int64` field (big-endian).
pub(crate) const I64_BYTES: usize = 8;

/// Width of an IEEE754 single-precision float payload.
pub(crate) const F32_BYTES: usize = 4;

/// Width of an IEEE754 double-precision float payload.
pub(crate) const F64_BYTES: usize = 8;

/// Width of the per-tuple `field_count` prefix and footer sentinel.
pub(crate) const COPY_FIELD_COUNT_BYTES: usize = I16_BYTES;

/// Width of the per-field `int32` length prefix in a field envelope.
pub(crate) const COPY_FIELD_LEN_BYTES: usize = I32_BYTES;

/// Field envelope length value meaning SQL NULL (no payload follows).
pub(crate) const COPY_FIELD_NULL: i32 = -1;

pub(crate) const BOOL_PAYLOAD_BYTES: usize = U8_BYTES;
pub(crate) const CHAR_PAYLOAD_BYTES: usize = I16_BYTES;
pub(crate) const INT2_PAYLOAD_BYTES: usize = I16_BYTES;
pub(crate) const INT4_PAYLOAD_BYTES: usize = I32_BYTES;
pub(crate) const INT8_PAYLOAD_BYTES: usize = I64_BYTES;
pub(crate) const FLOAT4_PAYLOAD_BYTES: usize = F32_BYTES;
pub(crate) const FLOAT8_PAYLOAD_BYTES: usize = F64_BYTES;
pub(crate) const DATE_PAYLOAD_BYTES: usize = I32_BYTES;
pub(crate) const TIME_PAYLOAD_BYTES: usize = I64_BYTES;
pub(crate) const TIMESTAMP_PAYLOAD_BYTES: usize = I64_BYTES;
pub(crate) const TIMESTAMPTZ_PAYLOAD_BYTES: usize = I64_BYTES;
pub(crate) const TIMETZ_PAYLOAD_BYTES: usize = I64_BYTES + I32_BYTES;
pub(crate) const INTERVAL_PAYLOAD_BYTES: usize = I64_BYTES + I32_BYTES + I32_BYTES;
pub(crate) const OID_PAYLOAD_BYTES: usize = I32_BYTES;
pub(crate) const MONEY_PAYLOAD_BYTES: usize = I64_BYTES;
pub(crate) const UUID_PAYLOAD_BYTES: usize = 16;

/// Leading format-version byte in a `jsonb` payload.
pub(crate) const JSONB_VERSION_BYTES: usize = U8_BYTES;

/// Header of a `numeric` payload: ndigits, weight, sign, dscale.
pub(crate) const NUMERIC_HEADER_BYTES: usize = 4 * I16_BYTES;

/// Width of each base-10000 digit limb in a `numeric` payload.
pub(crate) const NUMERIC_DIGIT_BYTES: usize = I16_BYTES;

/// Optional leading `int32` on multi-dimensional array payloads.
pub(crate) const ARRAY_TOTAL_LEN_BYTES: usize = I32_BYTES;

/// Minimum valid dimension count.
pub(crate) const ARRAY_MIN_NDIM: i32 = 1;

/// `has_nulls` flag values in the array header.
pub(crate) const ARRAY_HAS_NULLS_FALSE: i32 = 0;
pub(crate) const ARRAY_HAS_NULLS_TRUE: i32 = 1;

/// Dimension count above which payloads include an [`ARRAY_TOTAL_LEN_BYTES`] prefix.
pub(crate) const ARRAY_MULTI_DIM_NDIM: usize = 2;
