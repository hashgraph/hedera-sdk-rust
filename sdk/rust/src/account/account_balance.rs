use hedera_proto::services;

use crate::{AccountId, FromProtobuf};

/// Response from [`AccountBalanceQuery`][crate::AccountBalanceQuery].
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountBalanceResponse {
    /// The account that is being referenced.
    pub account_id: AccountId,

    /// Current balance of the referenced account.
    // TODO: use Hbar type
    pub balance: u64,
    //
    // Current balance of each token possessed by the referenced account.
    // TODO: pub tokens: HashMap<TokenId, AccountTokenBalance>,
}

impl FromProtobuf for AccountBalanceResponse {
    type Protobuf = services::response::Response;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self> {
        let response = pb_getv!(pb, CryptogetAccountBalance, services::response::Response);

        let account_id = pb_getf!(response, account_id)?;
        let account_id = AccountId::from_protobuf(account_id)?;

        let balance = response.balance;

        Ok(Self { account_id, balance })
    }
}
