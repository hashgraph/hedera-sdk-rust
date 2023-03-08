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
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

thread_local! {
    static LAST_ERROR: RefCell<Option<crate::Error>> = RefCell::new(None);
    static LAST_ERROR_MESSAGE: RefCell<CString> = RefCell::new(CString::new("").unwrap());
}

macro_rules! ffi_try {
    ($expr:expr) => {{
        match $expr {
            Ok(it) => it,
            Err(error) => {
                return $crate::ffi::error::Error::new(error);
            }
        }
    }};
}

/// Update the most recently set error, for this thread, clearing whatever may have been there before.
pub(crate) fn set_last_error(error: crate::Error) {
    LAST_ERROR.with(|slot| {
        slot.borrow_mut().replace(error);
    });
}

/// Represents any possible result from a fallible function in the Hedera SDK.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[repr(C)]
pub enum Error {
    Ok,
    TimedOut,
    GrpcStatus,
    FromProtobuf,
    TransactionPreCheckStatus,
    TransactionNoIdPreCheckStatus,
    QueryPreCheckStatus,
    QueryPaymentPreCheckStatus,
    QueryNoPaymentPreCheckStatus,
    BasicParse,
    KeyParse,
    KeyDerive,
    NoPayerAccountOrTransactionId,
    FreezeUnsetNodeAccountIds,
    MaxQueryPaymentExceeded,
    NodeAccountUnknown,
    ResponseStatusUnrecognized,
    ReceiptStatus,
    Signature,
    RequestParse,
    MnemonicParse,
    MnemonicEntropy,
    SignatureVerify,
    BadEntityId,
    CannotToStringWithChecksum,
    CannotPerformTaskWithoutLedgerId,
    NoEvmAddressPresent,
    WrongKeyType,
}

impl Error {
    pub(crate) fn new(error: crate::Error) -> Self {
        let err = match &error {
            crate::Error::TimedOut(_) => Self::TimedOut,
            crate::Error::GrpcStatus(_) => Self::GrpcStatus,
            crate::Error::FromProtobuf(_) => Self::FromProtobuf,
            crate::Error::TransactionPreCheckStatus { .. } => Self::TransactionPreCheckStatus,
            crate::Error::TransactionNoIdPreCheckStatus { .. } => {
                Self::TransactionNoIdPreCheckStatus
            }
            crate::Error::QueryPreCheckStatus { .. } => Self::QueryPreCheckStatus,
            crate::Error::QueryPaymentPreCheckStatus { .. } => Self::QueryPaymentPreCheckStatus,
            crate::Error::QueryNoPaymentPreCheckStatus { .. } => Self::QueryNoPaymentPreCheckStatus,
            crate::Error::BasicParse(_) => Self::BasicParse,
            crate::Error::KeyParse(_) => Self::KeyParse,
            crate::Error::KeyDerive(_) => Self::KeyDerive,
            crate::Error::NoPayerAccountOrTransactionId => Self::NoPayerAccountOrTransactionId,
            crate::Error::MaxQueryPaymentExceeded { .. } => Self::MaxQueryPaymentExceeded,
            crate::Error::NodeAccountUnknown(_) => Self::NodeAccountUnknown,
            crate::Error::ResponseStatusUnrecognized(_) => Self::ResponseStatusUnrecognized,
            crate::Error::ReceiptStatus { .. } => Self::ReceiptStatus,
            crate::Error::Signature(_) => Self::Signature,
            crate::Error::RequestParse(_) => Self::RequestParse,
            crate::Error::MnemonicParse { .. } => Self::MnemonicParse,
            crate::Error::MnemonicEntropy(_) => Self::MnemonicEntropy,
            crate::Error::SignatureVerify(_) => Self::SignatureVerify,
            crate::Error::BadEntityId { .. } => Self::BadEntityId,
            crate::Error::CannotToStringWithChecksum => Self::CannotToStringWithChecksum,
            crate::Error::CannotPerformTaskWithoutLedgerId { .. } => {
                Self::CannotPerformTaskWithoutLedgerId
            }
            crate::Error::NoEvmAddressPresent { .. } => Self::NoEvmAddressPresent,
            crate::Error::WrongKeyType { .. } => Self::WrongKeyType,
            crate::Error::FreezeUnsetNodeAccountIds => Self::FreezeUnsetNodeAccountIds,
        };

        set_last_error(error);

        err
    }
}

/// Returns English-language text that describes the last error. `null` if there has been
/// no last error.
///
/// Note: the returned string must be freed via `hedera_string_free` in order to prevent a memory leak.
///
/// # Safety
/// - the length of the returned string must not be modified.
/// - the returned string must NOT be freed with `free`.
#[no_mangle]
pub extern "C" fn hedera_error_message() -> *mut c_char {
    LAST_ERROR.with(|error| {
        if let Some(error) = &*error.borrow() {
            return CString::new(error.to_string()).unwrap().into_raw();
        }

        ptr::null_mut()
    })
}

#[no_mangle]
pub extern "C" fn hedera_last_error_details() -> ErrorDetails {
    LAST_ERROR.with(|it| match it.take() {
        Some(err) => ErrorDetails::from(err),
        None => ErrorDetails::None,
    })
}

// todo:
// #[repr(C)]
// struct ErasedError(Box<dyn std::error::Error>);

#[repr(C)]
pub enum ErrorDetails {
    None,
    ErrorGrpcStatus(i32),
    ErrorStatusTransactionId {
        status: i32,
        transaction_id: super::transaction_id::TransactionId,
    },
    ErrorStatusNoTransactionId {
        status: i32,
    },
    ErrorMaxQueryPaymentExceeded {
        max_query_payment: i64,
        query_cost: i64,
    },
    ErrorBadEntityId {
        shard: u64,
        realm: u64,
        num: u64,
        present_checksum: [u8; 5],
        expected_checksum: [u8; 5],
    },
}

// todo: Return `ErasedError`
impl From<crate::Error> for ErrorDetails {
    fn from(value: crate::Error) -> Self {
        use crate::Error;
        match value {
            Error::GrpcStatus(status) => ErrorDetails::ErrorGrpcStatus(status.code() as i32),
            Error::TransactionPreCheckStatus { status, transaction_id }
            | Error::QueryPreCheckStatus { status, transaction_id }
            | Error::QueryPaymentPreCheckStatus { status, transaction_id }
            | Error::ReceiptStatus { status, transaction_id: Some(transaction_id) } => {
                Self::ErrorStatusTransactionId {
                    status: status as i32,
                    transaction_id: (*transaction_id).into(),
                }
            }

            Error::MaxQueryPaymentExceeded { max_query_payment, query_cost } => {
                Self::ErrorMaxQueryPaymentExceeded {
                    max_query_payment: max_query_payment.to_tinybars(),
                    query_cost: query_cost.to_tinybars(),
                }
            }
            Error::BadEntityId { shard, realm, num, present_checksum, expected_checksum } => {
                Self::ErrorBadEntityId {
                    shard,
                    realm,
                    num,
                    present_checksum: present_checksum.to_bytes(),
                    expected_checksum: expected_checksum.to_bytes(),
                }
            }
            Error::TransactionNoIdPreCheckStatus { status }
            | Error::QueryNoPaymentPreCheckStatus { status }
            | Error::ReceiptStatus { status, transaction_id: None } => {
                Self::ErrorStatusNoTransactionId { status: status as i32 }
            }
            _ => ErrorDetails::None,
        }
    }
}
