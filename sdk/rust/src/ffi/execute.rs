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
use crate::ffi::util::cstr_from_ptr;
use crate::transaction::AnyTransaction;
use crate::{
    AnyMirrorQuery,
    AnyQuery,
    Client,
};

#[derive(serde::Deserialize)]
#[cfg_attr(feature = "ffi", serde(untagged))]
enum AnyRequest {
    Transaction(Box<AnyTransaction>),
    Query(Box<AnyQuery>),
    MirrorQuery(AnyMirrorQuery),
    QueryCost(QueryCostRequest),
}

#[derive(serde::Deserialize)]
struct QueryCostRequest {
    query: Box<AnyQuery>,
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
    callback: extern "C" fn(context: *const c_void, err: Error, response: *const c_char),
) -> Error {
    assert!(!client.is_null());

    let client = unsafe { &*client };
    let request = unsafe { cstr_from_ptr(request) };

    let request: AnyRequest =
        ffi_try!(serde_json::from_str(&request).map_err(crate::Error::request_parse));

    let signers_2: Vec<_> = signers.as_slice().iter().map(|it| it.to_csigner()).collect();

    drop(signers);
    let signers = signers_2;

    let callback = Callback::new(context, callback);

    RUNTIME.spawn(async move {
        let response = match request {
            AnyRequest::Query(mut query) => query
                .execute(client)
                .await
                .map(|response| serde_json::to_string(&response).unwrap()),

            AnyRequest::Transaction(mut transaction) => {
                for signer in signers {
                    transaction.sign_signer(crate::signer::Signer::C(signer));
                }

                transaction
                    .execute(client)
                    .await
                    .map(|response| serde_json::to_string(&response).unwrap())
            }

            AnyRequest::MirrorQuery(mut mirror_query) => mirror_query
                .execute(client)
                .await
                .map(|response| serde_json::to_string(&response).unwrap()),

            AnyRequest::QueryCost(req) => req
                .query
                .get_cost(client)
                .await
                .map(|response| serde_json::to_string(&response).unwrap()),
        };

        let response =
            response.map(|response| CString::new(response).unwrap().into_raw() as *const c_char);

        let (err, response) = match response {
            Ok(response) => (Error::Ok, response),
            Err(error) => (Error::new(error), ptr::null()),
        };

        callback.call(err, response);

        if !response.is_null() {
            drop(unsafe { CString::from_raw(response as *mut _) });
        }
    });

    Error::Ok
}
