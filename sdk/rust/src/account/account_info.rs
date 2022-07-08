use hedera_proto::services;
use time::{
    Duration,
    OffsetDateTime,
};

use crate::{
    AccountId,
    FromProtobuf,
    Key,
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
    pub deleted: bool,

    /// The account ID of the account to which this is proxy staked.
    #[deprecated]
    pub proxy_account_id: Option<AccountId>,

    /// The total number of hbars proxy staked to this account.
    pub proxy_received: u64, // TODO: Hbar

    /// The key for the account, which must sign in order to transfer_transaction out, or to modify the
    /// account in any way other than extending its expiration date.
    // TODO: serde
    #[serde(skip)]
    pub key: Key,

    /// Current balance of the referenced account.
    // TODO: use Hbar type
    pub balance: u64,

    /// The threshold amount, in hbars, at which a record is created of any
    /// transaction that decreases the balance of this account by more than the threshold.
    // TODO: use Hbar type
    #[deprecated]
    pub send_record_threshold: u64,

    /// The threshold amount, in hbars, at which a record is created of any
    /// transaction that increases the balance of this account by more than the threshold.
    // TODO: use Hbar type
    #[deprecated]
    pub receive_record_threshold: u64,

    /// If true, no transaction can transfer_transaction to this account unless signed by
    /// this account's key.
    pub receiver_signature_required: bool,

    /// The TimeStamp time at which this account is set to expire.
    pub expires_at: Option<OffsetDateTime>,

    /// The duration for expiration time will extend every this many seconds.
    pub auto_renew_period: Option<Duration>,
    //
    // All tokens related to this account.
    // TODO: pub token_relationships: HashMap<TokenId, TokenRelationship>,
    //
    /// The memo associated with the account.
    pub memo: String,

    /// The number of NFTs owned by this account
    pub owned_nfts: u64,

    /// The maximum number of tokens that an Account can be implicitly associated with.
    pub max_automatic_token_associations: u32,

    /// The alias of this account.
    pub alias: Option<Vec<u8>>, // TODO: Option<PublicKey>,
    //
    // The ledger ID
    // TODO: pub ledger_id: LedgerId,
    /// The ethereum transaction nonce associated with this account.
    pub ethereum_nonce: u64,
    //
    // TODO: pub staking: StakingInfo;
}

impl FromProtobuf for AccountInfo {
    type Protobuf = services::response::Response;

    #[allow(deprecated)]
    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, CryptoGetInfo, services::response::Response);
        let info = pb_getf!(response, account_info)?;
        let key = pb_getf!(info, key)?;
        let account_id = pb_getf!(info, account_id)?;

        let proxy_account_id = info
            .proxy_account_id
            .map(AccountId::from_protobuf)
            .transpose()?
            .filter(|id| id.num > 0);

        Ok(Self {
            account_id: AccountId::from_protobuf(account_id)?,
            contract_account_id: info.contract_account_id,
            deleted: info.deleted,
            #[allow(deprecated)]
            proxy_account_id,
            proxy_received: info.proxy_received as u64,
            // FIXME: key
            key: Key::from_protobuf(key)?,
            balance: info.balance as u64,
            send_record_threshold: info.generate_send_record_threshold,
            #[allow(deprecated)]
            receive_record_threshold: info.generate_receive_record_threshold,
            receiver_signature_required: info.receiver_sig_required,
            // FIXME: expires_at
            expires_at: None,
            // FIXME: auto_renew_period
            auto_renew_period: None,
            memo: info.memo,
            owned_nfts: info.owned_nfts as u64,
            max_automatic_token_associations: info.max_automatic_token_associations as u32,
            // FIXME: alias
            alias: None,
            ethereum_nonce: info.ethereum_nonce as u64,
        })
    }
}
