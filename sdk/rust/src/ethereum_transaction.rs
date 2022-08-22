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
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use serde_with::base64::Base64;
use serde_with::serde_as;
use tonic::transport::Channel;

use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    FileId,
    ToProtobuf,
    Transaction,
};

/// Submit an Ethereum transaction.
pub type EthereumTransaction = Transaction<EthereumTransactionData>;

#[serde_as]
#[derive(Debug, Default, serde::Serialize, serde::Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EthereumTransactionData {
    /// The raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    #[serde_as(as = "Base64")]
    pub ethereum_data: Vec<u8>,

    /// For large transactions (for example contract create) this should be used to
    /// set the FileId of an HFS file containing the callData
    /// of the ethereumData. The data in the ethereumData will be re-written with
    /// the callData element as a zero length string with the original contents in
    /// the referenced file at time of execution. The ethereumData will need to be
    /// "rehydrated" with the callData for signature validation to pass.
    pub call_data_file_id: Option<FileId>,

    /// The maximum amount that the payer of the hedera transaction
    /// is willing to pay to complete the transaction.
    pub max_gas_allowance_hbar: u64,
}

impl EthereumTransaction {
    /// Sets the raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    pub fn ethereum_data(&mut self, data: Vec<u8>) -> &mut Self {
        self.body.data.ethereum_data = data;
        self
    }

    /// Sets a file ID to find the raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    ///
    /// For large transactions (for example contract create) this should be used to
    /// set the [`FileId`] of an HFS file containing the callData
    /// of the ethereumData. The data in the ethereumData will be re-written with
    /// the callData element as a zero length string with the original contents in
    /// the referenced file at time of execution. The ethereumData will need to be
    /// "rehydrated" with the callData for signature validation to pass.
    ///
    pub fn call_data_file_id(&mut self, id: FileId) -> &mut Self {
        self.body.data.call_data_file_id = Some(id);
        self
    }

    /// Sets the maximum amount that the payer of the hedera transaction
    /// is willing to pay to complete the transaction.
    pub fn max_gas_allowance_hbar(&mut self, allowance: u64) -> &mut Self {
        self.body.data.max_gas_allowance_hbar = allowance;
        self
    }
}

#[async_trait]
impl TransactionExecute for EthereumTransactionData {
    // noinspection DuplicatedCode
    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status> {
        SmartContractServiceClient::new(channel).call_ethereum(request).await
    }
}

impl ToTransactionDataProtobuf for EthereumTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: crate::AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let call_data = self.call_data_file_id.as_ref().map(FileId::to_protobuf);

        services::transaction_body::Data::EthereumTransaction(services::EthereumTransactionBody {
            ethereum_data: self.ethereum_data.clone(),
            call_data,
            max_gas_allowance: self.max_gas_allowance_hbar as i64,
        })
    }
}

impl From<EthereumTransactionData> for AnyTransactionData {
    fn from(transaction: EthereumTransactionData) -> Self {
        Self::Ethereum(transaction)
    }
}
