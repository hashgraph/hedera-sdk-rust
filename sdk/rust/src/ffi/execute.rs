use std::cell::RefCell;
use std::ffi::{c_void, CString};
use std::os::raw::c_char;
use std::ptr::null;

use once_cell::sync::Lazy;
use tokio::runtime::{self, Runtime};

use crate::ffi::callback::Callback;
use crate::ffi::error::FfiResult;
use crate::ffi::util::cstr_from_ptr;
use crate::{AnyQuery, Client, Error};

static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    runtime::Builder::new_multi_thread().enable_all().max_blocking_threads(8).build().unwrap()
});

thread_local! {
    static EXECUTE_RESPONSE: RefCell<CString> = RefCell::new(CString::new("").unwrap());
}

/// Execute this request against the provided client of the Hedera network.
#[no_mangle]
pub extern "C" fn hedera_execute(
    client: *const Client,
    request: *const c_char,
    context: *const c_void,
    callback: extern "C" fn(context: *const c_void, err: FfiResult, response: *const c_char),
) -> FfiResult {
    assert!(!client.is_null());

    let client = unsafe { &*client };
    let request = unsafe { cstr_from_ptr(request) };

    let mut query: AnyQuery =
        ffi_try!(serde_json::from_str(&request).map_err(Error::request_parse));

    let callback = Callback::new(context, callback);

    RUNTIME.spawn(async move {
        let response = query.execute(client).await.map(|response| {
            EXECUTE_RESPONSE.with(|response_text| {
                *response_text.borrow_mut() =
                    CString::new(serde_json::to_string(&response).unwrap()).unwrap();

                response_text.borrow().as_ptr()
            })
        });

        let (err, response) = match response {
            Ok(response) => (FfiResult::Ok, response),
            Err(error) => (FfiResult::new(error), null()),
        };

        callback.call(err, response);
    });

    FfiResult::Ok
}
