use std::fmt::{Display, Formatter};
use std::str::FromStr;

use derive_more::{
    Add,
    AddAssign,
    Div,
    DivAssign,
    Mul,
    MulAssign,
    Neg,
    Sub,
    SubAssign,
};
use rust_decimal::prelude::*;
use serde_with::{
    DeserializeFromStr,
    SerializeDisplay,
};

use crate::Error;

// TODO: add tests

#[derive(SerializeDisplay, Copy, DeserializeFromStr, Hash, PartialEq, Eq, Clone, Add, Sub, Mul, Div, Neg, AddAssign, SubAssign, MulAssign, DivAssign)]
pub struct Hbar(i64);

pub type Tinybar = i64;

#[repr(i64)]
#[derive(Debug, SerializeDisplay, Copy, DeserializeFromStr, Hash, PartialEq, Eq, Clone)]
pub enum HbarUnit {
    /**
     * The atomic (smallest) unit of hbar, used natively by the Hedera network.
     * <p>
     * It is equivalent to <sup>1</sup>&frasl;<sub>100,000,000</sub> hbar.
     */
    Tinybar = 1,

    /**
     * Equivalent to 100 tinybar or <sup>1</sup>&frasl;<sub>1,000,000</sub> hbar.
     */
    Microbar = 100,

    /**
     * Equivalent to 100,000 tinybar or <sup>1</sup>&frasl;<sub>1,000</sub> hbar.
     */
    Millibar = 100_000,

    /**
     * The base unit of hbar, equivalent to 100 million tinybar.
     */
    Hbar = 100_000_000,

    /**
     * Equivalent to 1 thousand hbar or 100 billion tinybar.
     */
    Kilobar = 1_000 * 100_000_000,

    /**
     * Equivalent to 1 million hbar or 100 trillion tinybar.
     */
    Megabar = 1_000_000 * 100_000_000,

    /**
     * Equivalent to 1 billion hbar or 100 quadillion tinybar.
     * <p>
     * The maximum hbar amount supported by Hedera in any context is ~92 gigabar
     * (2<sup>63</sup> tinybar); use this unit sparingly.
     */
    Gigabar = 1_000_000_000 * 100_000_000,
}

impl HbarUnit {
    pub fn tinybars(self) -> Tinybar {
        self as Tinybar
    }

    pub fn symbol(self) -> &'static str {
        match self {
            HbarUnit::Tinybar => "tℏ",
            HbarUnit::Microbar => "μℏ",
            HbarUnit::Millibar => "mℏ",
            HbarUnit::Hbar => "ℏ",
            HbarUnit::Kilobar => "kℏ",
            HbarUnit::Megabar => "Mℏ",
            HbarUnit::Gigabar => "Gℏ",
        }
    }
}

impl Display for HbarUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

impl FromStr for HbarUnit {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tℏ" => Ok(HbarUnit::Tinybar),
            "μℏ" => Ok(HbarUnit::Microbar),
            "mℏ" => Ok(HbarUnit::Millibar),
            "ℏ" => Ok(HbarUnit::Hbar),
            "kℏ" => Ok(HbarUnit::Kilobar),
            "Mℏ" => Ok(HbarUnit::Megabar),
            "Gℏ" => Ok(HbarUnit::Gigabar),
            _ => Err(Error::basic_parse(format!("Given string \"{}\" was not recognized as an Hbar unit symbol", s)))
        }
    }
}

impl Hbar {
    pub const ZERO: Hbar = Hbar::from_tinybars(0);
    pub const MAX: Hbar = Hbar::from_tinybars(50_000_000_000 * 100_000_000);
    pub const MIN: Hbar = Hbar::from_tinybars(-50_000_000_000 * 100_000_000);


    pub const fn from_tinybars(tinybars: Tinybar) -> Self {
        Hbar(tinybars)
    }

    pub fn from_unit<T>(amount: T, unit: HbarUnit) -> Self where T: Into<Decimal> {
        let unit_tinybars: Decimal = unit.tinybars().into();
        let amount_tinybars = amount.into() * unit_tinybars;
        Hbar::from_tinybars(amount_tinybars.to_i64().unwrap())
    }

    pub fn to_tinybars(self) -> Tinybar {
        self.0
    }

    pub fn to(self, unit: HbarUnit) -> Decimal {
        Decimal::from(self.to_tinybars()) / Decimal::from(unit.tinybars())
    }

    pub fn get_value(self) -> Decimal {
        self.to(HbarUnit::Hbar)
    }

    pub fn negated(self) -> Self {
        -self
    }
}

impl From<Hbar> for Decimal {
    fn from(hbar: Hbar) -> Self {
        hbar.get_value()
    }
}

impl From<Hbar> for Tinybar {
    fn from(hbar: Hbar) -> Self {
        hbar.to_tinybars()
    }
}

impl From<Tinybar> for Hbar {
    fn from(tinybars: Tinybar) -> Self {
        Hbar::from_tinybars(tinybars)
    }
}

impl From<Decimal> for Hbar {
    fn from(hbars: Decimal) -> Self {
        Hbar::from_unit(hbars, HbarUnit::Hbar)
    }
}

impl Display for Hbar {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.to_tinybars() > -10_000 && self.to_tinybars() < 10_000 {
            write!(f, "{} {}", self.to_tinybars(), HbarUnit::Tinybar.symbol())
        } else {
            write!(f, "{} {}", self.to(HbarUnit::Hbar), HbarUnit::Hbar.symbol())
        }
    }
}

impl FromStr for Hbar {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.splitn(2, " ").collect();
        let amount: Decimal = parts[0].parse().map_err(|err| Error::basic_parse(err))?;
        let unit = if parts.len() == 2 {
            HbarUnit::from_str(parts[1])?
        } else {
            HbarUnit::Hbar
        };
        Ok(Hbar::from_unit(amount, unit))
    }
}
