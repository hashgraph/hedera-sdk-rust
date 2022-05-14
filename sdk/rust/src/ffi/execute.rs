use std::ffi::{c_void, CString};
use std::os::raw::c_char;

use once_cell::sync::Lazy;
use tokio::runtime::{self, Runtime};

use crate::ffi::callback::Callback;
use crate::ffi::util::cstr_from_ptr;
use crate::{AnyQuery, Client};

static RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    runtime::Builder::new_multi_thread().enable_all().max_blocking_threads(8).build().unwrap()
});

/// Execute this request against the provided client of the Hedera network.
#[no_mangle]
pub extern "C" fn hedera_execute(
    client: *const Client,
    request: *const c_char,
    context: *const c_void,
    callback: extern "C" fn(context: *const c_void, value: *const c_char),
) {
    assert!(!client.is_null());

    let client = unsafe { &*client };
    let request = unsafe { cstr_from_ptr(request) };

    // TODO: handle errors
    let mut query: AnyQuery = serde_json::from_str(&request).unwrap();

    let callback = Callback::new(context, callback);

    RUNTIME.spawn(async move {
        let response = query.execute(client).await;
        let response = response.unwrap();

        // FIXME: use static TLS for response storage here
        let response = serde_json::to_string(&response).unwrap();
        let response = CString::new(response).unwrap();

        callback.call(response.as_ptr());
    });
}
