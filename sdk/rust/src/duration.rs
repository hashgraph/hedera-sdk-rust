use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use hedera_proto::services;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{Error, FromProtobuf, ToProtobuf};

/// A length of time in seconds.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct Duration {
    /// The number of seconds.
    pub seconds: i64,
}

impl Debug for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for Duration {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.seconds)
    }
}

impl FromProtobuf for Duration {
    type Protobuf = services::Duration;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        Ok(Self {
            seconds: pb.seconds as i64,
        })
    }
}

impl ToProtobuf for Duration {
    type Protobuf = services::Duration;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::Duration {
            seconds: self.seconds,
        }
    }
}

impl FromStr for Duration {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            seconds: s.parse::<i64>().unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use hedera_proto::services;
    use crate::{FromProtobuf, ToProtobuf};
    use crate::duration::Duration;

    const SECONDS: i64 = 123456789;

    #[test]
    fn it_can_convert_to_protobuf() -> anyhow::Result<()> {
        let duration = Duration {
            seconds: SECONDS,
        };

        let duration_protobuf = duration.to_protobuf();

        assert_eq!(duration.seconds, duration_protobuf.seconds);

        Ok(())
    }

    #[test]
    fn it_can_be_created_from_protobuf() -> anyhow::Result<()> {
        let duration_protobuf = services::Duration {
            seconds: SECONDS,
        };

        let duration = Duration::from_protobuf(duration_protobuf).unwrap();

        assert_eq!(duration.seconds, duration_protobuf.seconds);

        Ok(())
    }

    #[test]
    fn it_can_parse_from_string() -> anyhow::Result<()> {
        let timestamp_string = SECONDS.to_string();

        let duration = Duration::from_str(timestamp_string.as_str()).unwrap();

        assert_eq!(duration.seconds, SECONDS);

        Ok(())
    }
}
