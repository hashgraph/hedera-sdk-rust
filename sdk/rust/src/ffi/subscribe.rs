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

use futures_util::StreamExt;

use crate::ffi::callback::Callback;
use crate::ffi::error::Error;
use crate::ffi::runtime::RUNTIME;
use crate::ffi::util::cstr_from_ptr;
use crate::{
    AnyMirrorQuery,
    Client,
};

thread_local! {
    static SUBSCRIBE_MESSAGE: RefCell<CString> = RefCell::new(CString::new("").unwrap());
}

/// Subscribe with this request against the provided client of the Hedera network.
/// On successful completion, calls `callback` with `ERROR_OK` and a `NULL` `message`.
#[no_mangle]
pub extern "C" fn hedera_subscribe(
    client: *const Client,
    request: *const c_char,
    context: *const c_void,
    callback: extern "C" fn(context: *const c_void, err: Error, message: *const c_char),
) -> Error {
    assert!(!client.is_null());

    let client = unsafe { &*client };
    let request = unsafe { cstr_from_ptr(request) };

    let request: AnyMirrorQuery =
        ffi_try!(serde_json::from_str(&request).map_err(crate::Error::request_parse));

    let callback = Callback::new(context, callback);

    RUNTIME.spawn(async move {
        let mut stream = request.subscribe(client);

        while let Some(message) = stream.next().await {
            let message = message.map(|message| {
                let message = serde_json::to_string(&message).unwrap();

                SUBSCRIBE_MESSAGE.with(|message_text| {
                    *message_text.borrow_mut() = CString::new(message).unwrap();

                    message_text.borrow().as_ptr()
                })
            });

            let (err, message) = match message {
                Ok(message) => (Error::Ok, message),
                Err(error) => (Error::new(error), null()),
            };

            callback.call(err, message);

            if err != Error::Ok {
                return;
            }
        }

        callback.call(Error::Ok, null());
    });

    Error::Ok
}
