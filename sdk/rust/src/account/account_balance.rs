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

use hedera_proto::services;

use crate::{
    AccountId,
    FromProtobuf,
    Hbar,
    Tinybar,
};

/// Response from [`AccountBalanceQuery`][crate::AccountBalanceQuery].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    /// The account that is being referenced.
    pub account_id: AccountId,

    /// Current balance of the referenced account.
    pub hbars: Hbar,
}

impl FromProtobuf<services::response::Response> for AccountBalance {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self> {
        let response = pb_getv!(pb, CryptogetAccountBalance, services::response::Response);

        let account_id = pb_getf!(response, account_id)?;
        let account_id = AccountId::from_protobuf(account_id)?;

        let balance = Hbar::from_tinybars(response.balance as Tinybar);

        Ok(Self { account_id, hbars: balance })
    }
}
