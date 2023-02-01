use std::num::NonZeroUsize;

use crate::{
    AccountId,
    TransactionId,
};

/// Per transaction chunk data (you'd add this to any chunked transaction)
pub(crate) struct ChunkData {
    pub(crate) max_chunks: usize,
    pub(crate) chunk_size: NonZeroUsize,
    pub(crate) message: Vec<u8>,
}

impl ChunkData {
    pub(crate) fn required_chunks(&self) -> usize {
        if self.message.len() == 0 {
            return 1;
        }

        // div ceil algorithm, fun fact: the intrinsic `div_ceil` can't get rid
        (self.message.len() + self.chunk_size.get()) / self.chunk_size
    }

    pub(crate) fn message_chunk(&self, chunk_info: &ChunkInfo) -> &[u8] {
        debug_assert!(chunk_info.current < self.max_chunks);
        let offset = self.chunk_size.get() * chunk_info.current;
        let len = self.chunk_size.get();

        &self.message[offset..][..len]
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
