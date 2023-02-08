/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
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

use libc::size_t;
use sha3::Digest;

use crate::ffi::util;

unsafe fn digest<D: Digest>(
    bytes: *const u8,
    bytes_size: size_t,
    result_out: *mut *mut u8,
) -> size_t {
    assert!(!bytes.is_null());
    assert!(!result_out.is_null());

    let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_size) };

    let bytes = D::digest(bytes).to_vec();
    unsafe { util::make_bytes(bytes, result_out) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_crypto_sha2_sha384_digest(
    bytes: *const u8,
    bytes_size: size_t,
    result_out: *mut *mut u8,
) -> size_t {
    unsafe { digest::<sha2::Sha384>(bytes, bytes_size, result_out) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_crypto_sha3_keccak256_digest(
    bytes: *const u8,
    bytes_size: size_t,
    result_out: *mut *mut u8,
) -> size_t {
    unsafe { digest::<sha3::Keccak256>(bytes, bytes_size, result_out) }
}
