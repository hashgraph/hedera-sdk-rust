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

use std::sync::atomic::{
    AtomicU64,
    Ordering,
};

use async_trait::async_trait;
use hedera_proto::services;
use prost::Message;
use tonic::transport::Channel;
use tonic::{
    Response,
    Status,
};

use crate::execute::Execute;
use crate::transaction::any::AnyTransactionData;
use crate::transaction::protobuf::ToTransactionDataProtobuf;
use crate::transaction::DEFAULT_TRANSACTION_VALID_DURATION;
use crate::{
    AccountId,
    Client,
    Error,
    ToProtobuf,
    Transaction,
    TransactionHash,
    TransactionId,
    TransactionResponse,
};

#[async_trait]
pub trait TransactionExecute: Clone + ToTransactionDataProtobuf + Into<AnyTransactionData> {
    fn default_max_transaction_fee(&self) -> u64 {
        2 * 100_000_000 // 2 hbar
    }

    async fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> Result<tonic::Response<services::TransactionResponse>, tonic::Status>;
}

#[async_trait]
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

    async fn make_request(
        &self,
        client: &Client,
        transaction_id: &Option<TransactionId>,
        node_account_id: AccountId,
    ) -> crate::Result<(Self::GrpcRequest, Self::Context)> {
        let transaction_id = transaction_id.as_ref().ok_or(Error::NoPayerAccountOrTransactionId)?;

        let transaction_body = self.to_transaction_body_protobuf(
            node_account_id,
            transaction_id,
            client.max_transaction_fee(),
        );

        let body_bytes = transaction_body.encode_to_vec();

        let mut signatures = Vec::with_capacity(1);

        let operator_signature =
            client.sign_with_operator(&body_bytes).await.map_err(Error::signature)?;

        signatures.push(operator_signature.to_protobuf());

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

    async fn execute(
        &self,
        channel: Channel,
        request: Self::GrpcRequest,
    ) -> Result<Response<Self::GrpcResponse>, Status> {
        self.body.data.execute(channel, request).await
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

impl<D> Transaction<D>
where
    D: TransactionExecute,
{
    #[allow(deprecated)]
    fn to_transaction_body_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: &TransactionId,
        client_max_transaction_fee: &AtomicU64,
    ) -> services::TransactionBody {
        let data = self.body.data.to_transaction_data_protobuf(node_account_id, transaction_id);

        let max_transaction_fee = self.body.max_transaction_fee.unwrap_or_else(|| {
            // no max has been set on the *transaction*
            // check if there is a global max set on the client
            match client_max_transaction_fee.load(Ordering::Relaxed) {
                max if max > 1 => max,

                // no max has been set on the client either
                // fallback to the hard-coded default for this transaction type
                _ => self.body.data.default_max_transaction_fee(),
            }
        });

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
            transaction_fee: max_transaction_fee,
        }
    }
}
