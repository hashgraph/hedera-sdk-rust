/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use std::os::raw::c_char;
use std::ptr;
use std::str::FromStr;

use libc::size_t;

use crate::ffi::error::Error;
use crate::ffi::util::cstr_from_ptr;
use crate::{
    AccountId,
    PublicKey,
};

// sr: why clone when you could just not.
struct RefAccountId<'a> {
    shard: u64,
    realm: u64,
    num: u64,
    alias: Option<&'a PublicKey>,
}

impl<'a> RefAccountId<'a> {
    fn into_bytes(self) -> Vec<u8> {
        use hedera_proto::services;
        use prost::Message;

        services::AccountId {
            realm_num: self.realm as i64,
            shard_num: self.shard as i64,
            account: Some(match self.alias {
                None => services::account_id::Account::AccountNum(self.num as i64),
                Some(alias) => services::account_id::Account::Alias(alias.to_bytes_raw()),
            }),
        }
        .encode_to_vec()
    }
}

/// Parse a Hedera `AccountId` from the passed string.
#[no_mangle]
pub unsafe extern "C" fn hedera_account_id_from_string(
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
    let parsed = ffi_try!(AccountId::from_str(&s));

    unsafe {
        ptr::write(id_shard, parsed.shard);
        ptr::write(id_realm, parsed.realm);
        ptr::write(id_num, parsed.num);

        if let Some(alias) = parsed.alias {
            ptr::write(id_alias, Box::into_raw(Box::new(alias)));
        }
    }

    Error::Ok
}

/// Parse a Hedera `AccountId` from the passed bytes.
#[no_mangle]
pub unsafe extern "C" fn hedera_account_id_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    id_shard: *mut u64,
    id_realm: *mut u64,
    id_num: *mut u64,
    id_alias: *mut *mut PublicKey,
) -> Error {
    assert!(!bytes.is_null());
    assert!(!id_shard.is_null());
    assert!(!id_realm.is_null());
    assert!(!id_num.is_null());
    assert!(!id_alias.is_null());

    let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_size) };

    let parsed = ffi_try!(AccountId::from_bytes(&bytes));

    unsafe {
        ptr::write(id_shard, parsed.shard);
        ptr::write(id_realm, parsed.realm);
        ptr::write(id_num, parsed.num);

        if let Some(alias) = parsed.alias {
            ptr::write(id_alias, Box::into_raw(Box::new(alias)));
        }
    }

    Error::Ok
}

/// Serialize the passed `AccountId` as bytes
///
/// # Safety
/// - `id_alias` must either be null or point to a valid public key.
/// - `buf` must be valid for writes.
/// - `buf` must only be freed with `hedera_bytes_free`, notably this means that it must not be freed with `free`.
#[no_mangle]
pub unsafe extern "C" fn hedera_account_id_to_bytes(
    id_shard: u64,
    id_realm: u64,
    id_num: u64,
    id_alias: *const PublicKey,
    buf: *mut *mut u8,
) -> size_t {
    // safety: `id_alias` must either be null or point to a valid public key.
    let id_alias = unsafe { id_alias.as_ref() };

    let bytes = RefAccountId { shard: id_shard, realm: id_realm, num: id_num, alias: id_alias }
        .into_bytes()
        .into_boxed_slice();

    let bytes = Box::leak(bytes);
    let len = bytes.len();
    let bytes = bytes.as_mut_ptr();

    // safety: invariants promise that `buf` must be valid for writes.
    unsafe {
        ptr::write(buf, bytes);
    }

    len
}
