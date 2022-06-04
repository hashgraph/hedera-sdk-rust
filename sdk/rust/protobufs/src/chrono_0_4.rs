use chrono_0_4::{DateTime, Duration, TimeZone, Utc};

impl From<super::services::Duration> for Duration {
    fn from(pb: super::services::Duration) -> Self {
        Self::seconds(pb.seconds)
    }
}

impl From<Duration> for super::services::Duration {
    fn from(duration: Duration) -> Self {
        Self { seconds: duration.num_seconds() }
    }
}

impl From<super::services::TimestampSeconds> for DateTime<Utc> {
    fn from(pb: super::services::TimestampSeconds) -> Self {
        Utc.timestamp(pb.seconds, 0)
    }
}

impl From<DateTime<Utc>> for super::services::TimestampSeconds {
    fn from(dt: DateTime<Utc>) -> Self {
        Self { seconds: dt.timestamp() }
    }
}

impl From<super::services::Timestamp> for DateTime<Utc> {
    fn from(pb: super::services::Timestamp) -> Self {
        Utc.timestamp(pb.seconds, 0) + Duration::nanoseconds(pb.nanos.into())
    }
}

impl From<DateTime<Utc>> for super::services::Timestamp {
    fn from(dt: DateTime<Utc>) -> Self {
        Self { seconds: dt.timestamp(), nanos: dt.timestamp_subsec_nanos() as i32 }
    }
}
