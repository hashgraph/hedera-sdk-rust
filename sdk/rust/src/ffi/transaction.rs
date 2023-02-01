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

use libc::{
    c_void,
    size_t,
};

use super::error::Error;
use super::signer::{
    Signer,
    Signers,
};
use super::util::{
    cstr_from_ptr,
    make_bytes2,
};
use crate::ffi::callback::Callback;
use crate::transaction::AnyTransaction;
use crate::Client;

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

    let mut transaction: AnyTransaction =
        ffi_try!(serde_json::from_str(&transaction).map_err(crate::Error::request_parse));

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

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    sources_out: *mut *mut crate::transaction::TransactionSources,
    transaction_out: *mut *mut c_char,
) -> Error {
    assert!(!bytes.is_null());
    assert!(!sources_out.is_null());
    assert!(!transaction_out.is_null());

    let bytes = unsafe { slice::from_raw_parts(bytes, bytes_size) };

    let tx = ffi_try!(AnyTransaction::from_bytes(bytes));

    let sources = Box::new(tx.sources().unwrap().clone());

    let sources = Box::into_raw(sources);

    let out = serde_json::to_vec(&tx).unwrap();

    let out = CString::new(out).unwrap().into_raw();

    unsafe {
        ptr::write(sources_out, sources);
        ptr::write(transaction_out, out)
    }

    Error::Ok
}

/// Execute this request against the provided client of the Hedera network.
///
/// # Safety
/// - todo(sr): Missing basically everything
/// - `callback` must not store `response` after it returns.
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_execute(
    client: *const Client,
    request: *const c_char,
    context: *const c_void,
    signers: Signers,
    has_timeout: bool,
    timeout: f64,
    sources: *const crate::transaction::TransactionSources,
    callback: extern "C" fn(context: *const c_void, err: Error, response: *const c_char),
) -> Error {
    assert!(!client.is_null());

    let client = unsafe { &*client };
    let sources = unsafe { sources.as_ref() };
    let request = unsafe { cstr_from_ptr(request) };

    let mut transaction: AnyTransaction =
        ffi_try!(serde_json::from_str(&request).map_err(crate::Error::request_parse));

    let signers_2: Vec<_> = signers.as_slice().iter().map(Signer::to_csigner).collect();

    let timeout = has_timeout
        .then(|| std::time::Duration::try_from_secs_f64(timeout))
        .transpose()
        .map_err(crate::Error::request_parse);

    let timeout = ffi_try!(timeout);

    drop(signers);
    let signers = signers_2;

    let callback = Callback::new(context, callback);

    super::runtime::RUNTIME.spawn(async move {
        let response = {
            for signer in signers {
                transaction.sign_signer(crate::signer::AnySigner::C(signer));
            }

            let res = match sources {
                Some(sources) => {
                    crate::transaction::ExecuteTransaction::new(&transaction, sources)
                        .execute(client, timeout)
                        .await
                }
                None => transaction.execute_with_optional_timeout(client, timeout).await,
            };

            res.map(|response| serde_json::to_string(&response).unwrap())
        };

        let response =
            response.map(|response| CString::new(response).unwrap().into_raw().cast_const());

        let (err, response) = match response {
            Ok(response) => (Error::Ok, response),
            Err(error) => (Error::new(error), ptr::null()),
        };

        callback.call(err, response);

        if !response.is_null() {
            drop(unsafe { CString::from_raw(response.cast_mut()) });
        }
    });

    Error::Ok
}

/// # Safety
/// - `sources` must be non-null and point to a `HederaTransactionSources` allocated by the Hedera SDK.
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_sources_free(
    sources: *mut crate::transaction::TransactionSources,
) {
    assert!(!sources.is_null());

    drop(unsafe { Box::from_raw(sources) })
}
