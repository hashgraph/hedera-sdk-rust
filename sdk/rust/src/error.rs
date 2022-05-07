use std::error::Error as StdError;
use std::result::Result as StdResult;

use hedera_proto::services::ResponseCodeEnum;

pub type Result<T> = StdResult<T, Error>;

pub(crate) type BoxStdError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to complete operation within the maximum time allowed; most recent attempt failed with: {0}")]
    Timeout(BoxStdError),

    #[error("grpc: {0}")]
    GrpcStatus(#[from] tonic::Status),

    #[error("failed to parse a protobuf response: {0}")]
    FromProtobuf(BoxStdError),

    /// Signals that a query or transaction has failed the pre-check.
    // FIXME: Use hedera::Status (once available)
    // TODO: Add transaction_id: Option<TransactionId>
    #[error("query failed pre-check with status `{status:?}`")]
    QueryPreCheckStatus { status: ResponseCodeEnum },

    /// Signals that a query or transaction has failed the pre-check.
    // FIXME: Use hedera::Status (once available)
    // TODO: Add transaction_id: Option<TransactionId>
    #[error("transaction `_` failed pre-check with status `{status:?}`")]
    TransactionPreCheckStatus { status: ResponseCodeEnum },
}

impl Error {
    pub(crate) fn from_protobuf<E: Into<BoxStdError>>(error: E) -> Self {
        Self::FromProtobuf(error.into())
    }

    pub(crate) fn query_pre_check(status: ResponseCodeEnum) -> Self {
        Self::QueryPreCheckStatus { status }
    }
}
