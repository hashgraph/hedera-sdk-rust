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

use std::ffi::{
    c_char,
    CString,
};
use std::{
    ptr,
    slice,
};

use libc::size_t;

use super::error::Error;
use crate::transaction::AnyTransaction;

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    transaction_out: *mut *mut c_char,
) -> Error {
    assert!(!bytes.is_null());
    assert!(!transaction_out.is_null());

    let bytes = unsafe { slice::from_raw_parts(bytes, bytes_size) };

    let tx = ffi_try!(AnyTransaction::from_bytes(bytes));

    let out = serde_json::to_vec(&tx).unwrap();

    let out = CString::new(out).unwrap().into_raw();

    unsafe {
        ptr::write(transaction_out, out);
    }

    Error::Ok
}
