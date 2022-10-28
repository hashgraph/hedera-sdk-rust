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

use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use time::{
    Duration,
    OffsetDateTime,
};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Key,
    TokenId,
    Transaction,
    TransactionId,
};

/// At consensus, updates an already created token to the given values.
///
/// If no value is given for a field, that field is left unchanged. For an immutable token (that is,
/// a token without an admin key), only the expiry may be updated. Setting any other field in that
/// case will cause the transaction status to resolve to `TokenIsImmutable`.
///
/// ### --- Signing Requirements ---
/// 1. Whether or not a token has an admin key, its expiry can be extended with only the transaction
///    payer's signature.
/// 2. Updating any other field of a mutable token requires the admin key's signature.
/// 3. If a new admin key is set, this new key must sign **unless** it is exactly an empty
///    `KeyList`. This special sentinel key removes the existing admin key and causes the
///    token to become immutable. (Other [`Key`][Key] structures without a constituent
///    `Ed25519` key will be rejected with `InvalidAdminKey`.
/// 4. If a new treasury is set, the new treasury account's key must sign the transaction.
///
/// ### --- Nft Requirements ---
/// 1. If a non fungible token has a positive treasury balance, the operation will abort with
///    `CurrentTreasuryStillOwnsNfts`.
pub type TokenUpdateTransaction = Transaction<TokenUpdateTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct TokenUpdateTransactionData {
    /// The token to be updated.
    token_id: Option<TokenId>,

    /// The publicly visible name of the token.
    name: String,

    /// The publicly visible token symbol.
    symbol: String,

    /// The account which will act as a treasury for the token.
    treasury_account_id: Option<AccountId>,

    /// The key which can perform update/delete operations on the token.
    admin_key: Option<Key>,

    /// The key which can grant or revoke KYC of an account for the token's transactions.
    kyc_key: Option<Key>,

    /// The key which can sign to freeze or unfreeze an account for token transactions.
    freeze_key: Option<Key>,

    /// The key which can wipe the token balance of an account.
    wipe_key: Option<Key>,

    /// The key which can change the supply of a token.
    supply_key: Option<Key>,

    /// An account which will be automatically charged to renew the token's expiration, at
    /// autoRenewPeriod interval
    auto_renew_account_id: Option<AccountId>,

    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::DurationSeconds<i64>>>")
    )]
    auto_renew_period: Option<Duration>,

    /// Sets the time at which the token should expire.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    expiration_time: Option<OffsetDateTime>,

    /// The memo associated with the token (UTF-8 encoding max 100 bytes)
    token_memo: String,

    /// The key which can change the token's custom fee schedule; must sign a TokenFeeScheduleUpdate
    /// transaction
    fee_schedule_key: Option<Key>,

    /// The Key which can pause and unpause the Token.
    /// If Empty the token pause status defaults to PauseNotApplicable, otherwise Unpaused.
    pause_key: Option<Key>,
}

impl TokenUpdateTransaction {
    /// Sets the token to be updated.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.body.data.token_id = Some(token_id.into());
        self
    }

    /// Sets the new publicly visible name of the token.
    /// Maximum 100 characters.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.body.data.name = name.into();
        self
    }

    /// Sets the new publicly visible token symbol.
    /// Maximum 100 characters.
    pub fn symbol(&mut self, symbol: impl Into<String>) -> &mut Self {
        self.body.data.symbol = symbol.into();
        self
    }

    /// Sets the new account which will act as a treasury for the token.
    ///
    /// If the provided `treasury_account_id` does not exist or has been deleted, the response
    /// will be `InvalidTreasuryAccountForToken`.
    ///
    /// If successful, the token balance held in the previous treasury account is transferred to the
    /// new one.
    pub fn treasury_account_id(&mut self, treasury_account_id: AccountId) -> &mut Self {
        self.body.data.treasury_account_id = Some(treasury_account_id);
        self
    }

    /// Sets the new key which can perform update/delete operations on the token.
    ///
    /// If the token is immutable, transaction will resolve to `TokenIsImmutable`.
    pub fn admin_key(&mut self, admin_key: impl Into<Key>) -> &mut Self {
        self.body.data.admin_key = Some(admin_key.into());
        self
    }

    /// Sets the new key which can grant or revoke KYC of an account for the token's transactions.
    ///
    /// If the token does not currently have a KYC key, transaction will resolve to `TokenHasNoKycKey`.
    pub fn kyc_key(&mut self, kyc_key: impl Into<Key>) -> &mut Self {
        self.body.data.kyc_key = Some(kyc_key.into());
        self
    }

    /// Sets the new key which can sign to freeze or unfreeze an account for token transactions.
    ///
    /// If the token does not currently have a Freeze key, transaction will resolve to `TokenHasNoFreezeKey`.
    pub fn freeze_key(&mut self, freeze_key: impl Into<Key>) -> &mut Self {
        self.body.data.freeze_key = Some(freeze_key.into());
        self
    }

    /// Sets the new key which can wipe the token balance of an account.
    ///
    /// If the token does not currently have a Wipe key, transaction will resolve to `TokenHasNoWipeKey`.
    pub fn wipe_key(&mut self, wipe_key: impl Into<Key>) -> &mut Self {
        self.body.data.wipe_key = Some(wipe_key.into());
        self
    }

    /// Sets the new key which can change the supply of a token.
    ///
    /// If the token does not currently have a Supply key, transaction will resolve to `TokenHasNoSupplyKey`.
    pub fn supply_key(&mut self, supply_key: impl Into<Key>) -> &mut Self {
        self.body.data.supply_key = Some(supply_key.into());
        self
    }

    /// Sets the new account which will be automatically charged to renew the token's expiration.
    pub fn auto_renew_account_id(&mut self, auto_renew_account_id: AccountId) -> &mut Self {
        self.body.data.auto_renew_account_id = Some(auto_renew_account_id);
        self
    }

    /// Sets the new interval at which the auto renew account will be charged to extend
    /// the token's expiry.
    pub fn auto_renew_period(&mut self, auto_renew_period: Duration) -> &mut Self {
        self.body.data.auto_renew_period = Some(auto_renew_period);
        self
    }

    /// Sets the new time at which the token should expire.
    ///
    /// If the new expiration time is earlier than the current expiration time, transaction
    /// will resolve to `InvalidExpirationTime`.
    pub fn expiration_time(&mut self, expiration_time: OffsetDateTime) -> &mut Self {
        self.body.data.expiration_time = Some(expiration_time);
        self.body.data.auto_renew_period = None;

        self
    }

    /// Sets the new memo associated with the token (UTF-8 encoding max 100 bytes).
    pub fn token_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.token_memo = memo.into();
        self
    }

    /// Sets the new key which can change the token's custom fee schedule.
    ///
    /// If the token does not currently have a fee schedule key, transaction will resolve to
    /// `TokenHasNoFeeScheduleKey`.
    pub fn fee_schedule_key(&mut self, fee_schedule_key: impl Into<Key>) -> &mut Self {
        self.body.data.fee_schedule_key = Some(fee_schedule_key.into());
        self
    }

    /// Sets the new key which can pause and unpause the Token.
    ///
    /// If the token does not currently have a pause key, transaction will resolve to `TokenHasNoPauseKey`.
    pub fn pause_key(&mut self, pause_key: impl Into<Key>) -> &mut Self {
        self.body.data.pause_key = Some(pause_key.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenUpdateTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).update_token(request).await
    }
}

impl ToTransactionDataProtobuf for TokenUpdateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        services::transaction_body::Data::TokenUpdate(services::TokenUpdateTransactionBody {
            token: self.token_id.as_ref().map(TokenId::to_protobuf),
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            treasury: self.treasury_account_id.as_ref().map(AccountId::to_protobuf),
            admin_key: self.admin_key.as_ref().map(Key::to_protobuf),
            kyc_key: self.kyc_key.as_ref().map(Key::to_protobuf),
            freeze_key: self.freeze_key.as_ref().map(Key::to_protobuf),
            wipe_key: self.wipe_key.as_ref().map(Key::to_protobuf),
            supply_key: self.supply_key.as_ref().map(Key::to_protobuf),
            expiry: self.expiration_time.map(Into::into),
            auto_renew_account: self.auto_renew_account_id.as_ref().map(AccountId::to_protobuf),
            auto_renew_period: self.auto_renew_period.map(Into::into),
            memo: Some(self.token_memo.clone()),
            fee_schedule_key: self.fee_schedule_key.as_ref().map(Key::to_protobuf),
            pause_key: self.pause_key.as_ref().map(Key::to_protobuf),
        })
    }
}

impl From<TokenUpdateTransactionData> for AnyTransactionData {
    fn from(transaction: TokenUpdateTransactionData) -> Self {
        Self::TokenUpdate(transaction)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "ffi")]
    mod ffi {
        use std::str::FromStr;

        use assert_matches::assert_matches;
        use time::{
            Duration,
            OffsetDateTime,
        };

        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            AccountId,
            Key,
            PublicKey,
            TokenId,
            TokenUpdateTransaction,
        };

        // language=JSON
        const TOKEN_UPDATE_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenUpdate",
  "tokenId": "0.0.1001",
  "name": "Pound",
  "symbol": "LB",
  "treasuryAccountId": "0.0.1002",
  "adminKey": {
    "single": "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd"
  },
  "kycKey": {
    "single": "302a300506032b6570032100b5b4d9351ebdf266ef3989aed4fd8f0cfcf24b75ba3d0df19cd3946771b40500"
  },
  "freezeKey": {
    "single": "302a300506032b657003210004e540b5fba8fc1ee1cc5cc450019c578b36311733507fabf4f85bf2744583e7"
  },
  "wipeKey": {
    "single": "302a300506032b657003210099f8981cad75fc7322bf5c89d5f4ce4f2af76b2a63780b22cbce1bfdfa237f4e"
  },
  "supplyKey": {
    "single": "302a300506032b6570032100c80c04aaca1783aafbaf6eba462bac89236ec82ac4db31953329ffbfeacdb88b"
  },
  "autoRenewAccountId": "0.0.1003",
  "autoRenewPeriod": 7776000,
  "expirationTime": 1656352251277559886,
  "tokenMemo": "A new memo",
  "feeScheduleKey": {
    "single": "302a300506032b65700321000cd029bfd4a818de944c21799f4b5f6b5616702d0495520c818d92488e5395fc"
  },
  "pauseKey": {
    "single": "302a300506032b65700321008b020177031eae1e4a721c814b08a3ef2c3f473781a570e9daaf9f7ad27f8967"
  }
}"#;

        const ADMIN_KEY: &str =
        "302a300506032b6570032100d1ad76ed9b057a3d3f2ea2d03b41bcd79aeafd611f941924f0f6da528ab066fd";
        const KYC_KEY: &str =
        "302a300506032b6570032100b5b4d9351ebdf266ef3989aed4fd8f0cfcf24b75ba3d0df19cd3946771b40500";
        const FREEZE_KEY: &str =
        "302a300506032b657003210004e540b5fba8fc1ee1cc5cc450019c578b36311733507fabf4f85bf2744583e7";
        const WIPE_KEY: &str =
        "302a300506032b657003210099f8981cad75fc7322bf5c89d5f4ce4f2af76b2a63780b22cbce1bfdfa237f4e";
        const SUPPLY_KEY: &str =
        "302a300506032b6570032100c80c04aaca1783aafbaf6eba462bac89236ec82ac4db31953329ffbfeacdb88b";
        const FEE_SCHEDULE_KEY: &str =
        "302a300506032b65700321000cd029bfd4a818de944c21799f4b5f6b5616702d0495520c818d92488e5395fc";
        const PAUSE_KEY: &str =
        "302a300506032b65700321008b020177031eae1e4a721c814b08a3ef2c3f473781a570e9daaf9f7ad27f8967";

        #[test]
        fn it_should_serialize() -> anyhow::Result<()> {
            let mut transaction = TokenUpdateTransaction::new();

            transaction
                .token_id(TokenId::from(1001))
                .name("Pound")
                .symbol("LB")
                .treasury_account_id(AccountId::from(1002))
                .admin_key(PublicKey::from_str(ADMIN_KEY)?)
                .kyc_key(PublicKey::from_str(KYC_KEY)?)
                .freeze_key(PublicKey::from_str(FREEZE_KEY)?)
                .wipe_key(PublicKey::from_str(WIPE_KEY)?)
                .supply_key(PublicKey::from_str(SUPPLY_KEY)?)
                .expiration_time(OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?)
                .auto_renew_account_id(AccountId::from(1003))
                .auto_renew_period(Duration::days(90))
                .token_memo("A new memo")
                .fee_schedule_key(PublicKey::from_str(FEE_SCHEDULE_KEY)?)
                .pause_key(PublicKey::from_str(PAUSE_KEY)?);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOKEN_UPDATE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOKEN_UPDATE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.body.data, AnyTransactionData::TokenUpdate(transaction) => transaction);

            assert_eq!(data.token_id.unwrap(), TokenId::from(1001));
            assert_eq!(data.name, "Pound");
            assert_eq!(data.symbol, "LB");
            assert_eq!(data.auto_renew_period.unwrap(), Duration::days(90));
            assert_eq!(data.token_memo, "A new memo");
            assert_eq!(
                data.expiration_time.unwrap(),
                OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?
            );

            assert_eq!(data.treasury_account_id, Some(AccountId::from(1002)));
            assert_eq!(data.auto_renew_account_id, Some(AccountId::from(1003)));

            let admin_key =
                assert_matches!(data.admin_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(admin_key, PublicKey::from_str(ADMIN_KEY)?);

            let kyc_key =
                assert_matches!(data.kyc_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(kyc_key, PublicKey::from_str(KYC_KEY)?);

            let freeze_key =
                assert_matches!(data.freeze_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(freeze_key, PublicKey::from_str(FREEZE_KEY)?);

            let wipe_key =
                assert_matches!(data.wipe_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(wipe_key, PublicKey::from_str(WIPE_KEY)?);

            let supply_key =
                assert_matches!(data.supply_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(supply_key, PublicKey::from_str(SUPPLY_KEY)?);

            let fee_schedule_key = assert_matches!(data.fee_schedule_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(fee_schedule_key, PublicKey::from_str(FEE_SCHEDULE_KEY)?);

            let pause_key =
                assert_matches!(data.pause_key.unwrap(), Key::Single(public_key) => public_key);
            assert_eq!(pause_key, PublicKey::from_str(PAUSE_KEY)?);

            Ok(())
        }
    }
}
