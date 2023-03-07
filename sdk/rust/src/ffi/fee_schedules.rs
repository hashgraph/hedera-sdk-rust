use libc::c_char;

use super::error::Error;
use super::util;
use crate::{
    FeeComponents,
    FeeData,
    FeeSchedule,
    FeeSchedules,
    TransactionFeeSchedule,
};

/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `s` must only be freed with `hedera_string_free`,
///   notably this means it must not be freed with `free`.
#[no_mangle]
pub unsafe extern "C" fn hedera_fee_schedules_from_bytes(
    bytes: *const u8,
    bytes_size: libc::size_t,
    s: *mut *mut c_char,
) -> Error {
    unsafe { util::json_from_bytes(bytes, bytes_size, s, FeeSchedules::from_bytes) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_fee_schedules_to_bytes(
    s: *const c_char,
    buf: *mut *mut u8,
    buf_size: *mut libc::size_t,
) -> Error {
    unsafe { util::json_to_bytes(s, buf, buf_size, FeeSchedules::to_bytes) }
}

/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `s` must only be freed with `hedera_string_free`,
///   notably this means it must not be freed with `free`.
#[no_mangle]
pub unsafe extern "C" fn hedera_fee_schedule_from_bytes(
    bytes: *const u8,
    bytes_size: libc::size_t,
    s: *mut *mut c_char,
) -> Error {
    unsafe { util::json_from_bytes(bytes, bytes_size, s, FeeSchedule::from_bytes) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_fee_schedule_to_bytes(
    s: *const c_char,
    buf: *mut *mut u8,
    buf_size: *mut libc::size_t,
) -> Error {
    unsafe { util::json_to_bytes(s, buf, buf_size, FeeSchedule::to_bytes) }
}

/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `s` must only be freed with `hedera_string_free`,
///   notably this means it must not be freed with `free`.
#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_fee_schedule_from_bytes(
    bytes: *const u8,
    bytes_size: libc::size_t,
    s: *mut *mut c_char,
) -> Error {
    unsafe { util::json_from_bytes(bytes, bytes_size, s, TransactionFeeSchedule::from_bytes) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_transaction_fee_schedule_to_bytes(
    s: *const c_char,
    buf: *mut *mut u8,
    buf_size: *mut libc::size_t,
) -> Error {
    unsafe { util::json_to_bytes(s, buf, buf_size, TransactionFeeSchedule::to_bytes) }
}

/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `s` must only be freed with `hedera_string_free`,
///   notably this means it must not be freed with `free`.
#[no_mangle]
pub unsafe extern "C" fn hedera_fee_data_from_bytes(
    bytes: *const u8,
    bytes_size: libc::size_t,
    s: *mut *mut c_char,
) -> Error {
    unsafe { util::json_from_bytes(bytes, bytes_size, s, FeeData::from_bytes) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_fee_data_to_bytes(
    s: *const c_char,
    buf: *mut *mut u8,
    buf_size: *mut libc::size_t,
) -> Error {
    unsafe { util::json_to_bytes(s, buf, buf_size, FeeData::to_bytes) }
}

/// # Safety
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
/// - `s` must only be freed with `hedera_string_free`,
///   notably this means it must not be freed with `free`.
#[no_mangle]
pub unsafe extern "C" fn hedera_fee_components_from_bytes(
    bytes: *const u8,
    bytes_size: libc::size_t,
    s: *mut *mut c_char,
) -> Error {
    unsafe { util::json_from_bytes(bytes, bytes_size, s, FeeComponents::from_bytes) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_fee_components_to_bytes(
    s: *const c_char,
    buf: *mut *mut u8,
    buf_size: *mut libc::size_t,
) -> Error {
    unsafe { util::json_to_bytes(s, buf, buf_size, FeeComponents::to_bytes) }
}
