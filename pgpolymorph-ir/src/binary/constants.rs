pub const COPY_MAGIC: &[u8; 11] = b"PGCOPY\n\xFF\r\n\x00";
pub(crate) const FOOTER_SENTINEL: i16 = -1;

/// Days between PostgreSQL date epoch (2000-01-01) and Unix epoch (1970-01-01).
pub(crate) const PG_DATE_EPOCH_OFFSET_DAYS: i32 = 10_957;

/// Microseconds between PostgreSQL timestamp epoch and Unix epoch.
pub(crate) const PG_TIMESTAMP_EPOCH_OFFSET_US: i64 = 946_684_800_000_000;
