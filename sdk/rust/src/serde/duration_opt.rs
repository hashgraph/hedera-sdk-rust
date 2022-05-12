use std::fmt::{self, Formatter};

use serde::de::{self, Visitor};
use serde::{Deserializer, Serializer};
use time::Duration;

#[allow(dead_code)]
pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match duration {
        Some(duration) => serializer.serialize_i64(duration.whole_seconds()),

        None => serializer.serialize_none(),
    }
}

#[allow(dead_code)]
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
where
    D: Deserializer<'de>,
{
    struct DurationSecondsVisitor;

    impl<'de> Visitor<'de> for DurationSecondsVisitor {
        type Value = Option<Duration>;

        fn expecting(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
            formatter.write_str("an integer between -2^63 and 2^63")
        }

        fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(Some(Duration::seconds(value)))
        }

        fn visit_none<E>(self) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(None)
        }
    }

    deserializer.deserialize_i64(DurationSecondsVisitor)
}
