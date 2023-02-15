use std::{
    ptr,
    slice,
};

use libc::size_t;

use crate::ffi;
use crate::ffi::error::Error;
use crate::protobuf::ToProtobuf;

#[repr(C)]
pub struct TokenBalance {
    id_shard: u64,
    id_realm: u64,
    id_num: u64,

    amount: u64,
    decimals: u32,
}

#[repr(C)]
pub struct AccountBalance {
    id: ffi::AccountId,
    hbars: i64,

    token_balances: *const TokenBalance,
    token_balances_len: size_t,
}

impl AccountBalance {
    fn borrow_ref(&self) -> RefAccountBalance<'_> {
        // note: `token_balances` is *technically* `&'static` if it came from us, but it could be `&'a` if it came from swift.
        // safety: immediate library UB to have invalid token_balances/token_balances_len.
        let token_balances =
            unsafe { slice::from_raw_parts(self.token_balances, self.token_balances_len) };
        RefAccountBalance { id: self.id.borrow_ref(), hbars: self.hbars, token_balances }
    }
}

impl From<crate::AccountBalance> for AccountBalance {
    #[allow(deprecated)]
    fn from(it: crate::AccountBalance) -> Self {
        let token_balances: Vec<_> = it
            .tokens
            .iter()
            .map(|(&id, &balance)| (id, balance, it.token_decimals[&id]))
            .map(|(id, amount, decimals)| TokenBalance {
                id_shard: id.shard,
                id_realm: id.realm,
                id_num: id.num,
                amount,
                decimals,
            })
            .collect();

        let token_balances_len = token_balances.len();

        let token_balances = Box::leak(token_balances.into_boxed_slice()).as_ptr();

        Self {
            id: it.account_id.into(),
            hbars: it.hbars.to_tinybars(),
            token_balances,
            token_balances_len,
        }
    }
}

struct RefAccountBalance<'a> {
    id: ffi::RefAccountId<'a>,
    hbars: i64,
    token_balances: &'a [TokenBalance],
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
            token_balances: self
                .token_balances
                .iter()
                .map(|it| services::TokenBalance {
                    token_id: Some(
                        crate::TokenId::new(it.id_shard, it.id_realm, it.id_num).to_protobuf(),
                    ),
                    balance: it.amount,
                    decimals: it.decimals,
                })
                .collect(),
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

    let parsed = ffi_try!(crate::AccountBalance::from_bytes(bytes)).into();

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

/// Free an array of `TokenBalance`s.
///
/// # Safety
/// - `token_balances` must point to an allocation made by `hedera`.
/// - `token_balances` must not already have been freed.
/// - `token_balances` must be valid for `size` elements.
#[no_mangle]
pub unsafe extern "C" fn hedera_account_balance_token_balances_free(
    token_balances: *mut TokenBalance,
    size: size_t,
) {
    assert!(!token_balances.is_null());

    // safety: function contract promises that we own this `Box<[TokenBalance]>`.
    let buf = unsafe {
        let token_balances = slice::from_raw_parts_mut(token_balances, size);
        Box::from_raw(token_balances)
    };

    drop(buf);
}
