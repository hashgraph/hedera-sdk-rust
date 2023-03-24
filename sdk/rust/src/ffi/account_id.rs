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

use crate::PublicKey;

#[repr(C)]
pub struct AccountId {
    shard: u64,
    realm: u64,
    num: u64,
    /// Safety:
    /// - If `alias` is not null, it must:
    ///   - be properly aligned
    ///   - be dereferenceable
    ///   - point to a valid instance of `PublicKey` (any `PublicKey` that `hedera` provides which hasn't been freed yet)
    alias: *mut PublicKey,

    /// Safety:
    /// - if `evm_address` is not null, it must:
    /// - be properly aligned
    /// - be dereferencable
    /// - point to an array of 20 bytes
    evm_address: *mut u8,
}

impl From<crate::AccountId> for AccountId {
    fn from(id: crate::AccountId) -> Self {
        Self {
            shard: id.shard,
            realm: id.realm,
            num: id.num,
            alias: id.alias.map(Box::new).map_or_else(ptr::null_mut, Box::into_raw),
            evm_address: id
                .evm_address
                .map(|it| it.to_bytes().to_vec().into_boxed_slice())
                .map_or_else(ptr::null_mut, |it| Box::leak(it).as_mut_ptr()),
        }
    }
}

impl From<AccountId> for crate::AccountId {
    fn from(value: AccountId) -> Self {
        // safety: invariants of self require a non-null `PublicKey` to follow the required invariants of `NonNull::as_ref`.
        let alias = unsafe { value.alias.as_ref() };
        // safety: invariants of self require a non-null `evm_address` to follow the required invariants of `NonNull::as_ref`.
        let evm_address = unsafe { value.alias.cast::<[u8; 20]>().as_ref() };

        crate::AccountId {
            shard: value.shard,
            realm: value.realm,
            num: value.num,
            alias: alias.copied(),
            evm_address: evm_address.copied().map(crate::EvmAddress),
            checksum: None,
        }
    }
}
