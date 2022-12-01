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
use prost::Message;
use time::{
    Duration,
    OffsetDateTime,
};

use crate::protobuf::ToProtobuf;
use crate::token::custom_fees::CustomFee;
use crate::{
    AccountId,
    FromProtobuf,
    Key,
    LedgerId,
    TokenId,
    TokenSupplyType,
    TokenType,
};

/// Response from [`TokenInfoQuery`][crate::TokenInfoQuery].
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
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
    pub auto_renew_account: Option<AccountId>,

    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::DurationSeconds<i64>>>")
    )]
    pub auto_renew_period: Option<Duration>,

    /// The epoch second at which the token will expire
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    pub expiration_time: Option<OffsetDateTime>,

    /// The memo associated with the token
    pub token_memo: String,

    /// The token type.
    pub token_type: TokenType,

    /// The token supply type
    pub supply_type: TokenSupplyType,

    /// The Maximum number of tokens that can be in circulation.
    pub max_supply: u64,

    /// The custom fees to be assessed during a transfer that transfers units of this token.
    pub custom_fees: Vec<CustomFee>,

    /// The Key which can pause and unpause the Token.
    pub pause_key: Option<Key>,

    /// Specifies whether the token is paused or not.
    pub pause_status: Option<bool>,

    /// The ledger ID the response was returned from.
    pub ledger_id: LedgerId,
}

impl TokenInfo {
    /// Create a new `TokenInfo` from protobuf-encoded `bytes`.
    ///
    /// # Errors
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the bytes fails to produce a valid protobuf.
    /// - [`Error::FromProtobuf`](crate::Error::FromProtobuf) if decoding the protobuf fails.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        FromProtobuf::<services::TokenInfo>::from_bytes(bytes)
    }

    /// Convert `self` to a protobuf-encoded [`Vec<u8>`].
    #[must_use]
    pub fn to_bytes(&self) -> Vec<u8> {
        let default_freeze_status = match self.default_freeze_status {
            Some(true) => TokenFreezeStatus::Frozen as i32,
            Some(false) => TokenFreezeStatus::Unfrozen as i32,
            None => TokenFreezeStatus::FreezeNotApplicable as i32,
        };

        let default_kyc_status = match self.default_kyc_status {
            Some(true) => TokenKycStatus::Granted as i32,
            Some(false) => TokenKycStatus::Revoked as i32,
            None => TokenKycStatus::KycNotApplicable as i32,
        };

        services::TokenInfo {
            token_id: Some(self.token_id.to_protobuf()),
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            decimals: self.decimals,
            total_supply: self.total_supply,
            treasury: Some(self.treasury_account_id.to_protobuf()),
            admin_key: self.admin_key.to_protobuf(),
            kyc_key: self.kyc_key.to_protobuf(),
            freeze_key: self.freeze_key.to_protobuf(),
            wipe_key: self.wipe_key.to_protobuf(),
            supply_key: self.supply_key.to_protobuf(),
            default_freeze_status,
            default_kyc_status,
            deleted: self.is_deleted,
            auto_renew_account: self.auto_renew_account.to_protobuf(),
            auto_renew_period: self.auto_renew_period.to_protobuf(),
            expiry: self.expiration_time.to_protobuf(),
            memo: self.token_memo.clone(),
            token_type: self.token_type.to_protobuf() as i32,
            supply_type: self.supply_type.to_protobuf() as i32,
            max_supply: self.max_supply as i64,
            fee_schedule_key: self.fee_schedule_key.to_protobuf(),
            custom_fees: self.custom_fees.to_protobuf(),
            pause_key: self.pause_key.to_protobuf(),
            pause_status: match self.pause_status {
                Some(true) => TokenPauseStatus::Paused as i32,
                Some(false) => TokenPauseStatus::Unpaused as i32,
                None => TokenPauseStatus::PauseNotApplicable as i32,
            },
            ledger_id: self.ledger_id.to_bytes(),
        }
        .encode_to_vec()
    }
}

impl FromProtobuf<services::response::Response> for TokenInfo {
    #[allow(deprecated)]
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, TokenGetInfo, services::response::Response);
        let info = pb_getf!(response, token_info)?;
        Self::from_protobuf(info)
    }
}

impl FromProtobuf<services::TokenInfo> for TokenInfo {
    fn from_protobuf(pb: services::TokenInfo) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let token_type = TokenType::from_protobuf(pb.token_type())?;
        let token_supply_type = TokenSupplyType::from_protobuf(pb.supply_type())?;
        let token_id = pb_getf!(pb, token_id)?;

        let default_kyc_status = match pb.default_kyc_status() {
            TokenKycStatus::KycNotApplicable => None,
            TokenKycStatus::Granted => Some(true),
            TokenKycStatus::Revoked => Some(false),
        };

        let default_freeze_status = match pb.default_freeze_status() {
            TokenFreezeStatus::FreezeNotApplicable => None,
            TokenFreezeStatus::Frozen => Some(true),
            TokenFreezeStatus::Unfrozen => Some(false),
        };

        let pause_status = match pb.pause_status() {
            TokenPauseStatus::PauseNotApplicable => None,
            TokenPauseStatus::Paused => Some(true),
            TokenPauseStatus::Unpaused => Some(false),
        };

        let auto_renew_account_id = Option::from_protobuf(pb.auto_renew_account)?;

        let treasury_account_id = pb_getf!(pb, treasury)?;
        let ledger_id = LedgerId::from_bytes(pb.ledger_id);

        Ok(Self {
            token_id: TokenId::from_protobuf(token_id)?,
            name: pb.name,
            symbol: pb.symbol,
            decimals: pb.decimals,
            total_supply: pb.total_supply,
            treasury_account_id: AccountId::from_protobuf(treasury_account_id)?,
            admin_key: Option::from_protobuf(pb.admin_key)?,
            kyc_key: Option::from_protobuf(pb.kyc_key)?,
            freeze_key: Option::from_protobuf(pb.freeze_key)?,
            wipe_key: Option::from_protobuf(pb.wipe_key)?,
            supply_key: Option::from_protobuf(pb.supply_key)?,
            default_freeze_status,
            default_kyc_status,
            is_deleted: pb.deleted,
            auto_renew_account: auto_renew_account_id,
            auto_renew_period: pb.auto_renew_period.map(Into::into),
            expiration_time: pb.expiry.map(Into::into),
            token_memo: pb.memo,
            token_type,
            supply_type: token_supply_type,
            max_supply: pb.max_supply as u64,
            fee_schedule_key: Option::from_protobuf(pb.fee_schedule_key)?,
            custom_fees: Vec::from_protobuf(pb.custom_fees)?, //test this
            pause_key: Option::from_protobuf(pb.pause_key)?,
            pause_status,
            ledger_id,
        })
    }
}
