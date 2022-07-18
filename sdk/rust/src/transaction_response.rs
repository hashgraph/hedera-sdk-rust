use crate::{
    AccountId,
    Client,
    TransactionHash,
    TransactionId,
    TransactionReceipt,
    TransactionReceiptQuery,
};

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

// TODO: get_record
// TODO: get_successful_record
impl TransactionResponse {
    /// Get the receipt of this transaction.
    /// Will wait for consensus.
    /// Will return an `Error::ReceiptStatus` for a failing receipt.
    pub async fn get_receipt(&self, client: &Client) -> crate::Result<TransactionReceipt> {
        Ok(TransactionReceiptQuery::new()
            .transaction_id(self.transaction_id)
            .node_account_ids([self.node_account_id])
            .validate_status(true)
            .execute(client)
            .await?)
    }
}
