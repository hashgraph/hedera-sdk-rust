use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;

use crate::Error;

thread_local! {
    static LAST_ERROR: RefCell<Option<Error>> = RefCell::new(None);
    static LAST_ERROR_MESSAGE: RefCell<CString> = RefCell::new(CString::new("").unwrap());
}

macro_rules! ffi_try {
    ($expr:expr) => {{
        match $expr {
            Ok(it) => it,
            Err(error) => {
                return $crate::ffi::error::FfiResult::new(error);
            }
        }
    }};
}

/// Update the most recently set error, for this thread, clearing whatever may have been there before.
pub(crate) fn set_last_error(error: Error) {
    LAST_ERROR.with(|slot| {
        slot.borrow_mut().replace(error);
    })
}

/// Represents any possible result from a fallible function in the Hedera SDK.
#[derive(Debug)]
#[repr(C)]
pub enum FfiResult {
    Ok = 0,
    ErrTimedOut = 1,
    ErrGrpcStatus = 2,
    ErrFromProtobuf = 3,
    ErrPreCheckStatus = 4,
    ErrBasicParse = 5,
    ErrKeyParse = 6,
    ErrNoPayerAccountOrTransactionId = 7,
    ErrMaxAttemptsExceeded = 8,
    ErrMaxQueryPaymentExceeded = 9,
    ErrNodeAccountUnknown = 10,
    ErrResponseStatusUnrecognized = 11,
    ErrSignature = 12,
    ErrRequestParse = 13,
}

impl FfiResult {
    pub(crate) fn new(error: Error) -> Self {
        let result = match &error {
            Error::TimedOut(_) => Self::ErrTimedOut,
            Error::GrpcStatus(_) => Self::ErrGrpcStatus,
            Error::FromProtobuf(_) => Self::ErrFromProtobuf,
            Error::PreCheckStatus { .. } => Self::ErrPreCheckStatus,
            Error::BasicParse(_) => Self::ErrBasicParse,
            Error::KeyParse(_) => Self::ErrKeyParse,
            Error::NoPayerAccountOrTransactionId => Self::ErrNoPayerAccountOrTransactionId,
            Error::MaxAttemptsExceeded(_) => Self::ErrMaxAttemptsExceeded,
            Error::MaxQueryPaymentExceeded { .. } => Self::ErrMaxQueryPaymentExceeded,
            Error::NodeAccountUnknown(_) => Self::ErrNodeAccountUnknown,
            Error::ResponseStatusUnrecognized(_) => Self::ErrResponseStatusUnrecognized,
            Error::Signature(_) => Self::ErrSignature,
            Error::RequestParse(_) => Self::ErrRequestParse,
        };

        set_last_error(error);

        result
    }
}

/// Returns English-language text that describes the last error. Undefined if there has been
/// no last error.
#[no_mangle]
pub extern "C" fn hedera_error_message() -> *const c_char {
    LAST_ERROR_MESSAGE.with(|message| {
        LAST_ERROR.with(|error| {
            if let Some(error) = &*error.borrow() {
                *message.borrow_mut() = CString::new(error.to_string()).unwrap();
            }
        });

        message.borrow().as_ptr()
    })
}

/// Returns the GRPC status code for the last error. Undefined if the last error was not
/// `HEDERA_RESULT_ERR_GRPC_STATUS`.
#[no_mangle]
pub extern "C" fn hedera_error_grpc_status() -> i32 {
    LAST_ERROR.with(|error| {
        if let Some(error) = &*error.borrow() {
            if let Error::GrpcStatus(status) = error {
                return status.code() as i32;
            }
        }

        // NOTE: -1 is an unlikely sentinel value for if this error wasn't a GrpcStatus
        -1
    })
}

/// Returns the hedera services response code for the last error. Undefined if the last error
/// was not `HEDERA_RESULT_ERR_PRE_CHECK_STATUS`.
#[no_mangle]
pub extern "C" fn hedera_error_pre_check_status() -> i32 {
    LAST_ERROR.with(|error| {
        if let Some(error) = &*error.borrow() {
            if let Error::PreCheckStatus { status, .. } = error {
                return *status as i32;
            }
        }

        // NOTE: -1 is an unlikely sentinel value for if this error wasn't a GrpcStatus
        -1
    })
}
