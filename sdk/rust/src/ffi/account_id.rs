use std::os::raw::c_char;
use std::str::FromStr;

use crate::ffi::error::Error;
use crate::ffi::util::cstr_from_ptr;
use crate::{AccountAddress, AccountAlias, AccountId, PublicKey};

/// Parse a Hedera `AccountAddress` from the passed string.
#[no_mangle]
pub extern "C" fn hedera_account_address_from_string(
    s: *const c_char,
    id_shard: *mut u64,
    id_realm: *mut u64,
    id_num: *mut u64,
    id_alias: *mut *mut PublicKey,
) -> Error {
    assert!(!id_shard.is_null());
    assert!(!id_realm.is_null());
    assert!(!id_num.is_null());
    assert!(!id_alias.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(AccountAddress::from_str(&s));

    match parsed {
        AccountAddress::AccountId(parsed) => unsafe {
            *id_shard = parsed.shard;
            *id_realm = parsed.realm;
            *id_num = parsed.num;
        },

        AccountAddress::AccountAlias(parsed) => unsafe {
            *id_shard = parsed.shard;
            *id_realm = parsed.realm;
            *id_alias = Box::into_raw(Box::new(parsed.alias));
        },
    }

    Error::Ok
}

/// Parse a Hedera `AccountAlias` from the passed string.
#[no_mangle]
pub extern "C" fn hedera_account_alias_from_string(
    s: *const c_char,
    id_shard: *mut u64,
    id_realm: *mut u64,
    id_alias: *mut *mut PublicKey,
) -> Error {
    assert!(!id_shard.is_null());
    assert!(!id_realm.is_null());
    assert!(!id_alias.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(AccountAlias::from_str(&s));

    unsafe {
        *id_shard = parsed.shard;
        *id_realm = parsed.realm;
        *id_alias = Box::into_raw(Box::new(parsed.alias));
    }

    Error::Ok
}
