/*
 * ‌
 * Hedera Rust SDK
 * ​
 * Copyright (C) 2022 - 2023 Hedera Hashgraph, LLC
 * ​
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *      http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ‍
 */

use async_trait::async_trait;

use crate::error::BoxStdError;
use crate::{
    PrivateKey,
    SignaturePair,
};

// todo(sr): not happy with this comment.
/// Represents the capability to sign a message.
#[async_trait]
pub trait Signer: 'static + Send + Sync {
    /// Attempt to sign the `message`.
    ///
    /// When signing is succesful, returns a `SignaturePair`.
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
        self.as_ref().sign(message).await
    }
}
