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
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use serde_with::skip_serializing_none;
use tonic::transport::Channel;

use crate::protobuf::ToProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Transaction,
};

/// Mark an account as deleted, moving all its current hbars to another account.
///
/// It will remain in the ledger, marked as deleted, until it expires.
/// Transfers into it a deleted account will fail.
///
pub type AccountDeleteTransaction = Transaction<AccountDeleteTransactionData>;

#[skip_serializing_none]
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
pub struct AccountDeleteTransactionData {
    /// The account ID which will receive all remaining hbars.
    pub transfer_account_id: Option<AccountId>,

    /// The account ID which should be deleted.
    pub delete_account_id: Option<AccountId>,
}

impl AccountDeleteTransaction {
    /// Sets the account ID which should be deleted.
    pub fn delete_account_id(&mut self, id: AccountId) -> &mut Self {
        self.body.data.delete_account_id = Some(id);
        self
    }

    /// Sets the account ID which will receive all remaining hbars.
    pub fn transfer_account_id(&mut self, id: AccountId) -> &mut Self {
        self.body.data.transfer_account_id = Some(id);
        self
    }
}

#[async_trait]
impl TransactionExecute for AccountDeleteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        CryptoServiceClient::new(channel).crypto_delete(request).await
    }
}

impl ToTransactionDataProtobuf for AccountDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let delete_account_id = self.delete_account_id.as_ref().map(AccountId::to_protobuf);
        let transfer_account_id = self.transfer_account_id.as_ref().map(AccountId::to_protobuf);

        services::transaction_body::Data::CryptoDelete(services::CryptoDeleteTransactionBody {
            transfer_account_id,
            delete_account_id,
        })
    }
}

impl From<AccountDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: AccountDeleteTransactionData) -> Self {
        Self::AccountDelete(transaction)
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;

    use crate::transaction::{
        AnyTransaction,
        AnyTransactionData,
    };
    use crate::{
        AccountDeleteTransaction,
        AccountId,
    };

    // language=JSON
    const ACCOUNT_DELETE_TRANSACTION_JSON: &str = r#"{
  "$type": "accountDelete",
  "transferAccountId": "0.0.1001",
  "deleteAccountId": "0.0.1002"
}"#;

    #[test]
    fn it_should_serialize() -> anyhow::Result<()> {
        let mut transaction = AccountDeleteTransaction::new();

        transaction
            .transfer_account_id(AccountId::from(1001))
            .delete_account_id(AccountId::from(1002));

        let transaction_json = serde_json::to_string_pretty(&transaction)?;

        assert_eq!(transaction_json, ACCOUNT_DELETE_TRANSACTION_JSON);

        Ok(())
    }

    #[test]
    fn it_should_deserialize() -> anyhow::Result<()> {
        let transaction: AnyTransaction = serde_json::from_str(ACCOUNT_DELETE_TRANSACTION_JSON)?;

        let data = assert_matches!(transaction.body.data, AnyTransactionData::AccountDelete(transaction) => transaction);

        assert_eq!(data.transfer_account_id, Some(AccountId::from(1001)));
        assert_eq!(data.delete_account_id, Some(AccountId::from(1002)));

        Ok(())
    }
}
