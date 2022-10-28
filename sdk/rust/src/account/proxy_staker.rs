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
};

/// Response from [`AccountStakersQuery`][crate::AccountStakersQuery].
pub type AllProxyStakers = Vec<ProxyStaker>;

/// Information about a single account that is proxy staking.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct ProxyStaker {
    /// The Account ID that is proxy staking.
    pub account_id: AccountId,

    /// The number of hbars that are currently proxy staked.
    pub amount: Hbar,
}

impl FromProtobuf<services::response::Response> for AllProxyStakers {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, CryptoGetProxyStakers, services::response::Response);
        let stakers = pb_getf!(response, stakers)?;

        AllProxyStakers::from_protobuf(stakers.proxy_staker)
    }
}

impl FromProtobuf<services::ProxyStaker> for ProxyStaker {
    fn from_protobuf(pb: services::ProxyStaker) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let account_id = pb_getf!(pb, account_id)?;

        Ok(Self {
            account_id: AccountId::from_protobuf(account_id)?,
            amount: Hbar::from_tinybars(pb.amount),
        })
    }
}
