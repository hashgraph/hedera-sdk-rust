use async_trait::async_trait;
use itertools::Itertools;
use tonic::transport::Channel;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::{serde_as, skip_serializing_none};

use crate::protobuf::ToProtobuf;
use crate::{AccountId, Key, Transaction, TransactionId};
use crate::duration::Duration;
use crate::timestamp::Timestamp;
use crate::token::custom_fees::{CustomFee};
use crate::token::token_supply_type::TokenSupplyType;
use crate::token::token_type::TokenType;
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};

/// Create a new token. After the token is created, the Token ID for it is in the receipt.
/// The specified Treasury Account is receiving the initial supply of tokens as-well as the tokens
/// from the Token Mint operation once executed. The balance of the treasury account is decreased
/// when the Token Burn operation is executed.
///
/// The <tt>initialSupply</tt> is the initial supply of the smallest parts of a token (like a
/// tinybar, not an hbar). These are the smallest units of the token which may be transferred.
///
/// The supply can change over time. If the total supply at some moment is <i>S</i> parts of tokens,
/// and the token is using <i>D</i> decimals, then <i>S</i> must be less than or equal to
/// 2<sup>63</sup>-1, which is 9,223,372,036,854,775,807. The number of whole tokens (not parts) will
/// be <i>S / 10<sup>D</sup></i>.
///
/// If decimals is 8 or 11, then the number of whole tokens can be at most a few billions or
/// millions, respectively. For example, it could match Bitcoin (21 million whole tokens with 8
/// decimals) or hbars (50 billion whole tokens with 8 decimals). It could even match Bitcoin with
/// milli-satoshis (21 million whole tokens with 11 decimals).
///
/// Note that a created token is <i>immutable</i> if the <tt>adminKey</tt> is omitted. No property of
/// an immutable token can ever change, with the sole exception of its expiry. Anyone can pay to
/// extend the expiry time of an immutable token.
///
/// A token can be either <i>FUNGIBLE_COMMON</i> or <i>NON_FUNGIBLE_UNIQUE</i>, based on its
/// <i>TokenType</i>. If it has been omitted, <i>FUNGIBLE_COMMON</i> type is used.
///
/// A token can have either <i>INFINITE</i> or <i>FINITE</i> supply type, based on its
/// <i>TokenType</i>. If it has been omitted, <i>INFINITE</i> type is used.
///
/// - If a <i>FUNGIBLE</i> TokenType is used, <i>initialSupply</i> should explicitly be set to a
/// non-negative. If not, the transaction will resolve to INVALID_TOKEN_INITIAL_SUPPLY.
///
/// - If a <i>NON_FUNGIBLE_UNIQUE</i> TokenType is used, <i>initialSupply</i> should explicitly be set
/// to 0. If not, the transaction will resolve to INVALID_TOKEN_INITIAL_SUPPLY.
///
/// - If an <i>INFINITE</i> TokenSupplyType is used, <i>maxSupply</i> should explicitly be set to 0. If
/// it is not 0, the transaction will resolve to INVALID_TOKEN_MAX_SUPPLY.
///
/// - If a <i>FINITE</i> TokenSupplyType is used, <i>maxSupply</i> should be explicitly set to a
/// non-negative value. If it is not, the transaction will resolve to INVALID_TOKEN_MAX_SUPPLY.
pub type TokenCreateTransaction = Transaction<TokenCreateTransactionData>;

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenCreateTransactionData {
    /// The publicly visible name of the token. The token name is specified as a Unicode string.
    /// Its UTF-8 encoding cannot exceed 100 bytes, and cannot contain the 0 byte (NUL).
    name: Option<String>,

    /// The publicly visible token symbol. The token symbol is specified as a Unicode string.
    /// Its UTF-8 encoding cannot exceed 100 bytes, and cannot contain the 0 byte (NUL).
    symbol: Option<String>,

    /// For tokens of type FUNGIBLE_COMMON - the number of decimal places a
    /// token is divisible by. For tokens of type NON_FUNGIBLE_UNIQUE - value
    /// must be 0
    decimals: Option<u32>,

    /// Specifies the initial supply of tokens to be put in circulation. The
    /// initial supply is sent to the Treasury Account. The supply is in the
    /// lowest denomination possible. In the case for NON_FUNGIBLE_UNIQUE Type
    /// the value must be 0
    initial_supply: Option<u64>,

    /// The account which will act as a treasury for the token. This account
    /// will receive the specified initial supply or the newly minted NFTs in
    /// the case for NON_FUNGIBLE_UNIQUE Type
    treasury_account_id: Option<AccountId>,

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
    freeze_default: Option<bool>,

    /// The epoch second at which the token should expire; if an auto-renew account and period are
    /// specified, this is coerced to the current epoch second plus the autoRenewPeriod
    expiration_time: Option<Timestamp>,

    /// An account which will be automatically charged to renew the token's expiration, at
    /// autoRenewPeriod interval
    auto_renew_account_id: Option<AccountId>,

    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    auto_renew_period: Option<Duration>,

    /// The memo associated with the token (UTF-8 encoding max 100 bytes)
    memo: Option<String>,

    /// IWA compatibility. Specifies the token type. Defaults to FUNGIBLE_COMMON
    token_type: Option<TokenType>,

    /// IWA compatibility. Specifies the token supply type. Defaults to INFINITE
    token_supply_type: Option<TokenSupplyType>,

    /// IWA Compatibility. Depends on TokenSupplyType. For tokens of type FUNGIBLE_COMMON - the
    /// maximum number of tokens that can be in circulation. For tokens of type NON_FUNGIBLE_UNIQUE -
    /// the maximum number of NFTs (serial numbers) that can be minted. This field can never be
    /// changed!
    max_supply: Option<i64>,

    /// The key which can change the token's custom fee schedule; must sign a TokenFeeScheduleUpdate
    /// transaction
    fee_schedule_key: Option<Key>,

    /// The custom fees to be assessed during a CryptoTransfer that transfers units of this token
    custom_fees: Vec<CustomFee>,

    /// The Key which can pause and unpause the Token.
    /// If Empty the token pause status defaults to PauseNotApplicable, otherwise Unpaused.
    pause_key: Option<Key>,
}

impl TokenCreateTransaction {
    /// Sets the publicly visible name of the token. The token name is specified as a Unicode string.
    /// Its UTF-8 encoding cannot exceed 100 bytes, and cannot contain the 0 byte (NUL).
    pub fn name(&mut self, name: impl Into<String>) -> &mut Self {
        self.body.data.name = Some(name.into());
        self
    }

    /// Sets the publicly visible token symbol. The token symbol is specified as a Unicode string.
    /// Its UTF-8 encoding cannot exceed 100 bytes, and cannot contain the 0 byte (NUL).
    pub fn symbol(&mut self, symbol: impl Into<String>) -> &mut Self {
        self.body.data.symbol = Some(symbol.into());
        self
    }

    /// For tokens of type FUNGIBLE_COMMON - sets the number of decimal places a
    /// token is divisible by. For tokens of type NON_FUNGIBLE_UNIQUE - value
    /// must be 0
    pub fn decimals(&mut self, decimals: impl Into<u32>) -> &mut Self {
        self.body.data.decimals = Some(decimals.into());
        self
    }

    /// Specifies the initial supply of tokens to be put in circulation. The
    /// initial supply is sent to the Treasury Account. The supply is in the
    /// lowest denomination possible. In the case for NON_FUNGIBLE_UNIQUE Type
    /// the value must be 0
    pub fn initial_supply(&mut self, initial_supply: impl Into<u64>) -> &mut Self {
        self.body.data.initial_supply = Some(initial_supply.into());
        self
    }

    /// Sets the account which will act as a treasury for the token. This account
    /// will receive the specified initial supply or the newly minted NFTs in
    /// the case for NON_FUNGIBLE_UNIQUE Type
    pub fn treasury_account_id(&mut self, treasury_account_id: impl Into<AccountId>) -> &mut Self {
        self.body.data.treasury_account_id = Some(treasury_account_id.into());
        self
    }

    /// Sets the key which can perform update/delete operations on the token. If empty, the token can be
    /// perceived as immutable (not being able to be updated/deleted)
    pub fn admin_key(&mut self, admin_key: impl Into<Key>) -> &mut Self {
        self.body.data.admin_key = Some(admin_key.into());
        self
    }

    /// Sets the key which can grant or revoke KYC of an account for the token's transactions. If empty,
    /// KYC is not required, and KYC grant or revoke operations are not possible.
    pub fn kyc_key(&mut self, kyc_key: impl Into<Key>) -> &mut Self {
        self.body.data.kyc_key = Some(kyc_key.into());
        self
    }

    /// Sets the key which can sign to freeze or unfreeze an account for token transactions. If empty,
    /// freezing is not possible
    pub fn freeze_key(&mut self, freeze_key: impl Into<Key>) -> &mut Self {
        self.body.data.freeze_key = Some(freeze_key.into());
        self
    }

    /// Sets the key which can wipe the token balance of an account. If empty, wipe is not possible
    pub fn wipe_key(&mut self, wipe_key: impl Into<Key>) -> &mut Self {
        self.body.data.wipe_key = Some(wipe_key.into());
        self
    }

    /// Sets the key which can change the supply of a token. The key is used to sign Token Mint/Burn
    /// operations
    pub fn supply_key(&mut self, supply_key: impl Into<Key>) -> &mut Self {
        self.body.data.supply_key = Some(supply_key.into());
        self
    }

    /// Sets the default Freeze status (frozen or unfrozen) of Hedera accounts relative to this token. If
    /// true, an account must be unfrozen before it can receive the token
    pub fn freeze_default(&mut self, freeze_default: impl Into<bool>) -> &mut Self {
        self.body.data.freeze_default = Some(freeze_default.into());
        self
    }

    /// Sets the epoch second at which the token should expire; if an auto-renew account and period are
    /// specified, this is coerced to the current epoch second plus the autoRenewPeriod
    pub fn expiration_time(&mut self, expiration_time: impl Into<Timestamp>) -> &mut Self {
        self.body.data.expiration_time = Some(expiration_time.into());
        self
    }

    /// Sets the account which will be automatically charged to renew the token's expiration, at
    /// autoRenewPeriod interval
    pub fn auto_renew_account_id(&mut self, auto_renew_account_id: impl Into<AccountId>) -> &mut Self {
        self.body.data.auto_renew_account_id = Some(auto_renew_account_id.into());
        self
    }

    /// Sets the interval at which the auto-renew account will be charged to extend the token's expiry
    pub fn auto_renew_period(&mut self, auto_renew_period: impl Into<Duration>) -> &mut Self {
        self.body.data.auto_renew_period = Some(auto_renew_period.into());
        self
    }

    /// Sets the memo associated with the token (UTF-8 encoding max 100 bytes)
    pub fn memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.memo = Some(memo.into());
        self
    }

    /// IWA compatibility. Specifies the token type. Defaults to FUNGIBLE_COMMON
    pub fn token_type(&mut self, token_type: impl Into<TokenType>) -> &mut Self {
        self.body.data.token_type = Some(token_type.into());
        self
    }

    /// IWA compatibility. Specifies the token supply type. Defaults to INFINITE
    pub fn token_supply_type(&mut self, token_supply_type: impl Into<TokenSupplyType>) -> &mut Self {
        self.body.data.token_supply_type = Some(token_supply_type.into());
        self
    }

    /// IWA Compatibility. Depends on TokenSupplyType. For tokens of type FUNGIBLE_COMMON - sets the
    /// maximum number of tokens that can be in circulation. For tokens of type NON_FUNGIBLE_UNIQUE -
    /// sets the maximum number of NFTs (serial numbers) that can be minted. This field can never be
    /// changed!
    pub fn max_supply(&mut self, max_supply: impl Into<i64>) -> &mut Self {
        self.body.data.max_supply = Some(max_supply.into());
        self
    }

    /// Sets the key which can change the token's custom fee schedule; must sign a TokenFeeScheduleUpdate
    /// transaction
    pub fn fee_schedule_key(&mut self, fee_schedule_key: impl Into<Key>) -> &mut Self {
        self.body.data.fee_schedule_key = Some(fee_schedule_key.into());
        self
    }

    /// Sets the custom fees to be assessed during a CryptoTransfer that transfers units of this token
    pub fn custom_fees(&mut self, custom_fees: impl IntoIterator<Item = CustomFee>) -> &mut Self {
        self.body.data.custom_fees = custom_fees.into_iter().collect();
        self
    }

    /// Sets the Key which can pause and unpause the Token.
    /// If Empty the token pause status defaults to PauseNotApplicable, otherwise Unpaused.
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
            name: self.name.clone().unwrap_or_default(),
            symbol: self.symbol.clone().unwrap_or_default(),
            decimals: self.decimals.clone().unwrap_or_default(),
            initial_supply: self.decimals.clone().unwrap_or_default() as u64,
            treasury: self.treasury_account_id.as_ref().map(AccountId::to_protobuf),
            admin_key: self.admin_key.as_ref().map(Key::to_protobuf),
            kyc_key: self.kyc_key.as_ref().map(Key::to_protobuf),
            freeze_key: self.freeze_key.as_ref().map(Key::to_protobuf),
            wipe_key: self.wipe_key.as_ref().map(Key::to_protobuf),
            supply_key: self.supply_key.as_ref().map(Key::to_protobuf),
            freeze_default: self.freeze_default.clone().unwrap_or_default(),
            expiry: self.expiration_time.as_ref().map(Timestamp::to_protobuf),
            auto_renew_account: self.auto_renew_account_id.as_ref().map(AccountId::to_protobuf),
            auto_renew_period: self.auto_renew_period.as_ref().map(Duration::to_protobuf),
            memo: self.memo.clone().unwrap_or_default(),
            token_type: self.token_type.as_ref().map(TokenType::to_protobuf).unwrap_or_default().into(),
            supply_type: self.token_supply_type.as_ref().map(TokenSupplyType::to_protobuf).unwrap_or_default().into(),
            max_supply: self.max_supply.clone().unwrap_or_default(),
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
