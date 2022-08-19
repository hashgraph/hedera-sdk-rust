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
use std::str::FromStr;

use crate::ffi::error::Error;
use crate::ffi::util::cstr_from_ptr;
use crate::EntityId;

/// Parse a Hedera `EntityId` from the passed string.
#[no_mangle]
pub extern "C" fn hedera_entity_id_from_string(
    s: *const c_char,
    id_shard: *mut u64,
    id_realm: *mut u64,
    id_num: *mut u64,
) -> Error {
    assert!(!id_shard.is_null());
    assert!(!id_realm.is_null());
    assert!(!id_num.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(EntityId::from_str(&s));

    unsafe {
        *id_shard = parsed.shard;
        *id_realm = parsed.realm;
        *id_num = parsed.num;
    }

    Error::Ok
}
