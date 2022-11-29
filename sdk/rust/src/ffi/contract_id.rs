use std::ffi::{
    c_char,
    CString,
};
use std::ptr::{
    self,
    NonNull,
};
use std::slice;
use std::str::FromStr;

use libc::size_t;

use super::error::Error;
use crate::ffi::util::cstr_from_ptr;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ContractId {
    shard: u64,
    realm: u64,
    num: u64,

    /// # Safety
    /// - must either be null or valid for 20 bytes
    /// - if allocated by `hedera` it must be freed by hedera
    /// - otherwise must *not* be freed by hedera.
    evm_address: Option<NonNull<u8>>,
}

impl ContractId {
    fn from_rust(rust: crate::ContractId) -> ContractId {
        let crate::ContractId { shard, realm, num, evm_address } = rust;

        let evm_address =
            evm_address.map(|it| NonNull::new(Box::into_raw(Box::new(it)).cast::<u8>()).unwrap());

        Self { shard, realm, num, evm_address }
    }

    fn to_rust(self) -> crate::ContractId {
        let Self { shard, realm, num, evm_address } = self;

        let evm_address = evm_address.map(|it| unsafe { *it.cast::<[u8; 20]>().as_ref() });

        crate::ContractId { shard, realm, num, evm_address }
    }
}

/// Parse a Hedera `ContractId` from the passed string.
#[no_mangle]
pub extern "C" fn hedera_contract_id_from_string(
    s: *const c_char,
    contract_id: *mut ContractId,
) -> Error {
    assert!(!contract_id.is_null());

    let s = unsafe { cstr_from_ptr(s) };
    let parsed = ffi_try!(crate::ContractId::from_str(&s));

    let parsed = ContractId::from_rust(parsed);

    unsafe {
        ptr::write(contract_id, parsed);
    }

    Error::Ok
}

/// Parse a Hedera `ContractId` from the passed bytes.
///
/// # Safety
/// - `contract_id` be valid for writes.
/// - `bytes` must be valid for reads of up to `bytes_size` bytes.
#[no_mangle]
pub unsafe extern "C" fn hedera_contract_id_from_bytes(
    bytes: *const u8,
    bytes_size: size_t,
    contract_id: *mut ContractId,
) -> Error {
    // safety: caller promises that `bytes` is valid for r/w of up to `bytes_size`, which is exactly what `slice::from_raw_parts` wants.
    let bytes = unsafe { slice::from_raw_parts(bytes, bytes_size) };

    let parsed = ffi_try!(crate::ContractId::from_bytes(bytes));

    let parsed = ContractId::from_rust(parsed);

    unsafe {
        ptr::write(contract_id, parsed);
    }

    Error::Ok
}

/// Create a `ContractId` from a `shard.realm.evm_address` set.
///
/// # Safety
/// - `contract_id` must be valid for writes.
/// - `address` must be valid for reads up until the first `\0` character.
#[no_mangle]
pub unsafe extern "C" fn hedera_contract_id_from_evm_address(
    shard: u64,
    realm: u64,
    evm_address: *const c_char,
    contract_id: *mut ContractId,
) -> Error {
    assert!(!contract_id.is_null());

    let evm_address = unsafe { cstr_from_ptr(evm_address) };
    let parsed = ffi_try!(crate::ContractId::from_evm_address(shard, realm, &evm_address));

    let parsed = ContractId::from_rust(parsed);

    unsafe {
        ptr::write(contract_id, parsed);
    }

    Error::Ok
}

/// create a `ContractId` from a solidity address.
///
/// # Safety
/// - `contract_id` must be valid for writes.
/// - `address` must be valid for reads up until the first `\0` character.
#[no_mangle]
pub unsafe extern "C" fn hedera_contract_id_from_solidity_address(
    address: *const c_char,
    contract_id: *mut ContractId,
) -> Error {
    assert!(!contract_id.is_null());

    let evm_address = unsafe { cstr_from_ptr(address) };
    let parsed = ffi_try!(crate::ContractId::from_solidity_address(&evm_address));

    let parsed = ContractId::from_rust(parsed);

    unsafe {
        ptr::write(contract_id, parsed);
    }

    Error::Ok
}

/// Serialize the passed `ContractId` as bytes
///
/// # Safety
/// - `buf` must be valid for writes.
#[no_mangle]
pub unsafe extern "C" fn hedera_contract_id_to_bytes(
    contract_id: ContractId,
    buf: *mut *mut u8,
) -> size_t {
    // todo: use `as_maybe_uninit_ref` once that's stable.
    assert!(!buf.is_null());

    let bytes = contract_id.to_rust().to_bytes().into_boxed_slice();

    let bytes = Box::leak(bytes);
    let len = bytes.len();
    let bytes = bytes.as_mut_ptr();

    // safety: invariants promise that `buf` must be valid for writes.
    unsafe {
        ptr::write(buf, bytes);
    }

    len
}

/// Serialize the passed `ContractId` as a solidity `address`
///
/// # Safety
/// - `s` must be valid for writes
#[no_mangle]
pub unsafe extern "C" fn hedera_contract_id_to_solidity_address(
    contract_id: ContractId,
    s: *mut *mut c_char,
) -> Error {
    // todo: use `as_maybe_uninit_ref` once that's stable.
    assert!(!s.is_null());

    let out = ffi_try!(contract_id.to_rust().to_solidity_address());
    let out = CString::new(out).unwrap().into_raw();

    // safety: invariants promise that `buf` must be valid for writes.
    unsafe {
        ptr::write(s, out);
    }

    Error::Ok
}
