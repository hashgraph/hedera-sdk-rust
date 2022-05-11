use hedera_proto::services;

use crate::{AccountId, TransactionId};

pub trait ToTransactionDataProtobuf: Send + Sync {
    fn to_transaction_data_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: &TransactionId,
    ) -> services::transaction_body::Data;
}
