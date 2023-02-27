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

use std::ptr;

use libc::size_t;

use super::util;
use crate::{
    Client,
    LedgerId,
    PrivateKey,
};

/// Construct a Hedera client pre-configured for mainnet access.
#[no_mangle]
pub extern "C" fn hedera_client_for_mainnet() -> *mut Client {
    let client = Client::for_mainnet();

    Box::into_raw(Box::new(client))
}

/// Construct a Hedera client pre-configured for testnet access.
#[no_mangle]
pub extern "C" fn hedera_client_for_testnet() -> *mut Client {
    let client = Client::for_testnet();

    Box::into_raw(Box::new(client))
}

/// Construct a Hedera client pre-configured for previewnet access.
#[no_mangle]
pub extern "C" fn hedera_client_for_previewnet() -> *mut Client {
    let client = Client::for_previewnet();

    Box::into_raw(Box::new(client))
}

/// Sets the account that will, by default, be paying for transactions and queries built with
/// this client.
#[no_mangle]
pub unsafe extern "C" fn hedera_client_set_operator(
    client: *mut Client,
    id: super::AccountId,
    key: *mut PrivateKey,
) {
    let client = unsafe { client.as_ref() }.unwrap();
    let key = unsafe { key.as_ref() }.unwrap();

    let key = key.clone();

    client.set_operator(id.into(), key);
}

/// Returns `true` if there was an operator and `false` if there wasn't.
///
/// If this method returns `false`, variables will not be modified.
#[no_mangle]
pub unsafe extern "C" fn hedera_client_get_operator(
    client: *mut Client,
    id_out: *mut super::AccountId,
    key_out: *mut *mut PrivateKey,
) -> bool {
    let client = unsafe { client.as_ref() }.unwrap();
    assert!(!id_out.is_null());
    assert!(!key_out.is_null());

    match client.operator_internal().as_deref().cloned() {
        Some(it) => {
            unsafe {
                key_out.write(Box::leak(Box::new(it.signer)) as *mut PrivateKey);
                id_out.write(it.account_id.into())
            }

            true
        }

        None => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_get_max_transaction_fee(client: *mut Client) -> u64 {
    let client = unsafe { client.as_ref() }.unwrap();

    client.max_transaction_fee().load(std::sync::atomic::Ordering::Relaxed)
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_get_random_node_ids(
    client: *mut Client,
    ids: *mut *mut super::AccountId,
) -> size_t {
    assert!(!ids.is_null());
    let client = unsafe { client.as_ref() }.unwrap();

    let buf: Vec<_> = client.random_node_ids().into_iter().map(Into::into).collect();

    let buf = buf.into_boxed_slice();

    let buf = Box::leak(buf);
    let size = buf.len();

    // safety `ids` must be valid for writes.
    unsafe { ids.write(buf.as_mut_ptr()) };

    size
}

/// Get all the nodes for the `Client`
///
/// For internal use _only_.
///
/// # Safety:
/// - `Client` must be valid for reads.
/// - `ids` must be freed by using `hedera_account_id_array_free`, notably this means that it must *not* be freed with `free`.
/// - the length of `ids` must not be changed.
#[no_mangle]
pub unsafe extern "C" fn hedera_client_get_nodes(
    client: *mut Client,
    ids: *mut *mut super::AccountId,
) -> size_t {
    assert!(!ids.is_null());
    let client = unsafe { client.as_ref() }.unwrap();

    let buf: Vec<_> =
        client.network().node_ids().iter().copied().map(super::AccountId::from).collect();

    let buf = buf.into_boxed_slice();

    let buf = Box::leak(buf);
    let size = buf.len();

    // safety `ids` must be valid for writes.
    unsafe { ids.write(buf.as_mut_ptr()) };

    size
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_set_ledger_id(
    client: *mut Client,
    ledger_id_bytes: *const u8,
    ledger_id_size: size_t,
) {
    let client = unsafe { client.as_ref() }.unwrap();

    let ledger_id_bytes = match ledger_id_bytes.is_null() {
        true => None,
        false => {
            Some(unsafe { std::slice::from_raw_parts(ledger_id_bytes, ledger_id_size) }.to_vec())
        }
    };

    let ledger_id = ledger_id_bytes.map(LedgerId::from_bytes);

    client.set_ledger_id(ledger_id);
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_get_ledger_id(
    client: *mut Client,
    ledger_id_bytes: *mut *mut u8,
) -> size_t {
    let client = unsafe { client.as_ref() }.unwrap();

    let ledger_id = match &*client.ledger_id_internal() {
        Some(it) => it.clone(),
        None => {
            unsafe { ptr::write(ledger_id_bytes, ptr::null_mut()) };
            return 0;
        }
    };

    unsafe { util::make_bytes(ledger_id.to_bytes(), ledger_id_bytes) }
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_set_auto_validate_checksums(
    client: *mut Client,
    auto_validate_checksums: bool,
) {
    let client = unsafe { client.as_ref() }.unwrap();

    client.set_auto_validate_checksums(auto_validate_checksums)
}

#[no_mangle]
pub unsafe extern "C" fn hedera_client_get_auto_validate_checksums(client: *mut Client) -> bool {
    let client = unsafe { client.as_ref() }.unwrap();

    client.auto_validate_checksums()
}

/// Release memory associated with the previously-opened Hedera client.
#[no_mangle]
pub unsafe extern "C" fn hedera_client_free(client: *mut Client) {
    assert!(!client.is_null());

    let _client = unsafe { Box::from_raw(client) };
}
