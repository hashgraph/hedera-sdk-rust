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
use std::collections::HashMap;

use hedera_proto::services;
use prost::Message;
use tonic::transport::Channel;

use super::TransactionSources;
use crate::execute::Execute;
use crate::protobuf::FromProtobuf;
use crate::transaction::any::AnyTransactionData;
use crate::transaction::protobuf::ToTransactionDataProtobuf;
use crate::transaction::DEFAULT_TRANSACTION_VALID_DURATION;
use crate::{
    AccountId,
    BoxGrpcFuture,
    Client,
    Error,
    Hbar,
    HbarUnit,
    LedgerId,
    PublicKey,
    ToProtobuf,
    Transaction,
    TransactionHash,
    TransactionId,
    TransactionResponse,
    ValidateChecksums,
};

#[derive(Debug)]
pub(super) struct SignaturePair {
    signature: Vec<u8>,
    public: PublicKey,
}

impl SignaturePair {
    pub fn into_protobuf(self) -> services::SignaturePair {
        let signature = match self.public.kind() {
            crate::key::KeyKind::Ed25519 => {
                services::signature_pair::Signature::Ed25519(self.signature)
            }
            crate::key::KeyKind::Ecdsa => {
                services::signature_pair::Signature::EcdsaSecp256k1(self.signature)
            }
        };
        services::SignaturePair {
            signature: Some(signature),
            // TODO: is there any way to utilize the _prefix_ nature of this field?
            pub_key_prefix: self.public.to_bytes_raw(),
        }
    }
}

impl From<(PublicKey, Vec<u8>)> for SignaturePair {
    fn from((public, signature): (PublicKey, Vec<u8>)) -> Self {
        Self { signature, public }
    }
}

impl<D> Transaction<D>
where
    D: TransactionExecute,
{
    pub(crate) fn make_request_inner(
        &self,
        transaction_id: TransactionId,
        node_account_id: AccountId,
    ) -> crate::Result<(services::Transaction, TransactionHash)> {
        assert!(self.is_frozen());

        let transaction_body = self.to_transaction_body_protobuf(node_account_id, &transaction_id);

        let body_bytes = transaction_body.encode_to_vec();

        let mut signatures = Vec::with_capacity(1 + self.signers.len());

        if let Some(operator) = &self.body.operator {
            let operator_signature = operator.sign(&body_bytes);

            // todo: avoid the `.map(xyz).collect()`
            signatures.push(SignaturePair::from(operator_signature));
        }

        for signer in &self.signers {
            if signatures.iter().all(|it| it.public != signer.public_key()) {
                let signature = signer.sign(&body_bytes);
                signatures.push(SignaturePair::from(signature));
            }
        }

        let signatures = signatures.into_iter().map(SignaturePair::into_protobuf).collect();

        let signed_transaction = services::SignedTransaction {
            body_bytes,
            sig_map: Some(services::SignatureMap { sig_pair: signatures }),
        };

        let signed_transaction_bytes = signed_transaction.encode_to_vec();

        let transaction_hash = TransactionHash::new(&signed_transaction_bytes);

        let transaction =
            services::Transaction { signed_transaction_bytes, ..services::Transaction::default() };

        Ok((transaction, transaction_hash))
    }
}

pub trait TransactionExecute:
    Clone + ToTransactionDataProtobuf + Into<AnyTransactionData> + ValidateChecksums
{
    fn default_max_transaction_fee(&self) -> Hbar {
        Hbar::from_unit(2, HbarUnit::Hbar)
    }

    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse>;
}

impl<D> Execute for Transaction<D>
where
    D: TransactionExecute,
{
    type GrpcRequest = services::Transaction;

    type GrpcResponse = services::TransactionResponse;

    type Context = TransactionHash;

    type Response = TransactionResponse;

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        self.body.node_account_ids.as_deref()
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        self.body.transaction_id
    }

    fn requires_transaction_id(&self) -> bool {
        true
    }

    fn make_request(
        &self,
        transaction_id: &Option<TransactionId>,
        node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)> {
        assert!(self.is_frozen());

        self.make_request_inner(
            transaction_id.ok_or(Error::NoPayerAccountOrTransactionId)?,
            node_account_id,
        )
    }

    fn execute(
        &self,
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> BoxGrpcFuture<'_, Self::GrpcResponse> {
        self.body.data.execute(channel, request)
    }

    fn make_response(
        &self,
        _response: Self::GrpcResponse,
        transaction_hash: Self::Context,
        node_account_id: AccountId,
        transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response> {
        Ok(TransactionResponse {
            node_account_id,
            transaction_id: transaction_id.unwrap(),
            transaction_hash,
            validate_status: true,
        })
    }

    fn make_error_pre_check(
        &self,
        status: crate::Status,
        transaction_id: Option<TransactionId>,
    ) -> crate::Error {
        if let Some(transaction_id) = transaction_id {
            crate::Error::TransactionPreCheckStatus { status, transaction_id }
        } else {
            crate::Error::TransactionNoIdPreCheckStatus { status }
        }
    }

    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32> {
        Ok(response.node_transaction_precheck_code)
    }
}

impl<D: ValidateChecksums> ValidateChecksums for Transaction<D> {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        if let Some(node_account_ids) = &self.body.node_account_ids {
            for node_account_id in node_account_ids {
                node_account_id.validate_checksums(ledger_id)?;
            }
        }
        self.body.transaction_id.validate_checksums(ledger_id)?;
        self.body.data.validate_checksums(ledger_id)
    }
}

impl<D> Transaction<D>
where
    D: TransactionExecute,
{
    #[allow(deprecated)]
    fn to_transaction_body_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: &TransactionId,
    ) -> services::TransactionBody {
        assert!(self.is_frozen());
        let data = self.body.data.to_transaction_data_protobuf(node_account_id, transaction_id);

        let max_transaction_fee = self
            .body
            .max_transaction_fee
            .unwrap_or_else(|| self.body.data.default_max_transaction_fee());

        services::TransactionBody {
            data: Some(data),
            transaction_id: Some(transaction_id.to_protobuf()),
            transaction_valid_duration: Some(
                self.body
                    .transaction_valid_duration
                    .unwrap_or(DEFAULT_TRANSACTION_VALID_DURATION)
                    .into(),
            ),
            memo: self.body.transaction_memo.clone(),
            node_account_id: Some(node_account_id.to_protobuf()),
            generate_record: false,
            transaction_fee: max_transaction_fee.to_tinybars() as u64,
        }
    }
}

// fixme: find a better name.
pub(crate) struct ExecuteTransaction<'a, D> {
    inner: &'a Transaction<D>,
    sources: Cow<'a, TransactionSources>,
    hashes: Box<[TransactionHash]>,
    node_account_ids: Box<[AccountId]>,
    indecies_by_node_id: HashMap<AccountId, usize>,
}

impl<'a, D> ExecuteTransaction<'a, D> {
    pub(crate) fn new(transaction: &'a Transaction<D>, sources: &'a TransactionSources) -> Self {
        // fixme: be way more lazy.
        let sources = sources.sign_with(&transaction.signers);

        // note: the transaction's node indexes have to match the ones in sources, but we don't know what order they're in, so, we have to do it like this
        // fixme: should `from_bytes` error if a node index is duplicated?
        let (node_account_ids, hashes) = {
            let mut node_account_ids = Vec::with_capacity(sources.0.len());
            let mut hashes = Vec::with_capacity(sources.0.len());

            for source in &*sources.0 {
                let tx =
                    services::SignedTransaction::decode(source.signed_transaction_bytes.as_slice())
                        .unwrap();

                hashes.push(TransactionHash::new(&tx.body_bytes));

                let node_account_id = services::TransactionBody::decode(tx.body_bytes.as_slice())
                    .unwrap()
                    .node_account_id
                    .unwrap();

                node_account_ids.push(AccountId::from_protobuf(node_account_id).unwrap());
            }

            (node_account_ids, hashes)
        };

        let node_account_id_indexes = node_account_ids
            .iter()
            .copied()
            .enumerate()
            .map(|(index, value)| (value, index))
            .collect();

        Self {
            inner: transaction,
            sources,
            hashes: hashes.into_boxed_slice(),
            node_account_ids: node_account_ids.into_boxed_slice(),
            indecies_by_node_id: node_account_id_indexes,
        }
    }

    pub(crate) async fn execute(
        &self,
        client: &Client,
        timeout: Option<std::time::Duration>,
    ) -> crate::Result<TransactionResponse>
    where
        D: TransactionExecute,
    {
        crate::execute::execute(client, self, timeout).await
    }
}

impl<'a, D: ValidateChecksums> ValidateChecksums for ExecuteTransaction<'a, D> {
    fn validate_checksums(&self, ledger_id: &LedgerId) -> Result<(), Error> {
        self.inner.validate_checksums(ledger_id)
    }
}

impl<'a, D: TransactionExecute> Execute for ExecuteTransaction<'a, D> {
    type GrpcRequest = <Transaction<D> as Execute>::GrpcRequest;

    type GrpcResponse = <Transaction<D> as Execute>::GrpcResponse;

    type Context = <Transaction<D> as Execute>::Context;

    type Response = <Transaction<D> as Execute>::Response;

    fn node_account_ids(&self) -> Option<&[AccountId]> {
        Some(&self.node_account_ids)
    }

    fn transaction_id(&self) -> Option<TransactionId> {
        Some(self.inner.transaction_id().unwrap())
    }

    fn requires_transaction_id(&self) -> bool {
        true
    }

    fn make_request(
        &self,
        transaction_id: &Option<TransactionId>,
        node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)> {
        debug_assert_eq!(transaction_id, &self.transaction_id());

        let index = *self.indecies_by_node_id.get(&node_account_id).unwrap();
        Ok((self.sources.0[index].clone(), self.hashes[index]))
    }

    fn execute(
        &self,
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> BoxGrpcFuture<Self::GrpcResponse> {
        self.inner.execute(channel, request)
    }

    fn make_response(
        &self,
        response: Self::GrpcResponse,
        context: Self::Context,
        node_account_id: AccountId,
        transaction_id: Option<TransactionId>,
    ) -> crate::Result<Self::Response> {
        self.inner.make_response(response, context, node_account_id, transaction_id)
    }

    fn make_error_pre_check(
        &self,
        status: crate::Status,
        transaction_id: Option<TransactionId>,
    ) -> crate::Error {
        self.inner.make_error_pre_check(status, transaction_id)
    }

    fn response_pre_check_status(response: &Self::GrpcResponse) -> crate::Result<i32> {
        Transaction::<D>::response_pre_check_status(response)
    }
}
