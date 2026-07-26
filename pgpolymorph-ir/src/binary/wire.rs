//! PostgreSQL binary wire sizes and COPY envelope constants.
//!
//! All multi-byte scalars are big-endian on the wire.

/// Width of a `uint8` / `bool` payload byte.
pub const U8: usize = 1;

/// Width of an `int16` field (big-endian).
pub const I16: usize = 2;

/// Width of an `int32` field (big-endian).
pub const I32: usize = 4;

/// Width of an `int64` field (big-endian).
pub const I64: usize = 8;

/// Width of an IEEE754 single-precision float on the wire.
pub const F32: usize = I32;

/// Width of an IEEE754 double-precision float on the wire.
pub const F64: usize = I64;

// --- COPY file envelope ---

/// Width of the per-tuple `field_count` prefix and footer sentinel.
pub const COPY_FIELD_COUNT: usize = I16;

/// Width of the per-field `int32` length prefix in a field envelope.
pub const COPY_FIELD_LEN: usize = I32;

/// Field envelope length value meaning SQL NULL (no payload follows).
pub const COPY_FIELD_NULL: i32 = -1;

// --- Scalar type payload sizes (PostgreSQL binary send format) ---

pub const BOOL: usize = U8;
pub const CHAR: usize = I16;
pub const INT2: usize = I16;
pub const INT4: usize = I32;
pub const INT8: usize = I64;
pub const FLOAT4: usize = F32;
pub const FLOAT8: usize = F64;
pub const DATE: usize = I32;
pub const TIME: usize = I64;
pub const TIMESTAMP: usize = I64;
pub const TIMESTAMPTZ: usize = I64;
pub const TIMETZ: usize = I64 + I32;
pub const INTERVAL: usize = I64 + I32 + I32;
pub const OID: usize = I32;
pub const MONEY: usize = I64;
pub const UUID: usize = 16;

/// Leading format-version byte in a `jsonb` payload.
pub const JSONB_VERSION: usize = U8;

// --- `numeric` ---

/// Header of a `numeric` payload: ndigits, weight, sign, dscale.
pub const NUMERIC_HEADER: usize = 4 * I16;

/// Width of each base-10000 digit limb in a `numeric` payload.
pub const NUMERIC_DIGIT: usize = I16;

// --- Array wire layout ---

/// Optional leading `int32` on multi-dimensional array payloads.
pub const ARRAY_TOTAL_LEN: usize = I32;

/// Minimum valid dimension count.
pub const ARRAY_MIN_NDIM: i32 = 1;

/// `has_nulls` flag values in the array header.
pub const ARRAY_HAS_NULLS_FALSE: i32 = 0;
pub const ARRAY_HAS_NULLS_TRUE: i32 = 1;

/// Dimension count above which payloads include an [`ARRAY_TOTAL_LEN`] prefix.
pub const ARRAY_MULTI_DIM_NDIM: usize = 2;
