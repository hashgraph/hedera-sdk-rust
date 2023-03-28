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
use hedera_proto::services::util_service_client::UtilServiceClient;

use crate::entity_id::ValidateChecksums;
use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::Transaction;

/// Random Number Generator Transaction.
pub type PrngTransaction = Transaction<PrngTransactionData>;

#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct PrngTransactionData {
    range: Option<u32>,
}

impl PrngTransaction {
    /// Returns the upper-bound for the random number.
    pub fn get_range(&self) -> Option<u32> {
        self.data().range
    }

    /// Sets the upper-bound for the random number.
    pub fn range(&mut self, range: u32) {
        self.data_mut().range = Some(range)
    }
}

impl FromProtobuf<services::UtilPrngTransactionBody> for PrngTransactionData {
    fn from_protobuf(pb: services::UtilPrngTransactionBody) -> crate::Result<Self> {
        Ok(Self { range: (pb.range != 0).then_some(pb.range as u32) })
    }
}

impl ToProtobuf for PrngTransactionData {
    type Protobuf = services::UtilPrngTransactionBody;
    fn to_protobuf(&self) -> Self::Protobuf {
        services::UtilPrngTransactionBody { range: self.range.unwrap_or_default() as i32 }
    }
}

impl TransactionData for PrngTransactionData {}

impl From<PrngTransactionData> for AnyTransactionData {
    fn from(value: PrngTransactionData) -> Self {
        Self::Prng(value)
    }
}

impl ValidateChecksums for PrngTransactionData {
    fn validate_checksums(&self, _ledger_id: &crate::LedgerId) -> crate::Result<()> {
        Ok(())
    }
}

impl ToSchedulableTransactionDataProtobuf for PrngTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::UtilPrng(self.to_protobuf())
    }
}

impl ToTransactionDataProtobuf for PrngTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::UtilPrng(self.to_protobuf())
    }
}

impl TransactionExecute for PrngTransactionData {
    fn execute(
        &self,
        channel: tonic::transport::Channel,
        request: services::Transaction,
    ) -> crate::BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { UtilServiceClient::new(channel).prng(request).await })
    }
}
