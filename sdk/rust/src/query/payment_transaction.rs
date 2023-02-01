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
use hedera_proto::services::crypto_service_client::CryptoServiceClient;
use tonic::transport::Channel;

use crate::transaction::{
    AnyTransactionData,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    AccountId,
    BoxGrpcFuture,
    Error,
    Hbar,
    LedgerId,
    ToProtobuf,
    Transaction,
    TransactionId,
    ValidateChecksums,
};

pub type PaymentTransaction = Transaction<PaymentTransactionData>;

#[cfg_attr(feature = "ffi", serde_with::skip_serializing_none)]
#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "ffi", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "ffi", serde(rename_all = "camelCase", default))]
pub struct PaymentTransactionData {
    amount: Option<Hbar>,
    max_amount: Option<Hbar>,
}

impl PaymentTransaction {
    pub(super) fn get_amount(&self) -> Option<Hbar> {
        self.data().amount
    }

    pub(super) fn amount(&mut self, amount: Hbar) -> &mut Self {
        self.data_mut().amount = Some(amount);
        self
    }

    pub(super) fn get_max_amount(&self) -> Option<Hbar> {
        self.data().max_amount
    }

    pub(super) fn max_amount(&mut self, amount: impl Into<Option<Hbar>>) -> &mut Self {
        self.data_mut().max_amount = amount.into();
        self
    }
}

impl TransactionData for PaymentTransactionData {}

impl TransactionExecute for PaymentTransactionData {
    // noinspection DuplicatedCode
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { CryptoServiceClient::new(channel).crypto_transfer(request).await })
    }
}

impl ValidateChecksums for PaymentTransactionData {
    fn validate_checksums(&self, _ledger_id: &LedgerId) -> Result<(), Error> {
        Ok(())
    }
}

impl ToTransactionDataProtobuf for PaymentTransactionData {
    #[allow(clippy::cast_possible_wrap)]
    fn to_transaction_data_protobuf(
        &self,
        node_account_id: AccountId,
        transaction_id: &TransactionId,
    ) -> services::transaction_body::Data {
        let amount = self.amount.unwrap_or_default();

        services::transaction_body::Data::CryptoTransfer(services::CryptoTransferTransactionBody {
            token_transfers: Vec::new(),
            transfers: Some(services::TransferList {
                account_amounts: vec![
                    services::AccountAmount {
                        account_id: Some(node_account_id.to_protobuf()),
                        amount: amount.to_tinybars(),
                        is_approval: false,
                    },
                    services::AccountAmount {
                        account_id: Some(transaction_id.account_id.to_protobuf()),
                        amount: -(amount.to_tinybars()),
                        is_approval: false,
                    },
                ],
            }),
        })
    }
}

impl From<PaymentTransactionData> for AnyTransactionData {
    fn from(_transaction: PaymentTransactionData) -> Self {
        // NOTE: this should only be reached if we try to serialize a PaymentTransaction
        //  as this is a private type that we have no intention of serializing, we should be good
        unreachable!()
    }
}
