use std::os::raw::c_char;
use std::str::FromStr;

use crate::ffi::error::Error;
use crate::ffi::util::cstr_from_ptr;
use crate::{
    AccountAddress,
    AccountAlias,
    AccountId,
    NftId,
    PublicKey,
};

/// Parse a Hedera `NftId` from the passed string.
#[no_mangle]
pub extern "C" fn hedera_nft_id_from_string(
    s: *const c_char,
    token_id_shard: *mut u64,
    token_id_realm: *mut u64,
    token_id_num: *mut u64,
    serial_number: *mut u64,
) -> Error {
    assert!(!token_id_shard.is_null());
    assert!(!token_id_realm.is_null());
    assert!(!token_id_num.is_null());
    assert!(!serial_number.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(NftId::from_str(&s));

    unsafe {
        *token_id_shard = parsed.token_id.shard;
        *token_id_realm = parsed.token_id.shard;
        *token_id_num = parsed.token_id.num;
        *serial_number = parsed.serial_number;
    }

    Error::Ok
}
