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

use libc::size_t;

use crate::{
    AccountId,
    Client,
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

/// Release memory associated with the previously-opened Hedera client.
#[no_mangle]
pub unsafe extern "C" fn hedera_client_free(client: *mut Client) {
    assert!(!client.is_null());

    let _client = unsafe { Box::from_raw(client) };
}

/// Sets the account that will, by default, be paying for transactions and queries built with
/// this client.
#[no_mangle]
pub extern "C" fn hedera_client_set_operator(
    client: *mut Client,
    id_shard: u64,
    id_realm: u64,
    id_num: u64,
    key: *mut PrivateKey,
) {
    let client = unsafe { client.as_ref() }.unwrap();
    assert!(!key.is_null());

    let key = unsafe { &*key };
    let key = key.clone();

    client.set_operator(
        AccountId { shard: id_shard, realm: id_realm, num: id_num, alias: None, checksum: None },
        key,
    );
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
