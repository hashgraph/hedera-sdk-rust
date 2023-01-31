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
use hedera_proto::services::file_service_client::FileServiceClient;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use tonic::transport::Channel;

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
    ContractId,
    Error,
    FileId,
    LedgerId,
    Transaction,
    ValidateChecksums,
};

/// Undelete a file or smart contract that was deleted by a [`SystemDeleteTransaction`](crate::SystemDeleteTransaction).
pub type SystemUndeleteTransaction = Transaction<SystemUndeleteTransactionData>;

/// Undelete a file or smart contract that was deleted by  [`SystemDeleteTransaction`](crate::SystemDeleteTransaction).
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct SystemUndeleteTransactionData {
    file_id: Option<FileId>,
    contract_id: Option<ContractId>,
}

impl SystemUndeleteTransaction {
    /// Returns the contract ID to undelete.
    #[must_use]
    pub fn get_contract_id(&self) -> Option<ContractId> {
        self.data().contract_id
    }

    /// Sets the contract ID to undelete.
    pub fn contract_id(&mut self, id: impl Into<ContractId>) -> &mut Self {
        let data = self.data_mut();
        data.file_id = None;
        data.contract_id = Some(id.into());
        self
    }

    /// Returns the file ID to undelete.
    #[must_use]
    pub fn get_file_id(&self) -> Option<FileId> {
        self.data().file_id
    }

    /// Sets the file ID to undelete.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        let data = self.data_mut();
        data.contract_id = None;
        data.file_id = Some(id.into());
        self
    }
}

#[async_trait]
impl TransactionExecute for SystemUndeleteTransactionData {
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        if self.file_id.is_some() {
            FileServiceClient::new(channel).system_undelete(request).await
        } else {
            SmartContractServiceClient::new(channel).system_undelete(request).await
        }
    }
}

impl ValidateChecksums for SystemUndeleteTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.contract_id.validate_checksums(ledger_id)?;
        self.file_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for SystemUndeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let contract_id = self.contract_id.to_protobuf();
        let file_id = self.file_id.to_protobuf();

        let id = match (contract_id, file_id) {
            (Some(contract_id), _) => {
                Some(services::system_undelete_transaction_body::Id::ContractId(contract_id))
            }

            (_, Some(file_id)) => {
                Some(services::system_undelete_transaction_body::Id::FileId(file_id))
            }

            _ => None,
        };

        services::transaction_body::Data::SystemUndelete(services::SystemUndeleteTransactionBody {
            id,
        })
    }
}

impl From<SystemUndeleteTransactionData> for AnyTransactionData {
    fn from(transaction: SystemUndeleteTransactionData) -> Self {
        Self::SystemUndelete(transaction)
    }
}

impl FromProtobuf<services::SystemUndeleteTransactionBody> for SystemUndeleteTransactionData {
    fn from_protobuf(pb: services::SystemUndeleteTransactionBody) -> crate::Result<Self> {
        use services::system_undelete_transaction_body::Id;
        let (file_id, contract_id) = match pb.id {
            Some(Id::FileId(it)) => (Some(FileId::from_protobuf(it)?), None),
            Some(Id::ContractId(it)) => (None, Some(ContractId::from_protobuf(it)?)),
            None => (None, None),
        };

        Ok(Self { file_id, contract_id })
    }
}
