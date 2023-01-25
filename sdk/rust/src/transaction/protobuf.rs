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

use hedera_proto::services;

use crate::{
    AccountId,
    TransactionId,
};

pub trait ToTransactionDataProtobuf: Send + Sync {
    fn to_transaction_data_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: &TransactionId,
    ) -> services::transaction_body::Data;
}

pub(crate) trait ToTransactionBodyProtobuf {
    type Request<'a>;
    fn to_transaction_body_protobuf(&self, request: Self::Request<'_>)
        -> services::TransactionBody;
}

pub(super) fn make_request<T: ToTransactionBodyProtobuf>(
    tx: &T,
    request: T::Request<'_>,
    signers: &[crate::transaction::AnySigner],
    operator: Option<&crate::client::Operator>,
) -> crate::Result<(services::Transaction, crate::TransactionHash)> {
    use prost::Message;

    let transaction_body = tx.to_transaction_body_protobuf(request);

    let body_bytes = transaction_body.encode_to_vec();

    let mut signatures = Vec::with_capacity(1 + signers.len());

    if let Some(operator) = operator {
        let operator_signature = operator.sign(&body_bytes);

        // todo: avoid the `.map(xyz).collect()`
        signatures.push(super::execute::SignaturePair::from(operator_signature));
    }

    for signer in signers {
        if signatures.iter().all(|it| it.public != signer.public_key()) {
            let signature = signer.sign(&body_bytes);
            signatures.push(super::execute::SignaturePair::from(signature));
        }
    }

    let signatures =
        signatures.into_iter().map(super::execute::SignaturePair::into_protobuf).collect();

    let signed_transaction = services::SignedTransaction {
        body_bytes,
        sig_map: Some(services::SignatureMap { sig_pair: signatures }),
    };

    let signed_transaction_bytes = signed_transaction.encode_to_vec();

    let transaction_hash = crate::TransactionHash::new(&signed_transaction_bytes);

    let transaction =
        services::Transaction { signed_transaction_bytes, ..services::Transaction::default() };

    Ok((transaction, transaction_hash))
}
