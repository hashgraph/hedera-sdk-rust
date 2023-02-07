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
use tonic::transport::Channel;

use crate::entity_id::AutoValidateChecksum;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    AccountId,
    Error,
    LedgerId,
    Transaction,
};

/// Mark an account as deleted, moving all its current hbars to another account.
///
/// It will remain in the ledger, marked as deleted, until it expires.
/// Transfers into it a deleted account will fail.
///
pub type AccountDeleteTransaction = Transaction<AccountDeleteTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct AccountDeleteTransactionData {
    /// The account ID which will receive all remaining hbars.
    transfer_account_id: Option<AccountId>,

    /// The account ID which should be deleted.
    account_id: Option<AccountId>,
}

impl AccountDeleteTransaction {
    /// Get the account ID which should be deleted.
    #[must_use]
    pub fn get_account_id(&self) -> Option<AccountId> {
        self.data().account_id
    }

    /// Sets the account ID which should be deleted.
    pub fn account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().account_id = Some(id);
        self
    }

    /// Get the account ID which will receive all remaining hbars.
    #[must_use]
    pub fn get_transfer_account_id(&self) -> Option<AccountId> {
        self.data().transfer_account_id
    }

    /// Sets the account ID which will receive all remaining hbars.
    pub fn transfer_account_id(&mut self, id: AccountId) -> &mut Self {
        self.data_mut().transfer_account_id = Some(id);
        self
    }
}

#[async_trait]
impl TransactionExecute for AccountDeleteTransactionData {
    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.transfer_account_id.validate_checksum_for_ledger_id(ledger_id)?;
        self.account_id.validate_checksum_for_ledger_id(ledger_id)
    }

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
        let account_id = self.account_id.to_protobuf();
        let transfer_account_id = self.transfer_account_id.to_protobuf();

        services::transaction_body::Data::CryptoDelete(services::CryptoDeleteTransactionBody {
            transfer_account_id,
            delete_account_id: account_id,
        })
    }
}

impl From<AccountDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: AccountDeleteTransactionData) -> Self {
        Self::AccountDelete(transaction)
    }
}

impl FromProtobuf<services::CryptoDeleteTransactionBody> for AccountDeleteTransactionData {
    fn from_protobuf(pb: services::CryptoDeleteTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            transfer_account_id: Option::from_protobuf(pb.transfer_account_id)?,
            account_id: Option::from_protobuf(pb.delete_account_id)?,
        })
    }
}
