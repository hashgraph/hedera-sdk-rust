use async_trait::async_trait;

use crate::error::BoxStdError;
use crate::{
    PrivateKey,
    SignaturePair,
};

#[async_trait]
pub trait Signer: 'static + Send + Sync {
    async fn sign(&self, message: &[u8]) -> Result<SignaturePair, BoxStdError>;
}

#[async_trait]
impl Signer for PrivateKey {
    async fn sign(&self, message: &[u8]) -> Result<SignaturePair, BoxStdError> {
        Ok(self.sign(message))
    }
}

// TODO: EnvironmentSigner
// TODO: GoogleCloudSecretSigner

#[async_trait]
impl Signer for Box<dyn Signer> {
    async fn sign(&self, message: &[u8]) -> Result<SignaturePair, BoxStdError> {
        self.sign(message).await
    }
}
