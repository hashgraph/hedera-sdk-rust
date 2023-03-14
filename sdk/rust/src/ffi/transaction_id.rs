use std::ffi::c_char;
use std::ptr;
use std::str::FromStr;

use super::error::Error;
use crate::ffi::util::cstr_from_ptr;

#[repr(C)]
pub struct TransactionId {
    account_id: super::AccountId,
    valid_start: super::Timestamp,
    nonce: i32,
    scheduled: bool,
}

impl From<crate::TransactionId> for TransactionId {
    fn from(it: crate::TransactionId) -> Self {
        let crate::TransactionId { account_id, valid_start, nonce, scheduled } = it;
        Self {
            account_id: account_id.into(),
            nonce: nonce.unwrap_or_default(),
            valid_start: valid_start.into(),
            scheduled,
        }
    }
}

// this function mostly exists because Rust parsing is nicer, has better errors, etc.
// plus the module is needed anyway for to/from bytes. So, might as well.
/// # Safety
/// - `s` must be a valid string
/// - `transaction_id` must be a valid for writes according to [*Rust* pointer rules].
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_id_from_string(
    s: *const c_char,
    transation_id: *mut TransactionId,
) -> Error {
    assert!(!transation_id.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(crate::TransactionId::from_str(&s)).into();

    unsafe {
        ptr::write(transation_id, parsed);
    }

    Error::Ok
}
