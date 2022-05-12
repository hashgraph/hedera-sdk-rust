use std::fmt::{self, Formatter};

use serde::de::{self, Visitor};
use serde::{Deserializer, Serializer};
use time::Duration;

#[allow(dead_code)]
pub fn serialize<S>(duration: &Duration, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_i64(duration.whole_seconds())
}

#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    struct DurationSecondsVisitor;

    impl<'de> Visitor<'de> for DurationSecondsVisitor {
        type Value = Duration;

        fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
            formatter.write_str("an integer between -2^63 and 2^63")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Duration::seconds(value))
        }
    }

    deserializer.deserialize_i64(DurationSecondsVisitor)
}
