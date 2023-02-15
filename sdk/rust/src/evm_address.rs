use std::fmt;
use std::str::FromStr;

use hex::FromHexError;

use crate::{
    EntityId,
    Error,
};

/// An address as implemented in the Ethereum Virtual Machine.
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(
    feature = "ffi",
    derive(serde_with::SerializeDisplay, serde_with::DeserializeFromStr),
    // the only function using unsafe here is sound for all inputs, and,
    // serialize/deserialize delegate to code that is manually written anyway.
    allow(clippy::unsafe_derive_deserialize)
)]
#[repr(transparent)]
pub struct EvmAddress(pub(crate) [u8; 20]);

impl EvmAddress {
    #[must_use]
    pub(crate) fn from_ref(bytes: &[u8; 20]) -> &Self {
        // safety: `self` is `#[repr(transpart)] over `[u8; 20]`
        unsafe { &*(bytes.as_ptr().cast::<EvmAddress>()) }
    }

    /// Gets the underlying bytes this EVM address is made from.
    #[must_use]
    pub fn to_bytes(self) -> [u8; 20] {
        self.0
    }
}

// potential point of confusion: This type is specifically for the `shard.realm.num` in 20 byte format.
#[derive(Copy, Clone)]
#[repr(transparent)]
pub(crate) struct IdEvmAddress(pub(crate) EvmAddress);

impl IdEvmAddress {
    #[must_use]
    pub(crate) fn from_ref(bytes: &[u8; 20]) -> &Self {
        // safety: `self` is `#[repr(transpart)] over `EvmAddress`, which is repr transparent over `[u8; 20]`.
        unsafe { &*(bytes.as_ptr().cast::<IdEvmAddress>()) }
    }

    #[must_use]
    pub(crate) fn to_bytes(self) -> [u8; 20] {
        self.0.to_bytes()
    }
}

impl From<IdEvmAddress> for EntityId {
    fn from(value: IdEvmAddress) -> Self {
        let value = value.to_bytes();
        // todo: once split_array_ref is stable, all the unwraps and panics will go away.
        let (shard, value) = value.split_at(4);
        let (realm, num) = value.split_at(8);

        Self {
            shard: u64::from(u32::from_be_bytes(shard.try_into().unwrap())),
            realm: u64::from_be_bytes(realm.try_into().unwrap()),
            num: u64::from_be_bytes(num.try_into().unwrap()),
            checksum: None,
        }
    }
}

impl TryFrom<EntityId> for IdEvmAddress {
    type Error = Error;

    fn try_from(value: EntityId) -> Result<Self, Self::Error> {
        // fixme: use the right error type
        let shard = u32::try_from(value.shard).map_err(Error::basic_parse)?.to_be_bytes();
        let realm = value.realm.to_be_bytes();
        let num = value.num.to_be_bytes();

        let mut buf = [0; 20];

        buf[0..][..shard.len()].copy_from_slice(&shard);
        buf[shard.len()..][..realm.len()].copy_from_slice(&realm);
        buf[(shard.len() + realm.len())..][..num.len()].copy_from_slice(&num);

        Ok(Self::from(buf))
    }
}

impl From<[u8; 20]> for IdEvmAddress {
    fn from(value: [u8; 20]) -> Self {
        Self(EvmAddress(value))
    }
}

impl<'a> From<&'a [u8; 20]> for &'a IdEvmAddress {
    fn from(value: &'a [u8; 20]) -> Self {
        IdEvmAddress::from_ref(value)
    }
}

impl<'a> TryFrom<&'a [u8]> for &'a IdEvmAddress {
    type Error = Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        value.try_into().map(IdEvmAddress::from_ref).map_err(|_| error_len(value.len()))
    }
}

impl TryFrom<Vec<u8>> for IdEvmAddress {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        <&IdEvmAddress>::try_from(value.as_slice()).copied()
    }
}

impl FromStr for IdEvmAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = [0; 20];

        // note: *optional* 0x prefix.
        let address = s.strip_prefix("0x").unwrap_or(s);

        hex::decode_to_slice(address, &mut buf).map(|()| Self(EvmAddress(buf))).map_err(|err| {
            match err {
                FromHexError::InvalidStringLength => error_len(address.len() / 2),
                err => Error::basic_parse(err),
            }
        })
    }
}

fn error_len(bytes: usize) -> crate::Error {
    Error::basic_parse(format!(
        "expected 20 byte (40 character) evm address, got: `{}` bytes",
        bytes
    ))
}

impl fmt::Debug for IdEvmAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self}\"")
    }
}

impl fmt::Display for IdEvmAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:x}", self.0)
    }
}

impl From<[u8; 20]> for EvmAddress {
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

impl<'a> From<&'a [u8; 20]> for &'a EvmAddress {
    fn from(value: &'a [u8; 20]) -> Self {
        EvmAddress::from_ref(value)
    }
}

impl<'a> TryFrom<&'a [u8]> for &'a EvmAddress {
    type Error = Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        value.try_into().map(EvmAddress::from_ref).map_err(|_| error_len(value.len()))
    }
}

impl TryFrom<Vec<u8>> for EvmAddress {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        <&EvmAddress>::try_from(value.as_slice()).copied()
    }
}

// Note: *requires* 0x prefix.
impl FromStr for EvmAddress {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut buf = [0; 20];

        let address = s
            .strip_prefix("0x")
            .ok_or_else(|| Error::basic_parse("expected `0x` prefix in evm address"))?;

        hex::decode_to_slice(address, &mut buf).map(|()| Self(buf)).map_err(|err| match err {
            FromHexError::InvalidStringLength => error_len(address.len() / 2),
            err => Error::basic_parse(err),
        })
    }
}

impl fmt::Debug for EvmAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\"{self:#x}\"")
    }
}

impl fmt::Display for EvmAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:#x}")
    }
}

impl fmt::LowerHex for EvmAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            f.write_str("0x")?;
        }

        let mut output = [0; 40];

        // panic: would either never panic or always panic, it never panics.
        hex::encode_to_slice(self.0, &mut output).unwrap();
        // should never fail. But `unsafe` here when we *aren't* in that crate would be... not great.
        let output = std::str::from_utf8_mut(&mut output).unwrap();
        f.write_str(output)
    }
}
