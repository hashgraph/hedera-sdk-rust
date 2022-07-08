use hedera_proto::services;
use serde::{
    Deserialize,
    Serialize,
};
use serde_with::serde_as;
use time::{
    Duration,
    OffsetDateTime,
};

use crate::{
    AccountId,
    ContractId,
    FromProtobuf,
    Key,
};

// TODO: token_relationships
// TODO: ledger_id
// TODO: staking_info
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractInfo {
    /// ID of the contract instance, in the format used by transactions.
    pub contract_id: ContractId,

    /// ID of the cryptocurrency account owned by the contract instance,
    /// in the format used in transactions.
    pub account_id: AccountId,

    /// ID of both the contract instance and the cryptocurrency account owned by the contract
    /// instance, in the format used by Solidity.
    pub contract_account_id: String,

    /// The admin key of the contract instance.
    pub admin_key: Option<Key>,

    /// The current time at which this contract instance (and its account) is set to expire.
    pub expires_at: Option<OffsetDateTime>,

    /// The auto renew period for this contract instance.
    pub auto_renew_period: Option<Duration>,

    /// Number of bytes of storage being used by this instance.
    pub storage: u64,

    /// The memo associated with the contract.
    pub memo: String,

    /// The current balance, in tinybars.
    pub balance: u64,

    /// Whether the contract has been deleted.
    pub deleted: bool,

    /// ID of the an account to charge for auto-renewal of this contract.
    pub auto_renew_account_id: Option<AccountId>,

    /// The maximum number of tokens that a contract can be implicitly associated with.
    pub max_automatic_token_associations: u32,
}

impl FromProtobuf for ContractInfo {
    type Protobuf = services::response::Response;

    #[allow(deprecated)]
    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let response = pb_getv!(pb, ContractGetInfo, services::response::Response);
        let info = pb_getf!(response, contract_info)?;
        let contract_id = pb_getf!(info, contract_id)?;
        let account_id = pb_getf!(info, account_id)?;
        let expires_at = info.expiration_time.map(Into::into);
        let auto_renew_period = info.auto_renew_period.map(Into::into);
        let auto_renew_account_id =
            info.auto_renew_account_id.map(AccountId::from_protobuf).transpose()?;
        let admin_key = info.admin_key.map(Key::from_protobuf).transpose()?;

        Ok(Self {
            contract_id: ContractId::from_protobuf(contract_id)?,
            account_id: AccountId::from_protobuf(account_id)?,
            contract_account_id: info.contract_account_id,
            deleted: info.deleted,
            balance: info.balance as u64,
            expires_at,
            auto_renew_period,
            auto_renew_account_id,
            memo: info.memo,
            max_automatic_token_associations: info.max_automatic_token_associations as u32,
            admin_key,
            storage: info.storage as u64,
        })
    }
}
