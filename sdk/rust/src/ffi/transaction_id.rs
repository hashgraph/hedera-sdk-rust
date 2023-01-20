use std::ptr;

use libc::size_t;

use super::error::Error;
use crate::protobuf::ToProtobuf;

#[repr(C)]
pub struct TransactionId {
    account_id: super::AccountId,
    valid_start: super::Timestamp,
    nonce: i32,
    scheduled: bool,
}

impl TransactionId {
    fn borrow_ref(&self) -> RefTransactionId<'_> {
        RefTransactionId {
            account_id: self.account_id.borrow_ref(),
            valid_start: self.valid_start,
            nonce: self.nonce,
            scheduled: self.scheduled,
        }
    }
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

struct RefTransactionId<'a> {
    account_id: super::RefAccountId<'a>,
    valid_start: super::Timestamp,
    nonce: i32,
    scheduled: bool,
}

impl<'a> RefTransactionId<'a> {
    fn into_bytes(self) -> Vec<u8> {
        use prost::Message;
        self.to_protobuf().encode_to_vec()
    }
}

impl<'a> ToProtobuf for RefTransactionId<'a> {
    type Protobuf = hedera_proto::services::TransactionId;

    fn to_protobuf(&self) -> Self::Protobuf {
        use hedera_proto::services;

        services::TransactionId {
            transaction_valid_start: Some(self.valid_start.into()),
            account_id: Some(self.account_id.to_protobuf()),
            scheduled: self.scheduled,
            nonce: self.nonce,
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_id_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    transation_id: *mut TransactionId,
) -> Error {
    assert!(!bytes.is_null());
    assert!(!transation_id.is_null());

    let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_size) };

    let parsed = ffi_try!(crate::TransactionId::from_bytes(bytes)).into();

    unsafe {
        ptr::write(transation_id, parsed);
    }

    Error::Ok
}

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_id_to_bytes(
    id: TransactionId,
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
