use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use hedera_proto::services;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{Error, FromProtobuf, ToProtobuf};

/// An exact date and time. This is the same data structure as the protobuf Timestamp.proto (see the
/// comments in https://github.com/google/protobuf/blob/master/src/google/protobuf/timestamp.proto)
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct Timestamp {
    /// Number of complete seconds since the start of the epoch
    pub seconds: i64,

    /// Number of nanoseconds since the start of the last second
    pub nanos: u32,
}

impl Debug for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for Timestamp {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.seconds, self.nanos)
    }
}

impl FromProtobuf for Timestamp {
    type Protobuf = services::Timestamp;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        Ok(Self {
            seconds: pb.seconds as i64,
            nanos: pb.nanos as u32,
        })
    }
}

impl ToProtobuf for Timestamp {
    type Protobuf = services::Timestamp;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::Timestamp {
            seconds: self.seconds,
            nanos: self.nanos as i32,
        }
    }
}

impl FromStr for Timestamp {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split(".").collect();
        Ok(Self {
            seconds: split[0].parse::<i64>().unwrap_or_default(),
            nanos: split[1].parse::<u32>().unwrap_or_default(),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use hedera_proto::services;
    use crate::{FromProtobuf, ToProtobuf};
    use crate::timestamp::Timestamp;

    const SECONDS: i64 = 123456789;
    const NANOS: u32 = 987654321;

    #[test]
    fn it_can_convert_to_protobuf() -> anyhow::Result<()> {
        let timestamp = Timestamp {
            seconds: SECONDS,
            nanos: NANOS,
        };

        let timestamp_protobuf = timestamp.to_protobuf();

        assert_eq!(timestamp.seconds, timestamp_protobuf.seconds);
        assert_eq!(timestamp.nanos as i32, timestamp_protobuf.nanos);

        Ok(())
    }

    #[test]
    fn it_can_be_created_from_protobuf() -> anyhow::Result<()> {
        let timestamp_protobuf = services::Timestamp {
            seconds: SECONDS,
            nanos: NANOS as i32,
        };

        let timestamp = Timestamp::from_protobuf(timestamp_protobuf).unwrap();

        assert_eq!(timestamp.seconds, timestamp_protobuf.seconds);
        assert_eq!(timestamp.nanos as i32, timestamp_protobuf.nanos);

        Ok(())
    }

    #[test]
    fn it_can_parse_from_string() -> anyhow::Result<()> {
        let timestamp_string = format!("{}.{}", SECONDS, NANOS);

        let timestamp = Timestamp::from_str(timestamp_string.as_str()).unwrap();

        assert_eq!(timestamp.seconds, SECONDS);
        assert_eq!(timestamp.nanos, NANOS);

        Ok(())
    }
}
