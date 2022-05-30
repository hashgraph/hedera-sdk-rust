use hedera_proto::services;

use crate::{FromProtobuf, TransactionReceipt};

/// Response from [`TransactionReceiptQuery`][crate::TransactionReceiptQuery].
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceiptResponse {
    pub receipt: TransactionReceipt,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub duplicate_receipts: Vec<TransactionReceipt>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub child_receipts: Vec<TransactionReceipt>,
}

impl FromProtobuf for TransactionReceiptResponse {
    type Protobuf = services::response::Response;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        let pb = pb_getv!(pb, TransactionGetReceipt, services::response::Response);

        let receipt = pb_getf!(pb, receipt)?;
        let receipt = TransactionReceipt::from_protobuf(receipt)?;

        let duplicate_receipts = pb
            .duplicate_transaction_receipts
            .into_iter()
            .map(TransactionReceipt::from_protobuf)
            .collect::<crate::Result<_>>()?;

        let child_receipts = pb
            .child_transaction_receipts
            .into_iter()
            .map(TransactionReceipt::from_protobuf)
            .collect::<crate::Result<_>>()?;

        Ok(Self { receipt, duplicate_receipts, child_receipts })
    }
}
