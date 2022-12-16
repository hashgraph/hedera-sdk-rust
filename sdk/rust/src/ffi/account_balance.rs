use std::ptr;

use libc::size_t;

use crate::ffi;
use crate::ffi::error::Error;
use crate::protobuf::ToProtobuf;

#[repr(C)]
pub struct AccountBalance {
    id: ffi::AccountId,
    hbars: i64,
}

impl AccountBalance {
    fn borrow_ref<'a>(&'a self) -> RefAccountBalance<'a> {
        RefAccountBalance { id: self.id.borrow_ref(), hbars: self.hbars }
    }
}

impl From<crate::AccountBalance> for AccountBalance {
    fn from(it: crate::AccountBalance) -> Self {
        Self { id: it.account_id.into(), hbars: it.hbars.to_tinybars() }
    }
}

struct RefAccountBalance<'a> {
    id: ffi::RefAccountId<'a>,
    hbars: i64,
}

impl<'a> ToProtobuf for RefAccountBalance<'a> {
    type Protobuf = hedera_proto::services::CryptoGetAccountBalanceResponse;

    fn to_protobuf(&self) -> Self::Protobuf {
        use hedera_proto::services;

        #[allow(deprecated)]
        services::CryptoGetAccountBalanceResponse {
            header: None,
            account_id: Some(self.id.to_protobuf()),
            balance: self.hbars as u64,
            token_balances: Vec::new(),
        }
    }
}

impl<'a> RefAccountBalance<'a> {
    fn into_bytes(self) -> Vec<u8> {
        use prost::Message;
        self.to_protobuf().encode_to_vec()
    }
}

/// Parse a Hedera `AccountBalance` from the passed bytes.
#[no_mangle]
pub unsafe extern "C" fn hedera_account_balance_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    id: *mut AccountBalance,
) -> Error {
    assert!(!bytes.is_null());
    assert!(!id.is_null());

    let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_size) };

    let parsed = ffi_try!(crate::AccountBalance::from_bytes(&bytes)).into();

    unsafe {
        ptr::write(id, parsed);
    }

    Error::Ok
}

/// Serialize the passed `AccountBalance` as bytes
///
/// # Safety
/// - `id` must uphold the safety requirements of `AccountBalance`.
/// - `buf` must be valid for writes.
/// - `buf` must only be freed with `hedera_bytes_free`, notably this means that it must not be freed with `free`.
#[no_mangle]
pub unsafe extern "C" fn hedera_account_balance_to_bytes(
    id: AccountBalance,
    buf: *mut *mut u8,
) -> size_t {
    let bytes = id.borrow_ref().into_bytes().into_boxed_slice();

    let bytes = Box::leak(bytes);
    let len = bytes.len();
    let bytes = bytes.as_mut_ptr();

    // safety: invariants promise that `buf` must be valid for writes.
    unsafe {
        ptr::write(buf, bytes);
    }

    len
}
