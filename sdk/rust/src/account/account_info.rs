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
use time::{
    Duration,
    OffsetDateTime,
};

use crate::{
    AccountId,
    FromProtobuf,
    Hbar,
    Key,
    LedgerId,
    PublicKey,
    StakingInfo,
    Tinybar,
};

/// Response from [`AccountInfoQuery`][crate::AccountInfoQuery].
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// The account that is being referenced.
    pub account_id: AccountId,

    /// The Contract Account ID comprising of both the contract instance and the cryptocurrency
    /// account owned by the contract instance, in the format used by Solidity.
    pub contract_account_id: String,

    /// If true, then this account has been deleted, it will disappear when it expires, and all
    /// transactions for it will fail except the transaction to extend its expiration date.
    pub is_deleted: bool,

    /// The total number of hbars proxy staked to this account.
    pub proxy_received: Hbar,

    /// The key for the account, which must sign in order to transfer out, or to modify the
    /// account in any way other than extending its expiration date.
    pub key: Key,

    /// Current balance of the referenced account.
    pub balance: Hbar,

    /// If true, no transaction can transfer to this account unless signed by
    /// this account's key.
    pub is_receiver_signature_required: bool,

    /// The TimeStamp time at which this account is set to expire.
    pub expiration_time: Option<OffsetDateTime>,

    /// The duration for expiration time will extend every this many seconds.
    pub auto_renew_period: Option<Duration>,

    /// The memo associated with the account.
    pub account_memo: String,

    /// The number of NFTs owned by this account
    pub owned_nfts: u64,

    /// The maximum number of tokens that an Account can be implicitly associated with.
    pub max_automatic_token_associations: u32,

    /// The alias of this account.
    pub alias_key: Option<PublicKey>,

    /// The ethereum transaction nonce associated with this account.
    pub ethereum_nonce: u64,

    /// The ledger ID the response was returned from.
    pub ledger_id: LedgerId,

    /// Staking metadata for this account.
    pub staking: Option<StakingInfo>,
}

impl FromProtobuf<services::response::Response> for AccountInfo {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, CryptoGetInfo, services::response::Response);
        let info = pb_getf!(response, account_info)?;
        let key = pb_getf!(info, key)?;
        let account_id = pb_getf!(info, account_id)?;
        let alias_key = PublicKey::from_alias_bytes(&info.alias)?;
        let ledger_id = LedgerId::from_bytes(info.ledger_id);
        let staking = info.staking_info.map(StakingInfo::from_protobuf).transpose()?;

        Ok(Self {
            ledger_id,
            staking,
            account_id: AccountId::from_protobuf(account_id)?,
            contract_account_id: info.contract_account_id,
            is_deleted: info.deleted,
            proxy_received: Hbar::from_tinybars(info.proxy_received),
            key: Key::from_protobuf(key)?,
            balance: Hbar::from_tinybars(info.balance as Tinybar),
            expiration_time: info.expiration_time.map(Into::into),
            auto_renew_period: info.auto_renew_period.map(Into::into),
            account_memo: info.memo,
            owned_nfts: info.owned_nfts as u64,
            max_automatic_token_associations: info.max_automatic_token_associations as u32,
            alias_key,
            ethereum_nonce: info.ethereum_nonce as u64,
            is_receiver_signature_required: info.receiver_sig_required,
        })
    }
}
