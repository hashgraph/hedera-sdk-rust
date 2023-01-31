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

use std::borrow::Cow;
use std::fmt;
use std::fmt::{
    Debug,
    Formatter,
};

use hedera_proto::services;
use prost::Message;
use time::Duration;

use crate::execute::{
    execute,
    Execute,
};
use crate::signer::AnySigner;
use crate::{
    AccountId,
    Client,
    Error,
    FromProtobuf,
    Hbar,
    Operator,
    PrivateKey,
    PublicKey,
    Signer,
    TransactionId,
    TransactionResponse,
    ValidateChecksums,
};

mod any;
mod execute;
mod protobuf;
#[cfg(test)]
mod tests;

pub use any::AnyTransaction;
#[cfg(feature = "ffi")]
pub(crate) use any::AnyTransactionBody;
pub(crate) use any::AnyTransactionData;
#[cfg(feature = "ffi")]
pub(crate) use execute::execute2;
pub(crate) use execute::TransactionExecute;
pub(crate) use protobuf::ToTransactionDataProtobuf;

const DEFAULT_TRANSACTION_VALID_DURATION: Duration = Duration::seconds(120);

#[derive(Clone)]
pub struct TransactionSources(pub(crate) Box<[services::Transaction]>);

impl TransactionSources {
    pub(crate) fn sign_all(txs: &mut [services::SignedTransaction], signers: &[AnySigner]) {
        // todo: don't be `O(nmk)`, we can do `O(m(n+k))` if we know all transactions already have the same signers.
        for tx in txs {
            let sig_map = tx.sig_map.get_or_insert_with(services::SignatureMap::default);

            for signer in signers {
                let pk = signer.public_key().to_bytes_raw();

                if sig_map.sig_pair.iter().any(|it| pk.starts_with(&it.pub_key_prefix)) {
                    continue;
                }

                // todo: reuse `pk_bytes` instead of re-serializing them.
                let sig_pair = execute::SignaturePair::from(signer.sign(&tx.body_bytes));

                sig_map.sig_pair.push(sig_pair.into_protobuf());
            }
        }
    }

    pub(crate) fn sign_with(&self, signers: &[AnySigner]) -> Cow<'_, Self> {
        if signers.is_empty() {
            return Cow::Borrowed(self);
        }

        // todo: avoid the double-collect.
        let mut signed_transactions: Vec<_> = self
            .0
            .iter()
            .map(|it| {
                if it.signed_transaction_bytes.is_empty() {
                    // sources can only be non-none if we were created from `from_bytes`.
                    // from_bytes ensures that all `sources` have `signed_transaction_bytes`.
                    unreachable!()
                } else {
                    // unreachable: sources can only be non-none if we were created from `from_bytes`.
                    // from_bytes checks all transaction bodies for equality, which involves desrializing all `signed_transaction_bytes`.
                    services::SignedTransaction::decode(it.signed_transaction_bytes.as_slice())
                        .unwrap_or_else(|_| unreachable!())
                }
            })
            .collect();

        Self::sign_all(&mut signed_transactions, signers);

        let transaction_list = signed_transactions
            .into_iter()
            .map(|it| services::Transaction {
                signed_transaction_bytes: it.encode_to_vec(),
                ..Default::default()
            })
            .collect();

        Cow::Owned(Self(transaction_list))
    }
}

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

    #[cfg_attr(feature = "ffi", serde(skip))]
    sources: Option<TransactionSources>,
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

    pub(crate) transaction_id: Option<TransactionId>,

    pub(crate) operator: Option<Operator>,

    #[cfg_attr(feature = "ffi", serde(skip_serializing_if = "std::ops::Not::not"))]
    pub(crate) is_frozen: bool,
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
                max_transaction_fee: None,
                transaction_memo: String::new(),
                transaction_id: None,
                operator: None,
                is_frozen: false,
            },
            signers: Vec::new(),
            sources: None,
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
    ///.
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
        Self { body, signers, sources: None }
    }

    pub(crate) fn is_frozen(&self) -> bool {
        self.body.is_frozen
    }

    #[cfg(feature = "ffi")]
    pub(crate) fn sources(&self) -> Option<&TransactionSources> {
        self.sources.as_ref()
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

    /// Freeze the transaction so that no further modifications can be made.
    pub fn freeze(&mut self) -> crate::Result<&mut Self> {
        self.freeze_with(None)
    }

    /// Freeze the transaction so that no further modifications can be made.
    pub fn freeze_with<'a>(
        &mut self,
        client: impl Into<Option<&'a Client>>,
    ) -> crate::Result<&mut Self> {
        if self.is_frozen() {
            return Ok(self);
        }
        let client: Option<&Client> = client.into();

        let node_account_ids = match &self.body.node_account_ids {
            // the clone here is the lesser of two evils.
            Some(it) => it.clone(),
            None => {
                client.ok_or_else(|| crate::Error::FreezeUnsetNodeAccountIds)?.random_node_ids()
            }
        };

        // note to reviewer: this is intentionally still an option, fallback is used later, swift doesn't *have* default max transaction fee and fixing it is a massive PITA.
        let max_transaction_fee = self.body.max_transaction_fee.or_else(|| {
            // no max has been set on the *transaction*
            // check if there is a global max set on the client
            let client_max_transaction_fee = client
                .map(|it| it.max_transaction_fee().load(std::sync::atomic::Ordering::Relaxed));

            match client_max_transaction_fee {
                Some(max) if max > 1 => Some(Hbar::from_tinybars(max as i64)),
                // no max has been set on the client either
                // fallback to the hard-coded default for this transaction type
                _ => None,
            }
        });

        let operator = client.and_then(|it| it.operator_internal().as_deref().map(|it| it.clone()));

        // note: yes, there's an `Some(opt.unwrap())`, this is INTENTIONAL.
        self.body.node_account_ids = Some(node_account_ids);
        self.body.max_transaction_fee = max_transaction_fee;
        self.body.operator = operator;
        self.body.is_frozen = true;

        if let Some(client) = client {
            if client.auto_validate_checksums() {
                if let Some(ledger_id) = &*client.ledger_id_internal() {
                    self.validate_checksums(ledger_id)?;
                } else {
                    return Err(crate::Error::CannotPerformTaskWithoutLedgerId {
                        task: "validate checksums",
                    });
                }
            }
        }

        Ok(self)
    }

    /// Convert `self` to protobuf encoded bytes.
    ///
    /// # Errors
    /// - If `freeze_with` wasn't called with an operator.
    ///
    /// # Panics
    /// - If `!self.is_frozen()`.
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        if !self.is_frozen() {
            panic!("Transaction must be frozen to call `to_bytes`");
        }

        if let Some(sources) = &self.sources {
            let transaction_list = sources.sign_with(&self.signers).0.to_vec();

            return Ok(hedera_proto::sdk::TransactionList { transaction_list }.encode_to_vec());
        }

        let transaction_id = match self.transaction_id() {
            Some(id) => id,
            None => self
                .body
                .operator
                .as_ref()
                .ok_or(crate::Error::NoPayerAccountOrTransactionId)?
                .generate_transaction_id(),
        };

        let transaction_list: Result<_, _> = self
            .body
            .node_account_ids
            .as_deref()
            .unwrap()
            .iter()
            .copied()
            .map(|node_account_id| {
                self.make_request_inner(transaction_id, node_account_id).map(|it| it.0)
            })
            .collect();

        let transaction_list = transaction_list?;

        Ok(hedera_proto::sdk::TransactionList { transaction_list }.encode_to_vec())
    }
}

impl<D> Transaction<D>
where
    D: TransactionExecute,
{
    /// Execute this transaction against the provided client of the Hedera network.
    // todo:
    pub async fn execute(&mut self, client: &Client) -> crate::Result<TransactionResponse> {
        self.execute_with_optional_timeout(client, None).await
    }

    pub(crate) async fn execute_with_optional_timeout(
        &mut self,
        client: &Client,
        timeout: Option<std::time::Duration>,
    ) -> crate::Result<TransactionResponse> {
        // it's fine to call freeze while already frozen, so, let `freeze_with` handle the freeze check.
        self.freeze_with(Some(client))?;

        if let Some(sources) = &self.sources {
            return self::execute::execute2(client, self, sources, timeout).await;
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
        self.execute_with_optional_timeout(client, Some(timeout)).await
    }
}

// these impls are on `AnyTransaction`, but they're here instead of in `any` because actually implementing them is only possible here.
impl AnyTransaction {
    /// # Examples
    /// ```
    /// # fn main() -> hedera::Result<()> {
    /// use hedera::AnyTransaction;
    /// let bytes = hex::decode(concat!(
    ///     "0a522a500a4c0a120a0c0885c8879e0610a8bdd9840312021865120218061880",
    ///     "94ebdc0322020877320c686920686173686772617068721a0a180a0a0a021802",
    ///     "108088debe010a0a0a02186510ff87debe0112000a522a500a4c0a120a0c0885",
    ///     "c8879e0610a8bdd984031202186512021807188094ebdc0322020877320c6869",
    ///     "20686173686772617068721a0a180a0a0a021802108088debe010a0a0a021865",
    ///     "10ff87debe011200"
    /// )).unwrap();
    /// let tx = AnyTransaction::from_bytes(&bytes)?;
    /// # let _ = tx;
    /// # Ok(())
    /// # }
    /// ```
    #[allow(deprecated)]
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        let list =
            hedera_proto::sdk::TransactionList::decode(bytes).map_err(Error::from_protobuf)?;

        let list = if list.transaction_list.is_empty() {
            Vec::from([services::Transaction::decode(bytes).map_err(Error::from_protobuf)?])
        } else {
            list.transaction_list
        };

        let tmp: Result<Vec<_>, _> = list.iter().map(transaction_body_from_transaction).collect();
        let tmp = tmp?;

        let node_ids: Result<std::collections::HashSet<_>, _> = tmp
            .iter()
            .map(|it| {
                let node_account_id = it.node_account_id.clone().ok_or_else(|| {
                    crate::Error::from_protobuf(concat!("unexpected missing `node_account_id`"))
                })?;

                AccountId::from_protobuf(node_account_id)
            })
            .collect();

        let (first, tmp) =
            tmp.split_first().ok_or_else(|| Error::from_protobuf("no transactions found"))?;

        for it in tmp.iter() {
            if &first.transaction_id != &it.transaction_id {
                return Err(Error::from_protobuf("chunked transactions not currently supported"));
            }

            if !pb_transaction_body_eq(first, it) {
                return Err(Error::from_protobuf("transaction parts unexpectedly unequal"));
            }
        }

        let node_ids: Vec<_> = node_ids?.into_iter().collect();

        // note: this creates the transaction in a frozen state.
        let mut res = Self::from_protobuf(first.clone())?;

        // note: this doesn't check freeze for obvious reasons.
        res.body.node_account_ids = Some(node_ids);
        res.sources = Some(TransactionSources(list.into_boxed_slice()));

        Ok(res)
    }
}

/// Returns `true` if lhs == rhs other than `transaction_id` and `node_account_id`, `false` otherwise.
#[allow(deprecated)]
fn pb_transaction_body_eq(
    lhs: &services::TransactionBody,
    rhs: &services::TransactionBody,
) -> bool {
    // destructure one side to ensure we don't miss any fields.
    let services::TransactionBody {
        transaction_id: _,
        node_account_id: _,
        transaction_fee,
        transaction_valid_duration,
        generate_record,
        memo,
        data,
    } = rhs;

    if &lhs.transaction_fee != transaction_fee {
        return false;
    }

    if &lhs.transaction_valid_duration != transaction_valid_duration {
        return false;
    }

    if &lhs.generate_record != generate_record {
        return false;
    }

    if &lhs.memo != memo {
        return false;
    }

    if &lhs.data != data {
        return false;
    }

    true
}

impl FromProtobuf<services::Transaction> for AnyTransaction {
    fn from_protobuf(pb: services::Transaction) -> crate::Result<Self>
    where
        Self: Sized,
    {
        transaction_body_from_transaction(&pb).and_then(Self::from_protobuf)
    }
}

#[allow(deprecated)]
fn transaction_body_from_transaction(
    tx: &services::Transaction,
) -> crate::Result<services::TransactionBody> {
    if !tx.signed_transaction_bytes.is_empty() {
        let tx = services::SignedTransaction::decode(&*tx.signed_transaction_bytes)
            .map_err(Error::from_protobuf)?;

        return services::TransactionBody::decode(&*tx.body_bytes).map_err(Error::from_protobuf);
    }

    Err(Error::from_protobuf("Transaction had no signed transaction bytes"))
}

impl FromProtobuf<services::TransactionBody> for AnyTransaction {
    fn from_protobuf(pb: services::TransactionBody) -> crate::Result<Self>
    where
        Self: Sized,
    {
        Ok(Transaction {
            body: TransactionBody {
                data: AnyTransactionData::from_protobuf(pb_getf!(pb, data)?)?,
                node_account_ids: None,
                transaction_valid_duration: pb.transaction_valid_duration.map(Into::into),
                max_transaction_fee: Some(Hbar::from_tinybars(pb.transaction_fee as i64)),
                transaction_memo: pb.memo,
                transaction_id: Some(TransactionId::from_protobuf(pb_getf!(pb, transaction_id)?)?),
                operator: None,
                is_frozen: true,
            },
            signers: Vec::new(),
            sources: None,
        })
    }
}
