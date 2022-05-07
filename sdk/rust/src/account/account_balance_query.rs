use hedera_proto::services;
use services::crypto_get_account_balance_query::BalanceSource;

use crate::query::Query;
use crate::{
    AccountBalance, AccountIdOrAlias, Client, ContractIdOrEvmAddress, FromProtobuf, ToProtobuf
};

/// Get the balance of a cryptocurrency account.
///
/// This returns only the balance, so it is a smaller reply
/// than [`AccountInfoQuery`][crate::AccountInfoQuery], which returns the balance plus
/// additional information.
///
pub type AccountBalanceQuery = Query<AccountBalanceQueryData>;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize, Debug)]
pub struct AccountBalanceQueryData {
    account_id: Option<AccountIdOrAlias>,
    contract_id: Option<ContractIdOrEvmAddress>,
}

impl AccountBalanceQuery {
    /// Sets the account ID for which information is requested.
    ///
    /// This is mutually exclusive with [`contract_id`](#method.contract_id).
    ///
    pub fn account_id(&mut self, id: AccountIdOrAlias) -> &mut Self {
        self.data.account_id = Some(id.into());
        self
    }

    /// Sets the contract ID for which information is requested.
    ///
    /// This is mutually exclusive with [`account_id`](#method.account_id).
    ///
    pub fn contract_id(&mut self, id: ContractIdOrEvmAddress) -> &mut Self {
        self.data.contract_id = Some(id.into());
        self
    }
}

impl ToProtobuf for AccountBalanceQueryData {
    type Protobuf = services::Query;

    fn to_protobuf(&self) -> Self::Protobuf {
        let source = match (&self.account_id, &self.contract_id) {
            (Some(id), _) => Some(BalanceSource::AccountId(id.to_protobuf())),
            (_, Some(id)) => todo!(), // Some(BalanceSource::ContractId(id.to_protobuf())),
            _ => None,
        };

        services::Query {
            query: Some(services::query::Query::CryptogetAccountBalance(
                services::CryptoGetAccountBalanceQuery { balance_source: source, header: None },
            )),
        }
    }
}

impl AccountBalanceQuery {
    pub async fn execute(&self, client: &Client) -> crate::Result<AccountBalance> {
        let request = self.data.to_protobuf();

        let response = client.crypto().crypto_get_balance(request).await?;
        let response = response.into_inner();

        AccountBalance::from_protobuf(response)
    }
}
