use std::fmt::{self, Debug, Display, Formatter};
use std::str::FromStr;

use hedera_proto::services;
use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::{Error, FromProtobuf, ToProtobuf};

/// A length of time in numerator.
#[derive(SerializeDisplay, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Copy)]
#[repr(C)]
pub struct Fraction {
    /// The number of numerator.
    pub numerator: i64,

    pub denominator: i64,
}

impl Debug for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl Display for Fraction {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

impl FromProtobuf for Fraction {
    type Protobuf = services::Fraction;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        Ok(Self {
            numerator: pb.numerator as i64,
            denominator: pb.denominator as i64,
        })
    }
}

impl ToProtobuf for Fraction {
    type Protobuf = services::Fraction;

    fn to_protobuf(&self) -> Self::Protobuf {
        services::Fraction {
            numerator: self.numerator,
            denominator: self.denominator,
        }
    }
}

impl FromStr for Fraction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split("/").collect();

        if parts.len() == 2 {
            Ok(Self {
                numerator: parts[0].parse::<i64>().unwrap_or_default(),
                denominator: parts[1].parse::<i64>().unwrap_or_default(),
            })
        } else {
            Err(Error::basic_parse("expecting <numerator>/<denominator> (ex: 37/42)"))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use hedera_proto::services;
    use crate::{FromProtobuf, ToProtobuf};
    use crate::fraction::Fraction;

    const NUMERATOR: i64 = 37;
    const DENOMINATOR: i64 = 42;

    #[test]
    fn it_can_convert_to_protobuf() -> anyhow::Result<()> {
        let fraction = Fraction {
            numerator: NUMERATOR,
            denominator: DENOMINATOR,
        };

        let fraction_proto = fraction.to_protobuf();

        assert_eq!(fraction.numerator, fraction_proto.numerator);
        assert_eq!(fraction.denominator, fraction_proto.denominator);

        Ok(())
    }

    #[test]
    fn it_can_be_created_from_protobuf() -> anyhow::Result<()> {
        let fraction_proto = services::Fraction {
            numerator: NUMERATOR,
            denominator: DENOMINATOR,
        };

        let fraction = Fraction::from_protobuf(fraction_proto).unwrap();

        assert_eq!(fraction.numerator, fraction_proto.numerator);
        assert_eq!(fraction.denominator, fraction_proto.denominator);

        Ok(())
    }

    #[test]
    fn it_can_parse_from_string() -> anyhow::Result<()> {
        let fraction_string = format!("{}/{}", NUMERATOR, DENOMINATOR);

        let duration = Fraction::from_str(fraction_string.as_str()).unwrap();

        assert_eq!(duration.numerator, NUMERATOR);
        assert_eq!(duration.denominator, DENOMINATOR);

        Ok(())
    }
}
