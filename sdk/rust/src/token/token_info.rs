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
use hedera_proto::services::{
    TokenFreezeStatus,
    TokenKycStatus,
    TokenPauseStatus,
};
use time::{
    Duration,
    OffsetDateTime,
};

use crate::token::custom_fees::CustomFee;
use crate::{
    AccountId,
    FromProtobuf,
    Key,
    TokenId,
    TokenSupplyType,
    TokenType,
};

// TODO: pub ledger_id: Vec<u8>,
/// Response from [`TokenInfoQuery`][crate::TokenInfoQuery].
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    /// The ID of the token for which information is requested.
    pub token_id: TokenId,

    /// Name of token.
    pub name: String,

    /// Symbol of token.
    pub symbol: String,

    /// The amount of decimal places that this token supports.
    pub decimals: u32,

    /// Total Supply of token.
    pub total_supply: u64,

    /// The ID of the account which is set as Treasury.
    pub treasury_account_id: AccountId,

    /// The key which can perform update/delete operations on the token.
    pub admin_key: Option<Key>,

    /// The key which can grant or revoke KYC of an account for the token's transactions.
    pub kyc_key: Option<Key>,

    /// The key which can freeze or unfreeze an account for token transactions.
    pub freeze_key: Option<Key>,

    /// The key which can wipe token balance of an account.
    pub wipe_key: Option<Key>,

    /// The key which can change the supply of a token.
    pub supply_key: Option<Key>,

    /// The key which can change the custom fees of the token.
    pub fee_schedule_key: Option<Key>,

    /// The default Freeze status (not applicable, frozen or unfrozen)
    pub default_freeze_status: Option<bool>,

    /// The default KYC status (KycNotApplicable or Revoked) of Hedera accounts relative to this token.
    pub default_kyc_status: Option<bool>,

    /// Specifies whether the token was deleted or not.
    pub is_deleted: bool,

    /// An account which will be automatically charged to renew the token's expiration,
    /// at autoRenewPeriod interval.
    pub auto_renew_account_id: Option<AccountId>,

    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    pub auto_renew_period: Option<Duration>,

    /// The epoch second at which the token will expire
    pub expiration_time: Option<OffsetDateTime>,

    /// The memo associated with the token
    pub token_memo: String,

    /// The token type.
    pub token_type: TokenType,

    /// The token supply type
    pub token_supply_type: TokenSupplyType,

    /// The Maximum number of tokens that can be in circulation.
    pub max_supply: u64,

    /// The custom fees to be assessed during a transfer that transfers units of this token.
    pub custom_fees: Vec<CustomFee>,

    /// The Key which can pause and unpause the Token.
    pub pause_key: Option<Key>,

    /// Specifies whether the token is paused or not.
    pub pause_status: Option<bool>,
}

impl FromProtobuf<services::response::Response> for TokenInfo {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, TokenGetInfo, services::response::Response);
        let info = pb_getf!(response, token_info)?;
        let token_type = TokenType::from_protobuf(info.token_type())?;
        let token_supply_type = TokenSupplyType::from_protobuf(info.supply_type())?;
        let token_id = pb_getf!(info, token_id)?;
        let auto_renew_account_id =
            info.auto_renew_account.clone().map(AccountId::from_protobuf).transpose()?;

        let default_kyc_status = match info.default_kyc_status() {
            TokenKycStatus::KycNotApplicable => None,
            TokenKycStatus::Granted => Some(true),
            TokenKycStatus::Revoked => Some(false),
        };

        let default_freeze_status = match info.default_freeze_status() {
            TokenFreezeStatus::FreezeNotApplicable => None,
            TokenFreezeStatus::Frozen => Some(true),
            TokenFreezeStatus::Unfrozen => Some(false),
        };

        let pause_status = match info.pause_status() {
            TokenPauseStatus::PauseNotApplicable => None,
            TokenPauseStatus::Paused => Some(true),
            TokenPauseStatus::Unpaused => Some(false),
        };

        let treasury_account_id = pb_getf!(info, treasury)?;

        Ok(Self {
            token_id: TokenId::from_protobuf(token_id)?,
            name: info.name,
            symbol: info.symbol,
            decimals: info.decimals as u32,
            total_supply: info.total_supply as u64,
            treasury_account_id: AccountId::from_protobuf(treasury_account_id)?,
            admin_key: info.admin_key.map(Key::from_protobuf).transpose()?,
            kyc_key: info.kyc_key.map(Key::from_protobuf).transpose()?,
            freeze_key: info.freeze_key.map(Key::from_protobuf).transpose()?,
            wipe_key: info.wipe_key.map(Key::from_protobuf).transpose()?,
            supply_key: info.supply_key.map(Key::from_protobuf).transpose()?,
            default_freeze_status,
            default_kyc_status,
            is_deleted: info.deleted,
            auto_renew_account_id,
            auto_renew_period: info.auto_renew_period.map(Into::into),
            expiration_time: info.expiry.map(Into::into),
            token_memo: info.memo,
            token_type,
            token_supply_type,
            max_supply: info.max_supply as u64,
            fee_schedule_key: info.fee_schedule_key.map(Key::from_protobuf).transpose()?,
            custom_fees: info
                .custom_fees
                .into_iter()
                .map(CustomFee::from_protobuf)
                .collect::<Result<Vec<_>, _>>()?, //test this
            pause_key: info.pause_key.map(Key::from_protobuf).transpose()?,
            pause_status,
        })
    }
}
