use time_0_2::{Duration, OffsetDateTime};

impl From<super::services::Duration> for Duration {
    fn from(pb: super::services::Duration) -> Self {
        Self::seconds(pb.seconds)
    }
}

impl From<Duration> for super::services::Duration {
    fn from(duration: Duration) -> Self {
        Self {
            seconds: duration.whole_seconds(),
        }
    }
}

impl From<super::services::TimestampSeconds> for OffsetDateTime {
    fn from(pb: super::services::TimestampSeconds) -> Self {
        OffsetDateTime::from_unix_timestamp(pb.seconds)
    }
}

impl From<OffsetDateTime> for super::services::TimestampSeconds {
    fn from(dt: OffsetDateTime) -> Self {
        Self {
            seconds: dt.unix_timestamp(),
        }
    }
}

impl From<super::services::Timestamp> for OffsetDateTime {
    fn from(pb: super::services::Timestamp) -> Self {
        OffsetDateTime::from_unix_timestamp(pb.seconds) + Duration::nanoseconds(pb.nanos.into())
    }
}

impl From<OffsetDateTime> for super::services::Timestamp {
    fn from(dt: OffsetDateTime) -> Self {
        let unix = dt - OffsetDateTime::unix_epoch();

        Self {
            seconds: unix.whole_seconds(),
            nanos: unix.subsec_nanoseconds(),
        }
    }
}
