use crate::{AccountId, TransactionHash, TransactionId};

/// Response from [`Transaction::execute`][crate::Transaction::execute].
///
/// When the client sends a node a transaction of any kind, the node replies with this, which
/// simply says that the transaction passed the pre-check (so the node will submit it to
/// the network).
///
/// To learn the consensus result, the client should later obtain a
/// receipt (free), or can buy a more detailed record (not free).
///
#[derive(Debug, serde::Serialize)]
// TODO: Deserialize
#[serde(rename_all = "camelCase")]
pub struct TransactionResponse {
    /// The account ID of the node that the transaction was submitted to.
    pub node_account_id: AccountId,

    /// The client-generated transaction ID of the transaction that was submitted.
    ///
    /// This can be used to lookup the transaction in an explorer.
    ///
    pub transaction_id: TransactionId,

    /// The client-generated SHA-384 hash of the transaction that was submitted.
    ///
    /// This can be used to lookup the transaction in an explorer.
    ///
    pub transaction_hash: TransactionHash,
}
