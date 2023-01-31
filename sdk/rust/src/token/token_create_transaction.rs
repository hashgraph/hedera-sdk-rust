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

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::token::custom_fees::AnyCustomFee;
use crate::token::token_supply_type::TokenSupplyType;
use crate::token::token_type::TokenType;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Error,
    Key,
    LedgerId,
    Transaction,
    TransactionId,
    ValidateChecksums,
};

/// Create a new token.
///
/// After the token is created, the [`TokenId`](crate::TokenId) for it is in the receipt.
///
/// The specified treasury account receives the initial supply of tokens, as well as the tokens
/// from a [`TokenMintTransaction`](crate::TokenMintTransaction) once executed.
/// The balance of the treasury account is decreased when a [`TokenBurnTransaction`](crate::TokenBurnTransaction) is executed.
///
/// The `initial_supply` is in the lowest denomination of the token (like a tinybar, not an hbar).
///
/// Note that a created token is __immutable__ if the `admin_key` is omitted. No property of
/// an immutable token can ever change, with the sole exception of its expiry. Anyone can pay to
/// extend the expiry time of an immutable token.
///
/// - If [`NonFungibleUnique`][TokenType::NonFungibleUnique] is used, the `initial_supply` should
/// explicitly be set to 0 (which is the default). If not, the transaction will
/// resolve to `InvalidTokenInitialSupply`.
///
/// - If [`Infinite`][TokenSupplyType::Infinite] is used, the `max_supply` should
/// explicitly be set to 0 (which is the default). If it is not 0,
/// the transaction will resolve to `InvalidTokenMaxSupply`.
///
pub type TokenCreateTransaction = Transaction<TokenCreateTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct TokenCreateTransactionData {
    /// The publicly visible name of the token.
    name: String,

    /// The publicly visible token symbol.
    symbol: String,

    /// The number of decimal places a fungible token is divisible by.
    decimals: u32,

    /// The initial supply of fungible tokens to to mint to the treasury account.
    initial_supply: u64,

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

    /// The default freeze status (frozen or unfrozen) of Hedera accounts relative to this token. If
    /// true, an account must be unfrozen before it can receive the token
    freeze_default: bool,

    /// The time at which the token should expire.
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    expiration_time: Option<OffsetDateTime>,

    /// An account which will be automatically charged to renew the token's expiration, at
    /// `auto_renew_period` interval.
    auto_renew_account_id: Option<AccountId>,

    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::DurationSeconds<i64>>>")
    )]
    auto_renew_period: Option<Duration>,

    /// The memo associated with the token.
    token_memo: String,

    /// The token type. Defaults to FungibleCommon.
    token_type: TokenType,

    /// The token supply type. Defaults to Infinite.
    token_supply_type: TokenSupplyType,

    /// Sets the maximum number of tokens that can be in circulation.
    max_supply: u64,

    /// The key which can change the token's custom fee schedule.
    fee_schedule_key: Option<Key>,

    /// The custom fees to be assessed during a transfer.
    custom_fees: Vec<AnyCustomFee>,

    /// The key which can pause and unpause the token.
    pause_key: Option<Key>,
}

impl Default for TokenCreateTransactionData {
    fn default() -> Self {
        Self {
            name: String::new(),
            symbol: String::new(),
            decimals: 0,
            initial_supply: 0,
            treasury_account_id: None,
            admin_key: None,
            kyc_key: None,
            freeze_key: None,
            wipe_key: None,
            supply_key: None,
            freeze_default: false,
            expiration_time: None,
            auto_renew_account_id: None,
            auto_renew_period: Some(Duration::days(90)),
            token_memo: String::new(),
            token_type: TokenType::FungibleCommon,
            token_supply_type: TokenSupplyType::Infinite,
            max_supply: 0,
            fee_schedule_key: None,
            custom_fees: vec![],
            pause_key: None,
        }
    }
}

impl TokenCreateTransaction {
    /// Returns the publicly visible name of the token.
    #[must_use]
    pub fn get_name(&self) -> &str {
        &self.data().name
    }

    /// Sets the publicly visible name of the token.
    ///
    /// Maximum 100 characters.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.data_mut().name = name.into();
        self
    }

    /// Returns the publicly visible token symbol.
    #[must_use]
    pub fn get_symbol(&self) -> &str {
        &self.data().symbol
    }

    /// Sets the publicly visible token symbol.
    ///
    /// Maximum 100 characters.
    pub fn symbol(&mut self, symbol: impl Into<String>) -> &mut Self {
        self.data_mut().symbol = symbol.into();
        self
    }

    /// Returns the number of decimal places the token is divisble by.
    #[must_use]
    pub fn get_decimals(&self) -> u32 {
        self.data().decimals
    }

    /// Sets the number of decimal places a token is divisible by.
    pub fn decimals(&mut self, decimals: u32) -> &mut Self {
        self.data_mut().decimals = decimals;
        self
    }

    /// Returns the initial supply of tokens to be put into circulation.
    #[must_use]
    pub fn get_initial_supply(&self) -> u64 {
        self.data().initial_supply
    }

    /// Sets the initial supply of tokens to be put in circulation.
    pub fn initial_supply(&mut self, initial_supply: u64) -> &mut Self {
        self.data_mut().initial_supply = initial_supply;
        self
    }

    /// Returns the account which will act as a treasury for the token.
    #[must_use]
    pub fn get_treasury_account_id(&self) -> Option<AccountId> {
        self.data().treasury_account_id
    }

    /// Sets the account which will act as a treasury for the token.
    pub fn treasury_account_id(&mut self, treasury_account_id: AccountId) -> &mut Self {
        self.data_mut().treasury_account_id = Some(treasury_account_id);
        self
    }

    /// Returns the key whcih can perform update/delete operations on the token.
    #[must_use]
    pub fn get_admin_key(&self) -> Option<&Key> {
        self.data().admin_key.as_ref()
    }

    /// Sets the key which can perform update/delete operations on the token.
    pub fn admin_key(&mut self, admin_key: impl Into<Key>) -> &mut Self {
        self.data_mut().admin_key = Some(admin_key.into());
        self
    }

    /// Returns the key which can grant or revoke KYC of an account for the token's transactions.
    #[must_use]
    pub fn get_kyc_key(&self) -> Option<&Key> {
        self.data().kyc_key.as_ref()
    }

    /// Sets the key which can grant or revoke KYC of an account for the token's transactions.
    pub fn kyc_key(&mut self, kyc_key: impl Into<Key>) -> &mut Self {
        self.data_mut().kyc_key = Some(kyc_key.into());
        self
    }

    /// Returns the key which can sign to freeze or unfreeze an account for token transactions.
    #[must_use]
    pub fn get_freeze_key(&self) -> Option<&Key> {
        self.data().freeze_key.as_ref()
    }

    /// Sets the key which can sign to freeze or unfreeze an account for token transactions.
    pub fn freeze_key(&mut self, freeze_key: impl Into<Key>) -> &mut Self {
        self.data_mut().freeze_key = Some(freeze_key.into());
        self
    }

    /// Returns the key which can wipe the token balance of an account.
    #[must_use]
    pub fn get_wipe_key(&self) -> Option<&Key> {
        self.data().wipe_key.as_ref()
    }

    /// Sets the key which can wipe the token balance of an account.
    pub fn wipe_key(&mut self, wipe_key: impl Into<Key>) -> &mut Self {
        self.data_mut().wipe_key = Some(wipe_key.into());
        self
    }

    /// Returns the key which can change the supply of the token.
    #[must_use]
    pub fn get_supply_key(&self) -> Option<&Key> {
        self.data().supply_key.as_ref()
    }

    /// Sets the key which can change the supply of the token.
    pub fn supply_key(&mut self, supply_key: impl Into<Key>) -> &mut Self {
        self.data_mut().supply_key = Some(supply_key.into());
        self
    }

    /// Returnsthe default freeze status (frozen or unfrozen) of hedera accounts
    /// relative to this token. If true, an account must be unfrozen before it can receive the token.
    #[must_use]
    pub fn get_freeze_default(&self) -> bool {
        self.data().freeze_default
    }

    /// Sets the default freeze status (frozen or unfrozen) of hedera accounts
    /// relative to this token. If true, an account must be unfrozen before it can receive the token.
    pub fn freeze_default(&mut self, freeze_default: bool) -> &mut Self {
        self.data_mut().freeze_default = freeze_default;
        self
    }

    /// Returns the time at which the token should expire.
    #[must_use]
    pub fn get_expiration_time(&self) -> Option<OffsetDateTime> {
        self.data().expiration_time
    }

    /// Sets the time at which the token should expire.
    pub fn expiration_time(&mut self, expiration_time: OffsetDateTime) -> &mut Self {
        let data = self.data_mut();
        data.expiration_time = Some(expiration_time);
        data.auto_renew_period = None;

        self
    }

    /// Returns the account which will be automatically charged to renew the token's expiration.
    #[must_use]
    pub fn get_auto_renew_account_id(&self) -> Option<AccountId> {
        self.data().auto_renew_account_id
    }

    /// Sets the account which will be automatically charged to renew the token's expiration.
    pub fn auto_renew_account_id(&mut self, auto_renew_account_id: AccountId) -> &mut Self {
        self.data_mut().auto_renew_account_id = Some(auto_renew_account_id);
        self
    }

    /// Returns the interval at which the auto renew account will be charged to extend the token's expiry.
    #[must_use]
    pub fn get_auto_renew_period(&self) -> Option<Duration> {
        self.data().auto_renew_period
    }

    /// Sets the interval at which the auto renew account will be charged to extend
    /// the token's expiry.
    pub fn auto_renew_period(&mut self, auto_renew_period: Duration) -> &mut Self {
        self.data_mut().auto_renew_period = Some(auto_renew_period);
        self
    }

    /// Returns the memo associated with the token.
    #[must_use]
    pub fn get_token_memo(&self) -> &str {
        &self.data().token_memo
    }

    // note(sr): I got rid of the comment stating UTF-8, since this is a Rust string, which implies UTF-8.
    /// Sets the memo associated with the token.
    ///
    /// Maximum 100 bytes.
    pub fn token_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.data_mut().token_memo = memo.into();
        self
    }

    /// Returns the token type.
    #[must_use]
    pub fn get_token_type(&self) -> TokenType {
        self.data().token_type
    }

    /// Sets the token type. Defaults to `FungibleCommon`.
    pub fn token_type(&mut self, token_type: TokenType) -> &mut Self {
        self.data_mut().token_type = token_type;
        self
    }

    /// Returns the token supply type.
    #[must_use]
    pub fn get_token_supply_type(&self) -> TokenSupplyType {
        self.data().token_supply_type
    }

    /// Sets the token supply type. Defaults to `Infinite`.
    pub fn token_supply_type(&mut self, token_supply_type: TokenSupplyType) -> &mut Self {
        self.data_mut().token_supply_type = token_supply_type;
        self
    }

    /// Returns the maximum number of tokens that can be in circulation.
    #[must_use]
    pub fn get_max_supply(&self) -> u64 {
        self.data().max_supply
    }

    /// Sets the maximum number of tokens that can be in circulation.
    pub fn max_supply(&mut self, max_supply: u64) -> &mut Self {
        self.data_mut().max_supply = max_supply;
        self
    }

    /// Returns the key which can change the token's custom fee schedule.
    #[must_use]
    pub fn get_fee_schedule_key(&self) -> Option<&Key> {
        self.data().fee_schedule_key.as_ref()
    }

    /// Sets the key which can change the token's custom fee schedule.
    pub fn fee_schedule_key(&mut self, fee_schedule_key: impl Into<Key>) -> &mut Self {
        self.data_mut().fee_schedule_key = Some(fee_schedule_key.into());
        self
    }

    /// Returns the custom fees to be assessed during a transfer.
    #[must_use]
    pub fn get_custom_fees(&self) -> &[AnyCustomFee] {
        &self.data().custom_fees
    }

    /// Sets the custom fees to be assessed during a transfer.
    pub fn custom_fees(
        &mut self,
        custom_fees: impl IntoIterator<Item = AnyCustomFee>,
    ) -> &mut Self {
        self.data_mut().custom_fees = custom_fees.into_iter().collect();
        self
    }

    /// Returns the key which can pause and unpause the token.
    #[must_use]
    pub fn get_pause_key(&self) -> Option<&Key> {
        self.data().pause_key.as_ref()
    }

    /// Sets the key which can pause and unpause the token.
    pub fn pause_key(&mut self, pause_key: impl Into<Key>) -> &mut Self {
        self.data_mut().pause_key = Some(pause_key.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenCreateTransactionData {
    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        // TODO: validate custom fee collector account IDs once that's merged
        self.treasury_account_id.validate_checksums_for_ledger_id(ledger_id)?;
        self.auto_renew_account_id.validate_checksums_for_ledger_id(ledger_id)
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        TokenServiceClient::new(channel).create_token(request).await
    }
}

impl ToTransactionDataProtobuf for TokenCreateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        services::transaction_body::Data::TokenCreation(services::TokenCreateTransactionBody {
            name: self.name.clone(),
            symbol: self.symbol.clone(),
            decimals: self.decimals,
            initial_supply: self.initial_supply,
            treasury: self.treasury_account_id.to_protobuf(),
            admin_key: self.admin_key.to_protobuf(),
            kyc_key: self.kyc_key.to_protobuf(),
            freeze_key: self.freeze_key.to_protobuf(),
            wipe_key: self.wipe_key.to_protobuf(),
            supply_key: self.supply_key.to_protobuf(),
            freeze_default: self.freeze_default,
            expiry: self.expiration_time.map(Into::into),
            auto_renew_account: self.auto_renew_account_id.to_protobuf(),
            auto_renew_period: self.auto_renew_period.map(Into::into),
            memo: self.token_memo.clone(),
            token_type: self.token_type.to_protobuf().into(),
            supply_type: self.token_supply_type.to_protobuf().into(),
            max_supply: self.max_supply as i64,
            fee_schedule_key: self.fee_schedule_key.to_protobuf(),
            custom_fees: self.custom_fees.to_protobuf(),
            pause_key: self.pause_key.to_protobuf(),
        })
    }
}

impl From<TokenCreateTransactionData> for AnyTransactionData {
    fn from(transaction: TokenCreateTransactionData) -> Self {
        Self::TokenCreate(transaction)
    }
}

impl FromProtobuf<services::TokenCreateTransactionBody> for TokenCreateTransactionData {
    fn from_protobuf(pb: services::TokenCreateTransactionBody) -> crate::Result<Self> {
        let services::TokenCreateTransactionBody {
            name,
            symbol,
            decimals,
            initial_supply,
            treasury,
            admin_key,
            kyc_key,
            freeze_key,
            wipe_key,
            supply_key,
            freeze_default,
            expiry,
            auto_renew_account,
            auto_renew_period,
            memo,
            token_type,
            supply_type,
            max_supply,
            fee_schedule_key,
            custom_fees,
            pause_key,
        } = pb;

        let token_type = services::TokenType::from_i32(token_type).unwrap_or_default();
        let token_supply_type =
            services::TokenSupplyType::from_i32(supply_type).unwrap_or_default();

        Ok(Self {
            name,
            symbol,
            decimals,
            initial_supply,
            treasury_account_id: Option::from_protobuf(treasury)?,
            admin_key: Option::from_protobuf(admin_key)?,
            kyc_key: Option::from_protobuf(kyc_key)?,
            freeze_key: Option::from_protobuf(freeze_key)?,
            wipe_key: Option::from_protobuf(wipe_key)?,
            supply_key: Option::from_protobuf(supply_key)?,
            freeze_default,
            expiration_time: expiry.map(Into::into),
            auto_renew_account_id: Option::from_protobuf(auto_renew_account)?,
            auto_renew_period: auto_renew_period.map(Into::into),
            token_memo: memo,
            token_type: TokenType::from_protobuf(token_type)?,
            token_supply_type: TokenSupplyType::from_protobuf(token_supply_type)?,
            max_supply: max_supply as u64,
            fee_schedule_key: Option::from_protobuf(fee_schedule_key)?,
            custom_fees: Vec::from_protobuf(custom_fees)?,
            pause_key: Option::from_protobuf(pause_key)?,
        })
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

        use crate::token::custom_fees::{
            CustomFee,
            FixedFeeData,
        };
        use crate::token::token_supply_type::TokenSupplyType;
        use crate::token::token_type::TokenType;
        use crate::transaction::{
            AnyTransaction,
            AnyTransactionData,
        };
        use crate::{
            AccountId,
            Key,
            PublicKey,
            TokenCreateTransaction,
            TokenId,
        };

        // language=JSON
        const TOKEN_CREATE_EMPTY: &str = r#"{
  "$type": "tokenCreate"
}"#;

        // language=JSON
        const TOKEN_CREATE_TRANSACTION_JSON: &str = r#"{
  "$type": "tokenCreate",
  "name": "Pound",
  "symbol": "LB",
  "decimals": 9,
  "initialSupply": 1000000000,
  "treasuryAccountId": "0.0.1001",
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
  "freezeDefault": false,
  "expirationTime": 1656352251277559886,
  "autoRenewAccountId": "0.0.1002",
  "autoRenewPeriod": 7776000,
  "tokenMemo": "A memo",
  "tokenType": "fungibleCommon",
  "tokenSupplyType": "finite",
  "maxSupply": 1000000000,
  "feeScheduleKey": {
    "single": "302a300506032b65700321000cd029bfd4a818de944c21799f4b5f6b5616702d0495520c818d92488e5395fc"
  },
  "customFees": [
    {
      "$type": "fixed",
      "amount": 1,
      "denominatingTokenId": "0.0.7",
      "feeCollectorAccountId": "0.0.8",
      "allCollectorsAreExempt": false
    }
  ],
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
            let mut transaction = TokenCreateTransaction::new();

            transaction
                .name("Pound")
                .symbol("LB")
                .decimals(9)
                .initial_supply(1_000_000_000)
                .treasury_account_id(AccountId::from_str("0.0.1001")?)
                .admin_key(PublicKey::from_str(ADMIN_KEY)?)
                .kyc_key(PublicKey::from_str(KYC_KEY)?)
                .freeze_key(PublicKey::from_str(FREEZE_KEY)?)
                .wipe_key(PublicKey::from_str(WIPE_KEY)?)
                .supply_key(PublicKey::from_str(SUPPLY_KEY)?)
                .freeze_default(false)
                .expiration_time(OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?)
                .auto_renew_account_id(AccountId::from_str("0.0.1002")?)
                .auto_renew_period(Duration::days(90))
                .token_memo("A memo")
                .token_type(TokenType::FungibleCommon)
                .token_supply_type(TokenSupplyType::Finite)
                .max_supply(1_000_000_000)
                .fee_schedule_key(PublicKey::from_str(FEE_SCHEDULE_KEY)?)
                .custom_fees([CustomFee {
                    fee: FixedFeeData { amount: 1, denominating_token_id: TokenId::from(7) }.into(),
                    fee_collector_account_id: Some(AccountId::from(8)),
                    all_collectors_are_exempt: false,
                }])
                .pause_key(PublicKey::from_str(PAUSE_KEY)?);

            let transaction_json = serde_json::to_string_pretty(&transaction)?;

            assert_eq!(transaction_json, TOKEN_CREATE_TRANSACTION_JSON);

            Ok(())
        }

        #[test]
        fn it_should_deserialize() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOKEN_CREATE_TRANSACTION_JSON)?;

            let data = assert_matches!(transaction.into_body().data, AnyTransactionData::TokenCreate(transaction) => transaction);

            assert_eq!(data.name, "Pound");
            assert_eq!(data.symbol, "LB");
            assert_eq!(data.decimals, 9);
            assert_eq!(data.initial_supply, 1_000_000_000);
            assert_eq!(data.freeze_default, false);
            assert_eq!(
                data.expiration_time.unwrap(),
                OffsetDateTime::from_unix_timestamp_nanos(1656352251277559886)?
            );
            assert_eq!(data.auto_renew_period.unwrap(), Duration::days(90));
            assert_eq!(data.token_memo, "A memo");
            assert_eq!(data.token_type, TokenType::FungibleCommon);
            assert_eq!(data.token_supply_type, TokenSupplyType::Finite);
            assert_eq!(data.max_supply, 1_000_000_000);
            assert_eq!(data.treasury_account_id, Some(AccountId::from(1001)));
            assert_eq!(data.auto_renew_account_id, Some(AccountId::from(1002)));

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

            assert_eq!(
                data.custom_fees,
                [CustomFee {
                    fee: FixedFeeData { amount: 1, denominating_token_id: TokenId::from(7) }.into(),
                    fee_collector_account_id: Some(AccountId::from(8)),
                    all_collectors_are_exempt: false,
                }]
            );

            Ok(())
        }

        #[test]
        #[ignore = "auto renew period is `None`"]
        fn it_should_deserialize_empty() -> anyhow::Result<()> {
            let transaction: AnyTransaction = serde_json::from_str(TOKEN_CREATE_EMPTY)?;

            let data = assert_matches!(transaction.data(), AnyTransactionData::TokenCreate(transaction) => transaction);

            assert_eq!(data.auto_renew_period.unwrap(), Duration::days(90));
            assert_eq!(data.token_type, TokenType::FungibleCommon);
            assert_eq!(data.token_supply_type, TokenSupplyType::Infinite);

            Ok(())
        }
    }
}
