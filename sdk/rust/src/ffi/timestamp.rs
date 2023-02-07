#[derive(Copy, Clone)]
#[repr(C)]
pub struct Timestamp {
    secs: u64,
    nanos: u32,
}

use time::OffsetDateTime;

impl From<OffsetDateTime> for Timestamp {
    fn from(it: OffsetDateTime) -> Self {
        Self { secs: u64::try_from(it.unix_timestamp()).unwrap(), nanos: it.nanosecond() }
    }
}
