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

use std::fmt;
use std::fmt::{
    Debug,
    Formatter,
};

use time::Duration;

use crate::execute::execute;
use crate::signer::AnySigner;
use crate::{
    AccountId,
    Client,
    Hbar,
    PrivateKey,
    PublicKey,
    Signer,
    TransactionId,
    TransactionResponse,
};

mod any;
mod execute;
mod protobuf;

#[cfg(feature = "ffi")]
pub use any::AnyTransaction;
#[cfg(feature = "ffi")]
pub(crate) use any::AnyTransactionBody;
pub(crate) use any::AnyTransactionData;
pub(crate) use execute::TransactionExecute;
pub(crate) use protobuf::ToTransactionDataProtobuf;

const DEFAULT_TRANSACTION_VALID_DURATION: Duration = Duration::seconds(120);

/// A transaction that can be executed on the Hedera network.
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
pub struct Transaction<D>
where
    D: TransactionExecute,
{
    #[cfg_attr(feature = "ffi", serde(flatten))]
    body: TransactionBody<D>,

    #[cfg_attr(feature = "ffi", serde(skip))]
    signers: Vec<AnySigner>,

    #[cfg_attr(feature = "ffi", serde(skip_serializing_if = "std::ops::Not::not"))]
    is_frozen: bool,
}

#[derive(Debug, Default, Clone)]
#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase"))]
// fires because of `serde_as`
#[allow(clippy::type_repetition_in_bounds)]
pub(crate) struct TransactionBody<D>
where
    D: TransactionExecute,
{
    #[cfg_attr(feature = "ffi", serde(flatten))]
    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<serde_with::FromInto<AnyTransactionData>>")
    )]
    pub(crate) data: D,

    pub(crate) node_account_ids: Option<Vec<AccountId>>,

    #[cfg_attr(
        feature = "ffi",
        serde(with = "serde_with::As::<Option<serde_with::DurationSeconds<i64>>>")
    )]
    pub(crate) transaction_valid_duration: Option<Duration>,

    pub(crate) max_transaction_fee: Option<Hbar>,

    #[cfg_attr(feature = "ffi", serde(skip_serializing_if = "String::is_empty"))]
    pub(crate) transaction_memo: String,

    pub(crate) payer_account_id: Option<AccountId>,

    pub(crate) transaction_id: Option<TransactionId>,
}

impl<D> Default for Transaction<D>
where
    D: Default + TransactionExecute,
{
    fn default() -> Self {
        Self {
            body: TransactionBody {
                data: D::default(),
                node_account_ids: None,
                transaction_valid_duration: None,
                transaction_memo: String::new(),
                max_transaction_fee: None,
                payer_account_id: None,
                transaction_id: None,
            },
            is_frozen: false,
            signers: Vec::new(),
        }
    }
}

impl<D> Debug for Transaction<D>
where
    D: Debug + TransactionExecute,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Transaction").field("body", &self.body).finish()
    }
}

impl<D> Transaction<D>
where
    D: Default + TransactionExecute,
{
    /// Create a new default transaction.
    ///
    /// Does the same thing as [`default`](Self::default)
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<D> Transaction<D>
where
    D: TransactionExecute,
{
    #[cfg(feature = "ffi")]
    pub(crate) fn from_parts(body: TransactionBody<D>, signers: Vec<AnySigner>) -> Self {
        Self { body, signers, is_frozen: true }
    }

    pub(crate) fn is_frozen(&self) -> bool {
        self.is_frozen
    }

    /// # Panics
    /// If `self.is_frozen().
    #[track_caller]
    pub(crate) fn require_not_frozen(&self) {
        assert!(
            !self.is_frozen(),
            "transaction is immutable; it has at least one signature or has been explicitly frozen"
        );
    }

    #[cfg(feature = "ffi")]
    pub(crate) fn body(&self) -> &TransactionBody<D> {
        &self.body
    }

    /// # Panics
    /// If `self.is_frozen()`.
    fn body_mut(&mut self) -> &mut TransactionBody<D> {
        self.require_not_frozen();
        &mut self.body
    }

    pub(crate) fn into_body(self) -> TransactionBody<D> {
        self.body
    }

    pub(crate) fn data(&self) -> &D {
        &self.body.data
    }

    /// # Panics
    /// If `self.is_frozen()`.
    pub(crate) fn data_mut(&mut self) -> &mut D {
        self.require_not_frozen();
        &mut self.body.data
    }

    /// Returns the account IDs of the nodes that this transaction may be submitted to.
    ///
    /// `None` means any node configured on the client.
    #[must_use]
    pub fn get_node_account_ids(&self) -> Option<&[AccountId]> {
        self.body.node_account_ids.as_deref()
    }

    /// Sets the account IDs of the nodes that this transaction may be submitted to.
    ///
    /// Defaults to the full list of nodes configured on the client.
    #[track_caller]
    pub fn node_account_ids(&mut self, ids: impl IntoIterator<Item = AccountId>) -> &mut Self {
        self.body_mut().node_account_ids = Some(ids.into_iter().collect());
        self
    }

    /// Returns the duration that this transaction is valid for, once finalized and signed.
    #[must_use]
    pub fn get_transaction_valid_duration(&self) -> Option<Duration> {
        self.body.transaction_valid_duration
    }

    /// Sets the duration that this transaction is valid for, once finalized and signed.
    ///
    /// Defaults to 120 seconds (or two minutes).
    pub fn transaction_valid_duration(&mut self, duration: Duration) -> &mut Self {
        self.body_mut().transaction_valid_duration = Some(duration);
        self
    }

    /// Returns the maximum transaction fee the paying account is willing to pay.
    #[must_use]
    pub fn get_max_transaction_fee(&self) -> Option<Hbar> {
        self.body.max_transaction_fee
    }

    /// Sets the maximum transaction fee the paying account is willing to pay.
    pub fn max_transaction_fee(&mut self, fee: Hbar) -> &mut Self {
        self.body_mut().max_transaction_fee = Some(fee);
        self
    }

    /// Sets a note / description that should be recorded in the transaction record.
    ///
    /// Maximum length of 100 characters.
    #[must_use]
    pub fn get_transaction_memo(&self) -> &str {
        &self.body.transaction_memo
    }

    /// Sets a note or description that should be recorded in the transaction record.
    ///
    /// Maximum length of 100 characters.
    pub fn transaction_memo(&mut self, memo: impl AsRef<str>) -> &mut Self {
        self.body_mut().transaction_memo = memo.as_ref().to_owned();
        self
    }

    /// Returns the explicit transaction ID to use to identify this transaction.
    ///
    /// Overrides the payer account defined on this transaction or on the client.
    #[must_use]
    pub fn get_transaction_id(&self) -> Option<TransactionId> {
        self.body.transaction_id
    }

    /// Sets an explicit transaction ID to use to identify this transaction.
    ///
    /// Overrides the payer account defined on this transaction or on the client.
    pub fn transaction_id(&mut self, id: TransactionId) -> &mut Self {
        self.body_mut().transaction_id = Some(id);
        self
    }

    /// Sign the transaction.
    pub fn sign(&mut self, private_key: PrivateKey) -> &mut Self {
        self.sign_signer(AnySigner::PrivateKey(private_key))
    }

    /// Sign the transaction.
    pub fn sign_with(&mut self, public_key: PublicKey, signer: Signer) -> &mut Self {
        self.sign_signer(AnySigner::Arbitrary(Box::new(public_key), signer))
    }

    pub(crate) fn sign_signer(&mut self, signer: AnySigner) -> &mut Self {
        // We're _supposed_ to require frozen here, but really there's no reason I can think of to do that.

        // skip the signer if we already have it.
        if self.signers.iter().any(|it| it.public_key() == signer.public_key()) {
            return self;
        }

        self.signers.push(signer);
        self
    }

    pub(crate) fn _freeze(&mut self) -> &mut Self {
        self.freeze_with(None)
    }

    pub(crate) fn freeze_with(&mut self, _client: Option<&Client>) -> &mut Self {
        // todo: do more here.
        self.is_frozen = true;

        self
    }
}

impl<D> Transaction<D>
where
    D: TransactionExecute,
{
    /// Execute this transaction against the provided client of the Hedera network.
    // todo:
    #[allow(clippy::missing_errors_doc)]
    pub async fn execute(&mut self, client: &Client) -> crate::Result<TransactionResponse> {
        if !self.is_frozen() {
            self.freeze_with(Some(client));
        }

        execute(client, self, None).await
    }

    #[cfg(feature = "ffi")]
    pub(crate) async fn execute_with_optional_timeout(
        &mut self,
        client: &Client,
        timeout: Option<std::time::Duration>,
    ) -> crate::Result<TransactionResponse> {
        if !self.is_frozen() {
            self.freeze_with(Some(client));
        }

        execute(client, self, timeout).await
    }

    /// Execute this transaction against the provided client of the Hedera network.
    // todo:
    #[allow(clippy::missing_errors_doc)]
    pub async fn execute_with_timeout(
        &mut self,
        client: &Client,
        // fixme: be consistent with `time::Duration`? Except `tokio::time` is `std::time`, and we depend on tokio.
        timeout: std::time::Duration,
    ) -> crate::Result<TransactionResponse> {
        if !self.is_frozen() {
            self.freeze_with(Some(client));
        }

        execute(client, self, timeout).await
    }
}
