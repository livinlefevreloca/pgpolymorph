/// Calendar date as days since the Unix epoch (1970-01-01).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgDate {
    pub days: i32,
}

impl PgDate {
    pub const fn new(days: i32) -> Self {
        Self { days }
    }
}

/// Time of day as microseconds since midnight (no date component).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgTime {
    pub micros: i64,
}

impl PgTime {
    pub const fn new(micros: i64) -> Self {
        Self { micros }
    }
}

/// Timestamp without time zone, as microseconds since the Unix epoch (UTC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgTimestamp {
    pub micros: i64,
}

impl PgTimestamp {
    pub const fn new(micros: i64) -> Self {
        Self { micros }
    }
}

/// Timestamp with time zone, as microseconds since the Unix epoch (UTC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgTimestamptz {
    pub micros: i64,
}

impl PgTimestamptz {
    pub const fn new(micros: i64) -> Self {
        Self { micros }
    }
}

/// Time of day with time-zone offset from UTC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PgTimetz {
    pub micros: i64,
    pub tz_offset_secs: i32,
}

impl PgTimetz {
    pub const fn new(micros: i64, tz_offset_secs: i32) -> Self {
        Self {
            micros,
            tz_offset_secs,
        }
    }
}
