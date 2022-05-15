use std::error::Error as StdError;
use std::result::Result as StdResult;

use hedera_proto::services::ResponseCodeEnum;

use crate::{AccountId, TransactionId};

pub type Result<T> = StdResult<T, Error>;

pub(crate) type BoxStdError = Box<dyn StdError + Send + Sync + 'static>;

/// Represents any possible error from a fallible function in the Hedera SDK.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to complete request within the maximum time allowed; most recent attempt failed with: {0}")]
    TimedOut(Box<Error>),

    #[error("grpc: {0}")]
    GrpcStatus(#[from] tonic::Status),

    #[error("failed to create a SDK type from a protobuf response: {0}")]
    FromProtobuf(BoxStdError),

    /// Signals that a query or transaction has failed the pre-check.
    // FIXME: Use hedera::Status (once available)
    // TODO: Add transaction_id: Option<TransactionId>
    #[error("transaction `{}` failed pre-check with status `{status:?}`", .transaction_id.as_ref().map(|id| id.to_string()).as_deref().unwrap_or("_"))]
    PreCheckStatus { status: ResponseCodeEnum, transaction_id: Option<TransactionId> },

    /// Failed to parse a basic type from string (ex. AccountId, ContractId, TransactionId, etc.).
    #[error("failed to parse: {0}")]
    BasicParse(BoxStdError),

    #[error("failed to parse a key: {0}")]
    KeyParse(BoxStdError),

    #[error("client must be configured with a payer account or requests must be given an explicit transaction id")]
    NoPayerAccountOrTransactionId,

    #[error("exceeded maximum attempts for request; most recent attempt failed with: {0}")]
    MaxAttemptsExceeded(Box<Error>),

    #[error("cost of {query_cost} without explicit payment is greater than the maximum allowed payment of {max_query_payment}")]
    MaxQueryPaymentExceeded { query_cost: u64, max_query_payment: u64 },

    #[error("node account `{0}` was not found in the configured network")]
    NodeAccountUnknown(AccountId),

    #[error("received unrecognized status code: {0}, try updating your SDK")]
    ResponseStatusUnrecognized(i32),

    #[error("failed to sign request: {0}")]
    Signature(BoxStdError),

    #[error("failed to parse a request from JSON: {0}")]
    RequestParse(BoxStdError),
}

impl Error {
    pub(crate) fn from_protobuf<E: Into<BoxStdError>>(error: E) -> Self {
        Self::FromProtobuf(error.into())
    }

    pub(crate) fn key_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::KeyParse(error.into())
    }

    pub(crate) fn basic_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::BasicParse(error.into())
    }

    pub(crate) fn request_parse<E: Into<BoxStdError>>(error: E) -> Self {
        Self::RequestParse(error.into())
    }

    pub(crate) fn signature<E: Into<BoxStdError>>(error: E) -> Self {
        Self::Signature(error.into())
    }

    pub(crate) fn pre_check(
        status: ResponseCodeEnum,
        transaction_id: Option<TransactionId>,
    ) -> Self {
        Self::PreCheckStatus { status, transaction_id }
    }
}
