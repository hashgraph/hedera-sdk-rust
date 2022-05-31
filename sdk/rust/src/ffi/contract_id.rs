use std::os::raw::c_char;
use std::str::FromStr;

use crate::ffi::error::Error;
use crate::ffi::util::cstr_from_ptr;
use crate::ContractId;

/// Parse a Hedera `ContractId` from the passed string.
#[no_mangle]
pub extern "C" fn hedera_contract_id_from_string(s: *const c_char, id: *mut ContractId) -> Error {
    assert!(!id.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(ContractId::from_str(&s));

    unsafe {
        *id = parsed;
    }

    Error::Ok
}
