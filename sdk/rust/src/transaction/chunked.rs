use std::num::NonZeroUsize;

use hedera_proto::services;
use tonic::transport::Channel;

use super::TransactionExecute;
use crate::entity_id::ValidateChecksums;
use crate::execute::Execute;
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    Transaction,
    TransactionHash,
    TransactionId,
    TransactionResponse,
};

/// Per transaction chunk data (you'd add this to any chunked transaction)
pub struct ChunkData {
    pub(crate) max_chunks: usize,
    pub(crate) chunk_size: NonZeroUsize,
    pub(crate) message: Vec<u8>,
}

impl ChunkData {
    pub(crate) fn used_chunks(&self) -> usize {
        if self.message.len() == 0 {
            return 1;
        }

        // div ceil algorithm, fun fact: the intrinsic `div_ceil` can't get rid of the panic (it's unstable anyway)
        (self.message.len() + self.chunk_size.get()) / self.chunk_size
    }

    pub(crate) fn message_chunk(&self, chunk_info: &ChunkInfo) -> &[u8] {
        debug_assert!(chunk_info.current < self.max_chunks);
        let offset = self.chunk_size.get() * chunk_info.current;
        let len = self.chunk_size.get();

        &self.message[offset..][..len]
    }

    pub(crate) fn max_message_len(&self) -> usize {
        self.max_chunks * self.chunk_size.get()
    }
}

pub struct ChunkInfo {
    /// Current chunk # out of [`max_chunks`](ChunkData.max_chunks) total.
    pub(crate) current: usize,

    /// How many chunks there will be.
    /// if `chunks` was a `[ChunkInfo]` this would be the length.
    pub(crate) total: usize,

    /// transaction ID for transaction 0 (if `current` is 0 this is that one).
    pub(crate) initial_transaction_id: TransactionId,

    /// transaction ID for the current transaction.
    pub(crate) current_transaction_id: TransactionId,

    /// ID for the account this transaction will be submitted to.
    pub(crate) node_account_id: AccountId,
}

impl ChunkInfo {
    #[must_use]
    pub(crate) fn assert_single_transaction(&self) -> (TransactionId, AccountId) {
        assert!(self.current == 0 && self.total == 1);
        (self.current_transaction_id, self.node_account_id)
    }

    #[must_use]
    pub(crate) const fn single(transaction_id: TransactionId, node_account_id: AccountId) -> Self {
        Self::initial(1, transaction_id, node_account_id)
    }

    #[must_use]
    pub(crate) const fn initial(
        total: usize,
        transaction_id: TransactionId,
        node_account_id: AccountId,
    ) -> Self {
        Self {
            current: 0,
            total,
            initial_transaction_id: transaction_id,
            current_transaction_id: transaction_id,
            node_account_id,
        }
    }
}

pub(super) struct FirstChunkView<'a, D> {
    pub(super) transaction: &'a Transaction<D>,
    pub(super) total_chunks: usize,
}

impl<'a, D> Execute for FirstChunkView<'a, D>
where
    D: TransactionExecute,
{
    type GrpcRequest = services::Transaction;

    type GrpcResponse = services::TransactionResponse;

    type Context = TransactionHash;

    type Response = TransactionResponse;

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        self.transaction.body.node_account_ids.as_deref()
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        self.transaction.get_transaction_id()
    }

    fn requires_transaction_id(&self) -> bool {
        true
    }

    fn make_request(
        &self,
        transaction_id: &Option<TransactionId>,
        node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)> {
        assert!(self.transaction.is_frozen());

        self.transaction.make_request_inner(&ChunkInfo::initial(
            self.total_chunks,
            transaction_id.ok_or(Error::NoPayerAccountOrTransactionId)?,
            node_account_id,
        ))
    }

    fn execute(
        &self,
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> BoxGrpcFuture<'_, Self::GrpcResponse> {
        self.transaction.body.data.execute(channel, request)
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
}

impl<'a, D: ValidateChecksums> ValidateChecksums for FirstChunkView<'a, D> {
    fn validate_checksums(&self, ledger_id: &crate::LedgerId) -> Result<(), Error> {
        self.transaction.validate_checksums(ledger_id)
    }
}

pub(super) struct ChunkView<'a, D> {
    pub(super) transaction: &'a Transaction<D>,
    pub(super) initial_transaction_id: TransactionId,
    pub(super) current_chunk: usize,
    pub(super) total_chunks: usize,
}

impl<'a, D> Execute for ChunkView<'a, D>
where
    D: TransactionExecute,
{
    type GrpcRequest = services::Transaction;

    type GrpcResponse = services::TransactionResponse;

    type Context = TransactionHash;

    type Response = TransactionResponse;

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        self.transaction.body.node_account_ids.as_deref()
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        None
    }

    fn requires_transaction_id(&self) -> bool {
        true
    }

    fn make_request(
        &self,
        transaction_id: &Option<TransactionId>,
        node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)> {
        assert!(self.transaction.is_frozen());

        self.transaction.make_request_inner(&ChunkInfo {
            total: self.total_chunks,
            current: self.current_chunk,
            initial_transaction_id: self.initial_transaction_id,
            node_account_id,
            current_transaction_id: transaction_id.ok_or(Error::NoPayerAccountOrTransactionId)?,
        })
    }

    fn execute(
        &self,
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> BoxGrpcFuture<'_, Self::GrpcResponse> {
        self.transaction.body.data.execute(channel, request)
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
}

impl<'a, D: ValidateChecksums> ValidateChecksums for ChunkView<'a, D> {
    fn validate_checksums(&self, ledger_id: &crate::LedgerId) -> Result<(), Error> {
        self.transaction.validate_checksums(ledger_id)?;
        self.initial_transaction_id.validate_checksums(ledger_id)?;

        Ok(())
    }
}
