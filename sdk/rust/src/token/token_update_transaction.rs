use async_trait::async_trait;
use tonic::transport::Channel;
use hedera_proto::services;
use hedera_proto::services::token_service_client::TokenServiceClient;
use serde_with::{serde_as, skip_serializing_none, DurationSeconds, TimestampNanoSeconds};
use time::{Duration, OffsetDateTime};

use crate::protobuf::ToProtobuf;
use crate::{AccountAddress, AccountId, Key, TokenId, Transaction, TransactionId};
use crate::transaction::{AnyTransactionData, ToTransactionDataProtobuf, TransactionExecute};

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

#[serde_as]
#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenUpdateTransactionData {
    /// The token to be updated.
    token_id: Option<TokenId>,

    /// The publicly visible name of the token.
    name: String,

    /// The publicly visible token symbol.
    symbol: String,

    /// The account which will act as a treasury for the token.
    treasury_account_id: Option<AccountAddress>,

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
    auto_renew_account_id: Option<AccountAddress>,

    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    #[serde_as(as = "Option<DurationSeconds<i64>>")]
    auto_renew_period: Option<Duration>,

    /// Sets the time at which the token should expire.
    #[serde_as(as = "Option<TimestampNanoSeconds>")]
    expires_at: Option<OffsetDateTime>,

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
    pub fn treasury_account_id(
        &mut self,
        treasury_account_id: impl Into<AccountAddress>,
    ) -> &mut Self {
        self.body.data.treasury_account_id = Some(treasury_account_id.into());
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
    pub fn auto_renew_account_id(
        &mut self,
        auto_renew_account_id: impl Into<AccountAddress>,
    ) -> &mut Self {
        self.body.data.auto_renew_account_id = Some(auto_renew_account_id.into());
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
    pub fn expires_at(&mut self, expires_at: OffsetDateTime) -> &mut Self {
        self.body.data.expires_at = Some(expires_at.into());
        self.body.data.auto_renew_period = None;

        self
    }

    /// Sets the new memo associated with the token (UTF-8 encoding max 100 bytes)
    pub fn token_memo(&mut self, memo: impl Into<String>) -> &mut Self {
        self.body.data.token_memo = memo.into();
        self
    }

    /// Sets the new key which can change the token's custom fee schedule.
    ///
    /// If the token does not currently have a Fee Schedule key, transaction will resolve to
    /// `TokenHasNoFeeScheduleKey`.
    pub fn fee_schedule_key(&mut self, fee_schedule_key: impl Into<Key>) -> &mut Self {
        self.body.data.fee_schedule_key = Some(fee_schedule_key.into());
        self
    }

    /// Sets the new Key which can pause and unpause the Token.
    ///
    /// If the token does not currently have a Pause key, transaction will resolve to `TokenHasNoPauseKey`.
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
            treasury: self.treasury_account_id.as_ref().map(AccountAddress::to_protobuf),
            admin_key: self.admin_key.as_ref().map(Key::to_protobuf),
            kyc_key: self.kyc_key.as_ref().map(Key::to_protobuf),
            freeze_key: self.freeze_key.as_ref().map(Key::to_protobuf),
            wipe_key: self.wipe_key.as_ref().map(Key::to_protobuf),
            supply_key: self.supply_key.as_ref().map(Key::to_protobuf),
            expiry: self.expires_at.map(Into::into),
            auto_renew_account: self
                .auto_renew_account_id
                .as_ref()
                .map(AccountAddress::to_protobuf),
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
