use std::ffi::{
    c_char,
    CString,
};

use super::error::Error;
use super::util;
use crate::{
    Mnemonic,
    PrivateKey,
};

/// Parse a `Mnemonic` from a string.
///
/// # Safety
/// - `s` must be valid for reads up until and including the first NUL (`'\0'`) byte.
/// - `mnemonic` must be valid for writes according to the [*Rust* pointer rules]
/// - if this method returns anything other than [`Error::Ok`],
///   then the contents of `mnemonic` are undefined and must not be used or inspected.
/// - `mnemonic` must only be freed via [`hedera_mnemonic_free`].
///   Notably this means that it *must not* be freed with `free`.
///
/// # Errors
/// - [`Error::MnemonicParse`] if the mnemonic has an invalid length.
/// - [`Error::MnemonicParse`] if the mnemonic uses invalid words.
/// - [`Error::MnemonicParse`] if the mnemonic has an invalid checksum.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_mnemonic_from_string(
    s: *const c_char,
    mnemonic: *mut *mut Mnemonic,
) -> Error {
    let s = unsafe { util::cstr_from_ptr(s) };

    let parsed: Mnemonic = ffi_try!(s.parse());
    let parsed = Box::into_raw(Box::new(parsed));
    unsafe {
        mnemonic.write(parsed);
    }

    Error::Ok
}

/// Generate a new 24 word mnemonic.
///
/// # Safety
/// This function is safe. However, there are invariants that must be upheld on the result.
///
/// - The returned mnemonic must only be freed via [`hedera_mnemonic_free`].
///   Notably this means that it *must not* be freed with `free`.
#[no_mangle]
pub extern "C" fn hedera_mnemonic_generate_24() -> *mut Mnemonic {
    let mnemonic = Mnemonic::generate_24();

    Box::into_raw(Box::new(mnemonic))
}

/// Generate a new 12 word mnemonic.
///
/// # Safety
/// This function is safe. However, there are invariants that must be upheld on the result.
///
/// - The returned mnemonic must only be freed via [`hedera_mnemonic_free`].
///   Notably this means that it *must not* be freed with `free`.
#[no_mangle]
pub extern "C" fn hedera_mnemonic_generate_12() -> *mut Mnemonic {
    let mnemonic = Mnemonic::generate_12();

    Box::into_raw(Box::new(mnemonic))
}

/// Returns `true` if `mnemonic` is a legacy mnemonic.
///
/// # Safety
/// - `mnemonic` must be valid for reads according to the [*Rust* pointer rules].
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_mnemonic_is_legacy(mnemonic: *mut Mnemonic) -> bool {
    let mnemonic = unsafe { mnemonic.as_ref() }.unwrap();
    mnemonic.is_legacy()
}

/// Recover a [`PrivateKey`] from `mnemonic`.
///
/// # Safety
/// - `mnemonic` must be valid for reads according to the [*Rust* pointer rules].
/// - `passphrase` must be valid for reads up until and including the first NUL (`'\0'`) byte.
/// - `private_key` must be valid for writes according to the [*Rust* pointer rules].
/// - if this method returns anything other than [`Error::Ok`],
///   then the contents of `private_key` are undefined and must not be used or inspected.
/// - `private_key` must only be freed via `hedera_private_key_free`.
///   Notably, this means that it *must not* be freed with `free`.
///
/// # Errors
/// - [`Error::MnemonicEntropy`] if this is a legacy private key, and the passphrase isn't empty.
/// - [`Error::MnemonicEntropy`] if this is a legacy private key,
///   and the `Mnemonic`'s checksum doesn't match up with the computed one.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_mnemonic_to_private_key(
    mnemonic: *mut Mnemonic,
    passphrase: *const c_char,
    private_key: *mut *mut PrivateKey,
) -> Error {
    let mnemonic = unsafe { mnemonic.as_ref() }.unwrap();
    let passphrase = unsafe { util::cstr_from_ptr(passphrase) };

    let sk = ffi_try!(mnemonic.to_private_key(&passphrase));
    let sk = Box::into_raw(Box::new(sk));

    unsafe {
        private_key.write(sk);
    }

    Error::Ok
}

/// Recover a [`PrivateKey`] from `mnemonic`.
///
/// # Safety
/// - `mnemonic` must be valid for reads according to the [*Rust* pointer rules].
/// - `private_key` must be valid for writes according to the [*Rust* pointer rules].
/// - if this method returns anything other than [`Error::Ok`],
///   then the contents of `private_key` are undefined and must not be used or inspected.
/// - `private_key` must only be freed via `hedera_private_key_free`.
///   Notably, this means that it *must not* be freed with `free`.
///
/// # Errors
/// - [`Error::MnemonicEntropy`] if the computed checksum doesn't match the actual checksum.
/// - [`Error::MnemonicEntropy`] if this is a v2 legacy mnemonic and doesn't have `24` words.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_mnemonic_to_legacy_private_key(
    mnemonic: *mut Mnemonic,
    private_key: *mut *mut PrivateKey,
) -> Error {
    let mnemonic = unsafe { mnemonic.as_ref() }.unwrap();

    let sk = ffi_try!(mnemonic.to_legacy_private_key());
    let sk = Box::into_raw(Box::new(sk));

    unsafe {
        private_key.write(sk);
    }

    Error::Ok
}

/// Format `mnemonic` as a string.
///
/// # Safety
/// - `mnemonic` must be valid for reads according to the [*Rust* pointer rules].
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_mnemonic_to_string(mnemonic: *mut Mnemonic) -> *mut c_char {
    let mnemonic = unsafe { mnemonic.as_ref() }.unwrap();

    CString::new(mnemonic.to_string()).unwrap().into_raw()
}

/// Free `mnemonic` and release all resources associated with it.
///
/// # Safety
/// - `mnemonic` must be valid for reads and writes according to the [*Rust* pointer rules].
/// - `mnemonic` must not be used at all after this function is called.
///
/// [*Rust* pointer rules]: https://doc.rust-lang.org/std/ptr/index.html#safety
#[no_mangle]
pub unsafe extern "C" fn hedera_mnemonic_free(mnemonic: *mut Mnemonic) {
    unsafe { drop(Box::from_raw(mnemonic)) }
}
