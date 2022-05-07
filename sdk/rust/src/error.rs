use std::error::Error as StdError;
use std::result::Result as StdResult;

pub type Result<T> = StdResult<T, Error>;

pub(crate) type BoxStdError = Box<dyn StdError + Send + Sync + 'static>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("grpc: {0}")]
    GrpcStatus(#[from] tonic::Status),

    #[error("failed to parse a protobuf response: {0}")]
    FromProtobuf(BoxStdError),
}

impl Error {
    pub(crate) fn from_protobuf<E: Into<BoxStdError>>(error: E) -> Self {
        Self::FromProtobuf(error.into())
    }
}
