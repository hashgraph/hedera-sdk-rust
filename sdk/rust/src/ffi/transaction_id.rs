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

#[repr(C)]
pub struct TransactionId {
    account_id: super::AccountId,
    valid_start: super::Timestamp,
    nonce: i32,
    scheduled: bool,
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
