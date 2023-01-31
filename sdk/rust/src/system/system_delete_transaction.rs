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

use hedera_proto::services;
use hedera_proto::services::file_service_client::FileServiceClient;
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use time::OffsetDateTime;
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
    BoxGrpcFuture,
    ContractId,
    Error,
    FileId,
    LedgerId,
    Transaction,
    ValidateChecksums,
};

/// Delete a file or smart contract - can only be done by a Hedera admin.
pub type SystemDeleteTransaction = Transaction<SystemDeleteTransactionData>;

/// Delete a file or smart contract - can only be done by a Hedera admin.
///
/// When it is deleted, it immediately disappears from the system as seen by the user,
/// but is still stored internally until the expiration time, at which time it
/// is truly and permanently deleted.
///
/// Until that time, it can be undeleted by the Hedera admin.
/// When a smart contract is deleted, the cryptocurrency account within it continues
/// to exist, and is not affected by the expiration time here.
///

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(default, rename_all = "camelCase"))]
pub struct SystemDeleteTransactionData {
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::TimestampNanoSeconds>>")
    )]
    expiration_time: Option<OffsetDateTime>,
    file_id: Option<FileId>,
    contract_id: Option<ContractId>,
}

impl SystemDeleteTransaction {
    /// Returns the contract ID which should be deleted.
    #[must_use]
    pub fn get_contract_id(&self) -> Option<ContractId> {
        self.data().contract_id
    }

    /// Sets the contract ID which should be deleted.
    pub fn contract_id(&mut self, id: impl Into<ContractId>) -> &mut Self {
        let data = self.data_mut();
        data.file_id = None;
        data.contract_id = Some(id.into());
        self
    }

    /// Returns the file ID which should be deleted.
    #[must_use]
    pub fn get_file_id(&self) -> Option<FileId> {
        self.data().file_id
    }

    /// Sets the file ID which should be deleted.
    pub fn file_id(&mut self, id: impl Into<FileId>) -> &mut Self {
        let data = self.data_mut();
        data.contract_id = None;
        data.file_id = Some(id.into());
        self
    }

    /// Returns the timestamp at which the "deleted" entity should
    /// truly be permanently deleted.
    #[must_use]
    pub fn get_expiration_time(&self) -> Option<OffsetDateTime> {
        self.data().expiration_time
    }

    /// Sets the timestamp at which the "deleted" file should
    /// truly be permanently deleted.
    pub fn expiration_time(&mut self, expiration_time: OffsetDateTime) -> &mut Self {
        self.data_mut().expiration_time = Some(expiration_time);
        self
    }
}

impl TransactionExecute for SystemDeleteTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async move {
            if self.file_id.is_some() {
                FileServiceClient::new(channel).system_delete(request).await
            } else {
                SmartContractServiceClient::new(channel).system_delete(request).await
            }
        })
    }
}

impl ValidateChecksums for SystemDeleteTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.file_id.validate_checksums(ledger_id)?;
        self.contract_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for SystemDeleteTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let expiration_time = self.expiration_time.map(Into::into);
        let contract_id = self.contract_id.to_protobuf();
        let file_id = self.file_id.to_protobuf();

        let id = match (contract_id, file_id) {
            (Some(contract_id), _) => {
                Some(services::system_delete_transaction_body::Id::ContractId(contract_id))
            }

            (_, Some(file_id)) => {
                Some(services::system_delete_transaction_body::Id::FileId(file_id))
            }

            _ => None,
        };

        services::transaction_body::Data::SystemDelete(services::SystemDeleteTransactionBody {
            expiration_time,
            id,
        })
    }
}

impl From<SystemDeleteTransactionData> for AnyTransactionData {
    fn from(transaction: SystemDeleteTransactionData) -> Self {
        Self::SystemDelete(transaction)
    }
}

impl FromProtobuf<services::SystemDeleteTransactionBody> for SystemDeleteTransactionData {
    fn from_protobuf(pb: services::SystemDeleteTransactionBody) -> crate::Result<Self> {
        use services::system_delete_transaction_body::Id;
        let (file_id, contract_id) = match pb.id {
            Some(Id::FileId(it)) => (Some(FileId::from_protobuf(it)?), None),
            Some(Id::ContractId(it)) => (None, Some(ContractId::from_protobuf(it)?)),
            None => (None, None),
        };

        Ok(Self { file_id, contract_id, expiration_time: pb.expiration_time.map(Into::into) })
    }
}
