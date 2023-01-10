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

use libc::size_t;

use super::error::Error;
use super::signer::{
    Signer,
    Signers,
};
use super::util::{
    cstr_from_ptr,
    make_bytes2,
};
use crate::transaction::AnyTransaction;

/// Convert the provided transaction to protobuf-encoded bytes.
///
/// # Safety
/// - todo(sr): Missing basically everything
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_to_bytes(
    transaction: *const c_char,
    signers: Signers,
    buf: *mut *mut u8,
    buf_size: *mut size_t,
) -> Error {
    let transaction = unsafe { cstr_from_ptr(transaction) };

    dbg!(&transaction);

    let mut transaction: AnyTransaction =
        ffi_try!(serde_json::from_str(&transaction).map_err(crate::Error::request_parse));

    dbg!(&transaction);

    let signers_2: Vec<_> = signers.as_slice().iter().map(Signer::to_csigner).collect();

    drop(signers);
    let signers = signers_2;

    for signer in signers {
        transaction.sign_signer(crate::signer::AnySigner::C(signer));
    }

    let bytes = ffi_try!(transaction.to_bytes());

    unsafe { make_bytes2(bytes, buf, buf_size) }

    Error::Ok
}
