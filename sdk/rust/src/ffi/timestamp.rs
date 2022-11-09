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

impl From<Timestamp> for hedera_proto::services::Timestamp {
    fn from(dt: Timestamp) -> Self {
        Self { seconds: dt.secs as i64, nanos: dt.nanos as i32 }
    }
}
