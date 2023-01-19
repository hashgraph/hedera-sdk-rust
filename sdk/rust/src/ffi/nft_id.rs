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

use std::{
    ptr,
    slice,
};

use libc::size_t;

use crate::ffi::error::Error;
use crate::{
    NftId,
    TokenId,
};

/// Parse a Hedera `NftId` from the passed bytes.
#[no_mangle]
pub unsafe extern "C" fn hedera_nft_id_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    token_id_shard: *mut u64,
    token_id_realm: *mut u64,
    token_id_num: *mut u64,
    serial: *mut u64,
) -> Error {
    assert!(!bytes.is_null());
    assert!(!token_id_shard.is_null());
    assert!(!token_id_realm.is_null());
    assert!(!token_id_num.is_null());
    assert!(!serial.is_null());

    // safety: caller promises that `bytes` is valid for r/w of up to `bytes_size`, which is exactly what `slice::from_raw_parts` wants.
    let bytes = unsafe { slice::from_raw_parts(bytes, bytes_size) };

    let parsed = ffi_try!(NftId::from_bytes(bytes));

    unsafe {
        ptr::write(token_id_shard, parsed.token_id.shard);
        ptr::write(token_id_realm, parsed.token_id.realm);
        ptr::write(token_id_num, parsed.token_id.num);
        ptr::write(serial, parsed.serial);
    }

    Error::Ok
}

/// Serialize the passed `NftId` as bytes
#[no_mangle]
pub unsafe extern "C" fn hedera_nft_id_to_bytes(
    token_id_shard: u64,
    token_id_realm: u64,
    token_id_num: u64,
    serial: u64,
    buf: *mut *mut u8,
) -> size_t {
    // todo: use `as_maybe_uninit_ref` once that's stable.
    assert!(!buf.is_null());

    let nft_id = NftId {
        token_id: TokenId {
            shard: token_id_shard,
            realm: token_id_realm,
            num: token_id_num,
            checksum: None,
        },
        serial,
    };

    let bytes = nft_id.to_bytes().into_boxed_slice();

    let bytes = Box::leak(bytes);
    let len = bytes.len();
    let bytes = bytes.as_mut_ptr();

    // safety: invariants promise that `buf` must be valid for writes.
    unsafe {
        ptr::write(buf, bytes);
    }

    len
}
