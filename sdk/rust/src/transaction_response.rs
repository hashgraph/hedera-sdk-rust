use crate::{AccountId, TransactionHash, TransactionId};

#[derive(Debug)]
pub struct TransactionResponse {
    pub node_account_id: AccountId,

    pub transaction_id: TransactionId,

    pub transaction_hash: TransactionHash,
}
