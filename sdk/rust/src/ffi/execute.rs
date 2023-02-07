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
    c_void,
    CString,
};
use std::os::raw::c_char;
use std::ptr;

use super::signer::Signers;
use crate::ffi::callback::Callback;
use crate::ffi::error::Error;
use crate::ffi::runtime::RUNTIME;
use crate::ffi::signer::Signer;
use crate::ffi::util::cstr_from_ptr;
use crate::transaction::AnyTransaction;
use crate::Client;

#[derive(serde::Deserialize)]
#[cfg_attr(feature = "ffi", serde(untagged))]
enum AnyRequest {
    Transaction(Box<AnyTransaction>),
}

/// Execute this request against the provided client of the Hedera network.
///
/// # Safety
/// - todo(sr): Missing basically everything
/// - `callback` must not store `response` after it returns.
#[no_mangle]
pub unsafe extern "C" fn hedera_execute(
    client: *const Client,
    request: *const c_char,
    context: *const c_void,
    signers: Signers,
    has_timeout: bool,
    timeout: f64,
    callback: extern "C" fn(context: *const c_void, err: Error, response: *const c_char),
) -> Error {
    assert!(!client.is_null());

    let client = unsafe { &*client };
    let request = unsafe { cstr_from_ptr(request) };

    let request: AnyRequest =
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

    RUNTIME.spawn(async move {
        let response = match request {
            AnyRequest::Transaction(mut transaction) => {
                for signer in signers {
                    transaction.sign_signer(crate::signer::AnySigner::C(signer));
                }

                transaction
                    .execute_with_optional_timeout(client, timeout)
                    .await
                    .map(|response| serde_json::to_string(&response).unwrap())
            }
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
