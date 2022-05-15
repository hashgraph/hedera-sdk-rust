use std::cell::RefCell;
use std::ffi::CString;
use std::os::raw::c_char;

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
    })
}

/// Represents any possible result from a fallible function in the Hedera SDK.
#[derive(Debug)]
#[repr(C)]
pub enum Error {
    Ok = 0,
    TimedOut = 1,
    GrpcStatus = 2,
    FromProtobuf = 3,
    PreCheckStatus = 4,
    BasicParse = 5,
    KeyParse = 6,
    NoPayerAccountOrTransactionId = 7,
    MaxAttemptsExceeded = 8,
    MaxQueryPaymentExceeded = 9,
    NodeAccountUnknown = 10,
    ResponseStatusUnrecognized = 11,
    Signature = 12,
    RequestParse = 13,
}

impl Error {
    pub(crate) fn new(error: crate::Error) -> Self {
        let err = match &error {
            crate::Error::TimedOut(_) => Self::TimedOut,
            crate::Error::GrpcStatus(_) => Self::GrpcStatus,
            crate::Error::FromProtobuf(_) => Self::FromProtobuf,
            crate::Error::PreCheckStatus { .. } => Self::PreCheckStatus,
            crate::Error::BasicParse(_) => Self::BasicParse,
            crate::Error::KeyParse(_) => Self::KeyParse,
            crate::Error::NoPayerAccountOrTransactionId => Self::NoPayerAccountOrTransactionId,
            crate::Error::MaxAttemptsExceeded(_) => Self::MaxAttemptsExceeded,
            crate::Error::MaxQueryPaymentExceeded { .. } => Self::MaxQueryPaymentExceeded,
            crate::Error::NodeAccountUnknown(_) => Self::NodeAccountUnknown,
            crate::Error::ResponseStatusUnrecognized(_) => Self::ResponseStatusUnrecognized,
            crate::Error::Signature(_) => Self::Signature,
            crate::Error::RequestParse(_) => Self::RequestParse,
        };

        set_last_error(error);

        err
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
/// `HEDERA_ERROR_GRPC_STATUS`.
#[no_mangle]
pub extern "C" fn hedera_error_grpc_status() -> i32 {
    LAST_ERROR.with(|error| {
        if let Some(error) = &*error.borrow() {
            if let crate::Error::GrpcStatus(status) = error {
                return status.code() as i32;
            }
        }

        // NOTE: -1 is an unlikely sentinel value for if this error wasn't a GrpcStatus
        -1
    })
}

/// Returns the hedera services response code for the last error. Undefined if the last error
/// was not `HEDERA_ERROR_PRE_CHECK_STATUS`.
#[no_mangle]
pub extern "C" fn hedera_error_pre_check_status() -> i32 {
    LAST_ERROR.with(|error| {
        if let Some(error) = &*error.borrow() {
            if let crate::Error::PreCheckStatus { status, .. } = error {
                return *status as i32;
            }
        }

        // NOTE: -1 is an unlikely sentinel value for if this error wasn't a GrpcStatus
        -1
    })
}
