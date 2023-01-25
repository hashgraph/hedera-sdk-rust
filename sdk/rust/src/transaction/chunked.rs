/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2023 - 2023 Hedera Hashgraph, LLC
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

use std::num::NonZeroUsize;

use futures_core::future::BoxFuture;
use hedera_proto::services;
use time::Duration;
use tonic::transport::Channel;

use super::protobuf::ToTransactionBodyProtobuf;
use super::DEFAULT_TRANSACTION_VALID_DURATION;
use crate::client::Operator;
use crate::entity_id::AutoValidateChecksum;
use crate::execute::Execute;
use crate::protobuf::ToProtobuf;
use crate::signer::AnySigner;
use crate::{
    AccountId,
    Client,
    Error,
    Hbar,
    HbarUnit,
    LedgerId,
    TransactionHash,
    TransactionId,
    TransactionResponse,
};

pub struct ChunkedTransaction<D> {
    body: ChunkedTransactionBody<D>,

    signers: Vec<AnySigner>,
}

impl<D> ChunkedTransaction<D> {
    pub(crate) fn max_message_size(&self) -> usize {
        self.body.max_bytes_per_chunk.get() * self.body.max_chunks
    }

    /// Returns the number of chunks that would be used if this transaction were to be executed.
    ///
    /// if `self.message` is empty, this will return `1`, otherwise this will return `ceil(self.message.len() / self.max_bytes_per_chunk)
    ///
    /// This function does *not* care about `self.max_chunks`.
    pub(crate) fn used_chunks(&self) -> usize {
        if self.body.message.is_empty() {
            return 1;
        }

        let lhs = self.body.message.len();
        let rhs = self.body.max_bytes_per_chunk;

        // todo: replace with `div_ceil` when that's stable.
        let (quotient, remainder) = (lhs / rhs, lhs % rhs);

        quotient + (remainder != 0) as usize
    }
}

pub struct ChunkedTransactionBody<D> {
    pub(crate) data: D,

    pub(crate) node_account_ids: Option<Vec<AccountId>>,

    pub(crate) transaction_valid_duration: Option<Duration>,

    pub(crate) max_transaction_fee: Option<Hbar>,

    pub(crate) transaction_memo: String,

    // for once there's a difference between `None` and `Vec::empty`.
    pub(crate) transaction_ids: Option<Vec<TransactionId>>,

    pub(crate) operator: Option<Operator>,

    pub(crate) is_frozen: bool,

    pub(crate) max_chunks: usize,

    pub(crate) max_bytes_per_chunk: NonZeroUsize,

    // unlike non-chunked transactions, all chunked transactions must currently handle `message` so...
    pub(crate) message: Vec<u8>,
}

pub(crate) struct ChunkInfo<'a> {
    /// Current chunk # out of `max_chunks` total.
    current: usize,

    /// How many chunks there will be.
    /// if `chunks` was a `[Chunk]` this would be the length.
    total: usize,

    /// this chunk's message.
    message: &'a [u8],

    /// transaction ID for transaction 0 (if `current` is 0 this is that one).
    initial_transaction_id: TransactionId,

    /// transaction ID for the current transaction.
    current_transaction_id: TransactionId,

    /// ID for the account this transaction will be submitted to.
    node_account_id: AccountId,
}

impl<D> ToTransactionBodyProtobuf for ChunkedTransaction<D>
where
    D: ToChunkedTransactionDataProtobuf + TransactionChunkExecute,
{
    type Request<'a> = ChunkInfo<'a>;

    #[allow(deprecated)]
    fn to_transaction_body_protobuf(
        &self,
        request: Self::Request<'_>,
    ) -> services::TransactionBody {
        // assert!(self.is_frozen());
        let data = self.body.data.to_chunked_transaction_data_protobuf(&request);

        let max_transaction_fee = self
            .body
            .max_transaction_fee
            .unwrap_or_else(|| self.body.data.default_max_transaction_fee());

        services::TransactionBody {
            data: Some(data),
            transaction_id: Some(request.current_transaction_id.to_protobuf()),
            transaction_valid_duration: Some(
                self.body
                    .transaction_valid_duration
                    .unwrap_or(DEFAULT_TRANSACTION_VALID_DURATION)
                    .into(),
            ),
            memo: self.body.transaction_memo.clone(),
            node_account_id: Some(request.node_account_id.to_protobuf()),
            generate_record: false,
            transaction_fee: max_transaction_fee.to_tinybars() as u64,
        }
    }
}

trait ToChunkedTransactionDataProtobuf {
    fn to_chunked_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo<'_>,
    ) -> services::transaction_body::Data;
}

trait TransactionChunkExecute {
    fn default_max_transaction_fee(&self) -> Hbar {
        Hbar::from_unit(2, HbarUnit::Hbar)
    }

    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> crate::Result<()>;

    fn execute_chunk(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxFuture<'_, Result<tonic::Response<services::TransactionResponse>, tonic::Status>>;
}

impl<D> ChunkedTransaction<D> {
    async fn execute_all_with_optional_timeout(
        &self,
        client: &Client,
        timeout_per_chunk: Option<std::time::Duration>,
    ) -> crate::Result<Vec<TransactionResponse>>
    where
        D: TransactionChunkExecute + ToChunkedTransactionDataProtobuf + Sync,
    {
        // todo: check frozen.
        if self.body.message.len() > self.max_message_size() {
            todo!("error: message too big")
        }

        let chunks = self.body.message.chunks(self.max_message_size());

        let mut responses: Vec<TransactionResponse> = Vec::with_capacity(self.used_chunks());

        let mut chunks =
            chunks.chain(self.body.message.is_empty().then(|| [].as_slice())).enumerate();

        let initial_transaction_id = {
            let message = chunks.next().map_or([].as_slice(), |it| it.1);

            let resp = crate::execute::execute(
                client,
                &FirstChunkView {
                    explicit_transaction_id: todo!(
                        "not sure what to do about multiple transaction ids yet"
                    ),
                    transaction: &self,
                    chunk_message: message,
                },
                timeout_per_chunk,
            )
            .await?;

            let initial_transaction_id = resp.transaction_id;
            responses.push(resp);

            initial_transaction_id
        };

        for (index, message) in chunks {
            responses.push(
                crate::execute::execute(
                    client,
                    &ChunkView {
                        explicit_transaction_id: todo!(
                            "not sure what to do about multiple transaction ids yet"
                        ),
                        transaction: &self,
                        chunk_message: message,
                        index,
                        initial_transaction_id,
                    },
                    timeout_per_chunk,
                )
                .await?,
            );
        }

        Ok(responses)
    }
}
struct FirstChunkView<'a, D> {
    explicit_transaction_id: Option<TransactionId>,
    transaction: &'a ChunkedTransaction<D>,
    chunk_message: &'a [u8],
}

// execute first -> execute rest.

impl<'a, D> Execute for FirstChunkView<'a, D>
where
    D: TransactionChunkExecute + ToChunkedTransactionDataProtobuf,
{
    type GrpcRequest = services::Transaction;

    type GrpcResponse = services::TransactionResponse;

    type Context = TransactionHash;

    type Response = TransactionResponse;

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        self.transaction.body.node_account_ids.as_deref()
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        self.explicit_transaction_id
    }

    fn requires_transaction_id(&self) -> bool {
        true
    }

    fn make_request(
        &self,
        transaction_id: &Option<TransactionId>,
        node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)> {
        // assert!(self.is_frozen());
        let transaction_id = transaction_id.ok_or(Error::NoPayerAccountOrTransactionId)?;

        super::protobuf::make_request(
            self.transaction,
            ChunkInfo {
                current: 0,
                total: self.transaction.used_chunks(),
                message: self.chunk_message,
                initial_transaction_id: transaction_id,
                current_transaction_id: transaction_id,
                node_account_id,
            },
            &self.transaction.signers,
            self.transaction.body.operator.as_ref(),
        )
    }

    fn execute<'life0, 'async_trait>(
        &'life0 self,
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> BoxFuture<'async_trait, Result<tonic::Response<Self::GrpcResponse>, tonic::Status>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.transaction.body.data.execute_chunk(channel, request)
    }

    fn make_response(
        &self,
        _response: Self::GrpcResponse,
        context: Self::Context,
        node_account_id: AccountId,
        transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response> {
        Ok(TransactionResponse {
            node_account_id,
            transaction_id: transaction_id.unwrap(),
            transaction_hash: context,
            validate_status: true,
        })
    }

    fn make_error_pre_check(
        &self,
        status: services::ResponseCodeEnum,
        transaction_id: Option<TransactionId>,
    ) -> crate::Error {
        if let Some(transaction_id) = transaction_id {
            crate::Error::TransactionPreCheckStatus { status, transaction_id }
        } else {
            crate::Error::TransactionNoIdPreCheckStatus { status }
        }
    }

    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32> {
        Ok(response.node_transaction_precheck_code)
    }

    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.explicit_transaction_id.validate_checksum_for_ledger_id(ledger_id)?;
        // self.transaction.validate_checksums_for

        Ok(())
    }
}

struct ChunkView<'a, D> {
    explicit_transaction_id: Option<TransactionId>,
    index: usize,
    transaction: &'a ChunkedTransaction<D>,
    chunk_message: &'a [u8],
    initial_transaction_id: TransactionId,
}

impl<'a, D> Execute for ChunkView<'a, D>
where
    D: TransactionChunkExecute + ToChunkedTransactionDataProtobuf,
{
    type GrpcRequest = services::Transaction;

    type GrpcResponse = services::TransactionResponse;

    type Context = TransactionHash;

    type Response = TransactionResponse;

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        self.transaction.body.node_account_ids.as_deref()
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        self.explicit_transaction_id
    }

    fn requires_transaction_id(&self) -> bool {
        true
    }

    fn make_request(
        &self,
        transaction_id: &Option<TransactionId>,
        node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)> {
        // assert!(self.is_frozen());
        let transaction_id = transaction_id.ok_or(Error::NoPayerAccountOrTransactionId)?;

        super::protobuf::make_request(
            self.transaction,
            ChunkInfo {
                current: self.index,
                total: self.transaction.used_chunks(),
                message: self.chunk_message,
                initial_transaction_id: self.initial_transaction_id,
                current_transaction_id: transaction_id,
                node_account_id,
            },
            &self.transaction.signers,
            self.transaction.body.operator.as_ref(),
        )
    }

    fn execute<'life0, 'async_trait>(
        &'life0 self,
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> BoxFuture<'async_trait, Result<tonic::Response<Self::GrpcResponse>, tonic::Status>>
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        self.transaction.body.data.execute_chunk(channel, request)
    }

    fn make_response(
        &self,
        _response: Self::GrpcResponse,
        context: Self::Context,
        node_account_id: AccountId,
        transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response> {
        Ok(TransactionResponse {
            node_account_id,
            transaction_id: transaction_id.unwrap(),
            transaction_hash: context,
            validate_status: true,
        })
    }

    fn make_error_pre_check(
        &self,
        status: services::ResponseCodeEnum,
        transaction_id: Option<TransactionId>,
    ) -> crate::Error {
        if let Some(transaction_id) = transaction_id {
            crate::Error::TransactionPreCheckStatus { status, transaction_id }
        } else {
            crate::Error::TransactionNoIdPreCheckStatus { status }
        }
    }

    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32> {
        Ok(response.node_transaction_precheck_code)
    }

    fn validate_checksums_for_ledger_id(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.explicit_transaction_id.validate_checksum_for_ledger_id(ledger_id)?;
        // self.transaction.validate_checksums_for

        Ok(())
    }
}
