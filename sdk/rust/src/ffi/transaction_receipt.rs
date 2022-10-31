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

use std::ffi::c_char;

use super::error::Error;
use crate::ffi::util;
use crate::TransactionReceipt;

/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `s` must only be freed with `hedera_string_free`,
///   notably this means it must not be freed with `free`.
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_receipt_from_bytes(
    bytes: *const u8,
    bytes_size: libc::size_t,
    s: *mut *mut c_char,
) -> Error {
    unsafe { util::json_from_bytes(bytes, bytes_size, s, TransactionReceipt::from_bytes) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_receipt_to_bytes(
    s: *const c_char,
    buf: *mut *mut u8,
    buf_size: *mut libc::size_t,
) -> Error {
    unsafe { util::json_to_bytes(s, buf, buf_size, TransactionReceipt::to_bytes) }
}
