use hedera_proto::services;
use time::{
    Duration,
    OffsetDateTime,
};

use crate::{
    AccountId,
    FromProtobuf,
    Key,
    PublicKey,
};

// TODO: pub ledger_id: LedgerId,
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
    pub proxy_received: u64, // TODO: Hbar

    /// The key for the account, which must sign in order to transfer out, or to modify the
    /// account in any way other than extending its expiration date.
    pub key: Key,

    /// Current balance of the referenced account.
    // TODO: use Hbar type
    pub balance: u64,

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
    pub alias: Option<PublicKey>,
    /// The ethereum transaction nonce associated with this account.
    pub ethereum_nonce: u64,
    //
    // TODO: pub staking: StakingInfo;
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

        // TODO: alias
        // let alias =
        //     if info.alias.is_empty() { Some(PublicKey::from_protobuf(&info.alias)?) } else { None };

        Ok(Self {
            account_id: AccountId::from_protobuf(account_id)?,
            contract_account_id: info.contract_account_id,
            is_deleted: info.deleted,
            proxy_received: info.proxy_received as u64,
            key: Key::from_protobuf(key)?,
            balance: info.balance as u64,
            expiration_time: info.expiration_time.map(Into::into),
            auto_renew_period: info.auto_renew_period.map(Into::into),
            account_memo: info.memo,
            owned_nfts: info.owned_nfts as u64,
            max_automatic_token_associations: info.max_automatic_token_associations as u32,
            alias: None,
            ethereum_nonce: info.ethereum_nonce as u64,
            is_receiver_signature_required: info.receiver_sig_required,
        })
    }
}
