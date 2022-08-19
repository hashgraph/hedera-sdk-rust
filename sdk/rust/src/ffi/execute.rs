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

use std::cell::RefCell;
use std::ffi::{
    c_void,
    CString,
};
use std::os::raw::c_char;
use std::ptr::null;

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

thread_local! {
    static EXECUTE_RESPONSE: RefCell<CString> = RefCell::new(CString::new("").unwrap());
}

#[derive(serde::Deserialize)]
#[serde(untagged)]
enum AnyRequest {
    Transaction(AnyTransaction),
    Query(AnyQuery),
    MirrorQuery(AnyMirrorQuery),
}

/// Execute this request against the provided client of the Hedera network.
#[no_mangle]
pub extern "C" fn hedera_execute(
    client: *const Client,
    request: *const c_char,
    context: *const c_void,
    callback: extern "C" fn(context: *const c_void, err: Error, response: *const c_char),
) -> Error {
    assert!(!client.is_null());

    let client = unsafe { &*client };
    let request = unsafe { cstr_from_ptr(request) };

    let request: AnyRequest =
        ffi_try!(serde_json::from_str(&request).map_err(crate::Error::request_parse));

    let callback = Callback::new(context, callback);

    RUNTIME.spawn(async move {
        let response = match request {
            AnyRequest::Query(mut query) => query
                .execute(client)
                .await
                .map(|response| serde_json::to_string(&response).unwrap()),

            AnyRequest::Transaction(mut transaction) => transaction
                .execute(client)
                .await
                .map(|response| serde_json::to_string(&response).unwrap()),

            AnyRequest::MirrorQuery(mut mirror_query) => mirror_query
                .execute(client)
                .await
                .map(|response| serde_json::to_string(&response).unwrap()),
        };

        let response = response.map(|response| {
            EXECUTE_RESPONSE.with(|response_text| {
                *response_text.borrow_mut() = CString::new(response).unwrap();

                response_text.borrow().as_ptr()
            })
        });

        let (err, response) = match response {
            Ok(response) => (Error::Ok, response),
            Err(error) => (Error::new(error), null()),
        };

        callback.call(err, response);
    });

    Error::Ok
}
