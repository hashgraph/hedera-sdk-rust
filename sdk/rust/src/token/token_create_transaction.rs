use async_trait::async_trait;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use itertools::Itertools;
use serde_with::{serde_as, skip_serializing_none, DurationSeconds, TimestampNanoSeconds};
use time::{Duration, OffsetDateTime};
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::token::custom_fees::CustomFee;
use crate::token::token_supply_type::TokenSupplyType;
use crate::token::token_type::TokenType;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};
use crate::{AccountAddress, AccountId, Key, Transaction, TransactionId};

/// Create a new token.
///
/// After the token is created, the [`TokenId`] for it is in the receipt.
///
/// The specified treasury account receives the initial supply of tokens, as well as the tokens
/// from a [`TokenMintTransaction`] once executed. The balance of the treasury account is
/// decreased when a [`TokenBurnTransaction`] is executed.
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

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCreateTransactionData {
    /// The publicly visible name of the token. The token name is specified as a Unicode string.
    /// Its UTF-8 encoding cannot exceed 100 bytes, and cannot contain the 0 byte (NUL).
    name: String,

    /// The publicly visible token symbol. The token symbol is specified as a Unicode string.
    /// Its UTF-8 encoding cannot exceed 100 bytes, and cannot contain the 0 byte (NUL).
    symbol: String,

    /// For tokens of type FUNGIBLE_COMMON - the number of decimal places a
    /// token is divisible by. For tokens of type NON_FUNGIBLE_UNIQUE - value
    /// must be 0
    decimals: u32,

    /// Specifies the initial supply of tokens to be put in circulation. The
    /// initial supply is sent to the Treasury Account. The supply is in the
    /// lowest denomination possible. In the case for NON_FUNGIBLE_UNIQUE Type
    /// the value must be 0
    initial_supply: u64,

    /// The account which will act as a treasury for the token. This account
    /// will receive the specified initial supply or the newly minted NFTs in
    /// the case for NON_FUNGIBLE_UNIQUE Type
    treasury_account_id: Option<AccountAddress>,

    /// The key which can perform update/delete operations on the token. If empty, the token can be
    /// perceived as immutable (not being able to be updated/deleted)
    admin_key: Option<Key>,

    /// The key which can grant or revoke KYC of an account for the token's transactions. If empty,
    /// KYC is not required, and KYC grant or revoke operations are not possible.
    kyc_key: Option<Key>,

    /// The key which can sign to freeze or unfreeze an account for token transactions. If empty,
    /// freezing is not possible
    freeze_key: Option<Key>,

    /// The key which can wipe the token balance of an account. If empty, wipe is not possible
    wipe_key: Option<Key>,

    /// The key which can change the supply of a token. The key is used to sign Token Mint/Burn
    /// operations
    supply_key: Option<Key>,

    /// The default Freeze status (frozen or unfrozen) of Hedera accounts relative to this token. If
    /// true, an account must be unfrozen before it can receive the token
    freeze_default: bool,

    /// The epoch second at which the token should expire; if an auto-renew account and period are
    /// specified, this is coerced to the current epoch second plus the autoRenewPeriod
    #[serde_as(as = "Option<TimestampNanoSeconds>")]
    expires_at: Option<OffsetDateTime>,

    /// An account which will be automatically charged to renew the token's expiration, at
    /// autoRenewPeriod interval
    auto_renew_account_id: Option<AccountAddress>,

    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    auto_renew_period: Option<Duration>,

    /// The memo associated with the token (UTF-8 encoding max 100 bytes)
    token_memo: String,

    /// IWA compatibility. Specifies the token type. Defaults to FUNGIBLE_COMMON
    token_type: TokenType,

    /// IWA compatibility. Specifies the token supply type. Defaults to INFINITE
    token_supply_type: TokenSupplyType,

    /// IWA Compatibility. Depends on TokenSupplyType. For tokens of type FUNGIBLE_COMMON - the
    /// maximum number of tokens that can be in circulation. For tokens of type NON_FUNGIBLE_UNIQUE -
    /// the maximum number of NFTs (serial numbers) that can be minted. This field can never be
    /// changed!
    max_supply: u64,

    /// The key which can change the token's custom fee schedule; must sign a TokenFeeScheduleUpdate
    /// transaction
    fee_schedule_key: Option<Key>,

    /// The custom fees to be assessed during a CryptoTransfer that transfers units of this token
    custom_fees: Vec<CustomFee>,

    /// The Key which can pause and unpause the Token.
    /// If Empty the token pause status defaults to PauseNotApplicable, otherwise Unpaused.
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
            expires_at: None,
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
    // TODO: @Daniel why is this tokenName in v2?
    /// Sets the publicly visible name of the token.
    /// Maximum 100 characters.
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.body.data.name = name.into();
        self
    }

    // TODO: @Daniel why is this tokenSymbol in v2?
    /// Sets the publicly visible token symbol.
    /// Maximum 100 characters.
    pub fn symbol(&mut self, symbol: impl Into<String>) -> &mut Self {
        self.body.data.symbol = symbol.into();
        self
    }

    /// Sets the number of decimal places a token is divisible by.
    pub fn decimals(&mut self, decimals: u32) -> &mut Self {
        self.body.data.decimals = decimals;
        self
    }

    /// Sets the initial supply of tokens to be put in circulation.
    pub fn initial_supply(&mut self, initial_supply: u64) -> &mut Self {
        self.body.data.initial_supply = initial_supply;
        self
    }

    /// Sets the account which will act as a treasury for the token.
    pub fn treasury_account_id(
        &mut self,
        treasury_account_id: impl Into<AccountAddress>,
    ) -> &mut Self {
        self.body.data.treasury_account_id = Some(treasury_account_id.into());
        self
    }

    /// Sets the key which can perform update/delete operations on the token.
    pub fn admin_key(&mut self, admin_key: impl Into<Key>) -> &mut Self {
        self.body.data.admin_key = Some(admin_key.into());
        self
    }

    /// Sets the key which can grant or revoke KYC of an account for the token's transactions.
    pub fn kyc_key(&mut self, kyc_key: impl Into<Key>) -> &mut Self {
        self.body.data.kyc_key = Some(kyc_key.into());
        self
    }

    /// Sets the key which can sign to freeze or unfreeze an account for token transactions.
    pub fn freeze_key(&mut self, freeze_key: impl Into<Key>) -> &mut Self {
        self.body.data.freeze_key = Some(freeze_key.into());
        self
    }

    /// Sets the key which can wipe the token balance of an account.
    pub fn wipe_key(&mut self, wipe_key: impl Into<Key>) -> &mut Self {
        self.body.data.wipe_key = Some(wipe_key.into());
        self
    }

    /// Sets the key which can change the supply of a token.
    pub fn supply_key(&mut self, supply_key: impl Into<Key>) -> &mut Self {
        self.body.data.supply_key = Some(supply_key.into());
        self
    }

    /// Sets the default freeze status (frozen or unfrozen) of hedera accounts
    /// relative to this token. If true, an account must be unfrozen before it can receive the token
    pub fn freeze_default(&mut self, freeze_default: bool) -> &mut Self {
        self.body.data.freeze_default = freeze_default;
        self
    }

    /// Sets the time at which the token should expire.
    pub fn expires_at(&mut self, expires_at: OffsetDateTime) -> &mut Self {
        self.body.data.expires_at = Some(expires_at.into());
        self.body.data.auto_renew_period = None;

        self
    }

    /// Sets the account which will be automatically charged to renew the token's expiration.
    pub fn auto_renew_account_id(
        &mut self,
        auto_renew_account_id: impl Into<AccountAddress>,
    ) -> &mut Self {
        self.body.data.auto_renew_account_id = Some(auto_renew_account_id.into());
        self
    }

    /// Sets the interval at which the auto renew account will be charged to extend
    /// the token's expiry.
    pub fn auto_renew_period(&mut self, auto_renew_period: Duration) -> &mut Self {
        self.body.data.auto_renew_period = Some(auto_renew_period);
        self
    }

    /// Sets the memo associated with the token (UTF-8 encoding max 100 bytes)
    pub fn token_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.token_memo = memo.into();
        self
    }

    /// Sets the token type. Defaults to `FungibleCommon`.
    pub fn token_type(&mut self, token_type: TokenType) -> &mut Self {
        self.body.data.token_type = token_type;
        self
    }

    /// Sets the token supply type. Defaults to `Infinite`.
    pub fn token_supply_type(&mut self, token_supply_type: TokenSupplyType) -> &mut Self {
        self.body.data.token_supply_type = token_supply_type;
        self
    }

    /// Sets the maximum number of tokens that can be in circulation.
    pub fn max_supply(&mut self, max_supply: u64) -> &mut Self {
        self.body.data.max_supply = max_supply;
        self
    }

    /// Sets the key which can change the token's custom fee schedule.
    pub fn fee_schedule_key(&mut self, fee_schedule_key: impl Into<Key>) -> &mut Self {
        self.body.data.fee_schedule_key = Some(fee_schedule_key.into());
        self
    }

    /// Sets the custom fees to be assessed during a transfer.
    pub fn custom_fees(&mut self, custom_fees: impl IntoIterator<Item = CustomFee>) -> &mut Self {
        self.body.data.custom_fees = custom_fees.into_iter().collect();
        self
    }

    /// Sets the Key which can pause and unpause the Token.
    pub fn pause_key(&mut self, pause_key: impl Into<Key>) -> &mut Self {
        self.body.data.pause_key = Some(pause_key.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for TokenCreateTransactionData {
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
            treasury: self.treasury_account_id.as_ref().map(AccountAddress::to_protobuf),
            admin_key: self.admin_key.as_ref().map(Key::to_protobuf),
            kyc_key: self.kyc_key.as_ref().map(Key::to_protobuf),
            freeze_key: self.freeze_key.as_ref().map(Key::to_protobuf),
            wipe_key: self.wipe_key.as_ref().map(Key::to_protobuf),
            supply_key: self.supply_key.as_ref().map(Key::to_protobuf),
            freeze_default: self.freeze_default,
            expiry: self.expires_at.map(Into::into),
            auto_renew_account: self
                .auto_renew_account_id
                .as_ref()
                .map(AccountAddress::to_protobuf),
            auto_renew_period: self.auto_renew_period.map(Into::into),
            memo: self.token_memo.clone(),
            token_type: self.token_type.to_protobuf().into(),
            supply_type: self.token_supply_type.to_protobuf().into(),
            max_supply: self.max_supply as i64,
            fee_schedule_key: self.fee_schedule_key.as_ref().map(Key::to_protobuf),
            custom_fees: self.custom_fees.iter().map(CustomFee::to_protobuf).collect_vec(),
            pause_key: self.pause_key.as_ref().map(Key::to_protobuf),
        })
    }
}

impl From<TokenCreateTransactionData> for AnyTransactionData {
    fn from(transaction: TokenCreateTransactionData) -> Self {
        Self::TokenCreate(transaction)
    }
}
