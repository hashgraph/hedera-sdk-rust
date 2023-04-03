mod error;
mod ffi;
mod key;

pub(crate) use error::{
    Error,
    Result,
};
pub(crate) use key::{
    PrivateKey,
    PublicKey,
};
