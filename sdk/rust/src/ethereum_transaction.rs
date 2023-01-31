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
use hedera_proto::services::smart_contract_service_client::SmartContractServiceClient;
use tonic::transport::Channel;

use crate::protobuf::FromProtobuf;
use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionExecute,
};
use crate::{
    BoxGrpcFuture,
    Error,
    FileId,
    LedgerId,
    ToProtobuf,
    Transaction,
    ValidateChecksums,
};

/// Submit an Ethereum transaction.
pub type EthereumTransaction = Transaction<EthereumTransactionData>;

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct EthereumTransactionData {
    /// The raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    #[cfg_attr(feature = "ffi", serde(with = "serde_with::As::<serde_with::base64::Base64>"))]
    ethereum_data: Vec<u8>,

    /// For large transactions (for example contract create) this should be used to
    /// set the FileId of an HFS file containing the call_data
    /// of the ethereum_data. The data in the ethereum_data will be re-written with
    /// the call_data element as a zero length string with the original contents in
    /// the referenced file at time of execution. The ethereum_data will need to be
    /// "rehydrated" with the call_data for signature validation to pass.
    call_data_file_id: Option<FileId>,

    /// The maximum amount that the payer of the hedera transaction
    /// is willing to pay to complete the transaction.
    max_gas_allowance_hbar: u64,
}

impl EthereumTransaction {
    /// Returns the raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    #[must_use]
    pub fn get_ethereum_data(&self) -> &[u8] {
        &self.data().ethereum_data
    }

    /// Sets the raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    pub fn ethereum_data(&mut self, data: Vec<u8>) -> &mut Self {
        self.data_mut().ethereum_data = data;
        self
    }

    /// Returns the file ID to find the raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    #[must_use]
    pub fn get_call_data_file_id(&self) -> Option<FileId> {
        self.data().call_data_file_id
    }

    /// Sets a file ID to find the raw Ethereum transaction (RLP encoded type 0, 1, and 2).
    ///
    /// For large transactions (for example contract create) this should be used to
    /// set the [`FileId`] of an HFS file containing the `call_data`
    /// of the `ethereum_data`. The data in `the ethereum_data` will be re-written with
    /// the `call_data` element as a zero length string with the original contents in
    /// the referenced file at time of execution. `The ethereum_data` will need to be
    /// "rehydrated" with the `call_data` for signature validation to pass.
    pub fn call_data_file_id(&mut self, id: FileId) -> &mut Self {
        self.data_mut().call_data_file_id = Some(id);
        self
    }

    /// Returns the maximum amount that the payer of the hedera transaction
    /// is willing to pay to complete the transaction.
    #[must_use]
    pub fn get_max_gas_allowance_hbar(&self) -> u64 {
        self.data().max_gas_allowance_hbar
    }

    /// Sets the maximum amount that the payer of the hedera transaction
    /// is willing to pay to complete the transaction.
    pub fn max_gas_allowance_hbar(&mut self, allowance: u64) -> &mut Self {
        self.data_mut().max_gas_allowance_hbar = allowance;
        self
    }
}

impl TransactionExecute for EthereumTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { SmartContractServiceClient::new(channel).call_ethereum(request).await })
    }
}

impl ValidateChecksums for EthereumTransactionData {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.call_data_file_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for EthereumTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        _node_account_id: crate::AccountId,
        _transaction_id: &crate::TransactionId,
    ) -> services::transaction_body::Data {
        let call_data = self.call_data_file_id.to_protobuf();

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

impl FromProtobuf<services::EthereumTransactionBody> for EthereumTransactionData {
    fn from_protobuf(pb: services::EthereumTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            ethereum_data: pb.ethereum_data,
            call_data_file_id: Option::from_protobuf(pb.call_data)?,
            max_gas_allowance_hbar: pb.max_gas_allowance as u64,
        })
    }
}
