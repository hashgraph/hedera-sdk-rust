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

use crate::ffi::error::Error;
use crate::ffi::util::cstr_from_ptr;
use crate::NftId;

/// Parse a Hedera `NftId` from the passed string.
#[no_mangle]
pub unsafe extern "C" fn hedera_nft_id_from_string(
    s: *const c_char,
    token_id_shard: *mut u64,
    token_id_realm: *mut u64,
    token_id_num: *mut u64,
    serial: *mut u64,
) -> Error {
    assert!(!token_id_shard.is_null());
    assert!(!token_id_realm.is_null());
    assert!(!token_id_num.is_null());
    assert!(!serial.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(NftId::from_str(&s));

    unsafe {
        ptr::write(token_id_shard, parsed.token_id.shard);
        ptr::write(token_id_realm, parsed.token_id.realm);
        ptr::write(token_id_num, parsed.token_id.num);
        ptr::write(serial, parsed.serial);
    }

    Error::Ok
}
