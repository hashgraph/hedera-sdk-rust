mod error;
mod key;
mod mnemonic;

pub(crate) use error::{
    Error,
    Result,
};
pub(crate) use key::{
    PrivateKey,
    PublicKey,
};
pub(crate) use mnemonic::Mnemonic;

mod ffi;
