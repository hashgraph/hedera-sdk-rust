use hedera_proto::services;

use crate::{
    AccountId,
    FromProtobuf,
};

/// Response from [`AccountBalanceQuery`][crate::AccountBalanceQuery].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalance {
    /// The account that is being referenced.
    pub account_id: AccountId,

    /// Current balance of the referenced account.
    // TODO: use Hbar type
    pub hbars: u64,
}

impl FromProtobuf<services::response::Response> for AccountBalance {
    fn from_protobuf(pb: services::response::Response) -> crate::Result<Self> {
        let response = pb_getv!(pb, CryptogetAccountBalance, services::response::Response);

        let account_id = pb_getf!(response, account_id)?;
        let account_id = AccountId::from_protobuf(account_id)?;

        let balance = response.balance;

        Ok(Self { account_id, hbars: balance })
    }
}
