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
use hedera_proto::services::token_service_client::TokenServiceClient;
use tonic::transport::Channel;

use crate::ledger_id::RefLedgerId;
use crate::protobuf::FromProtobuf;
use crate::transaction::{
    AnyTransactionData, ChunkInfo, ToSchedulableTransactionDataProtobuf, ToTransactionDataProtobuf,
    TransactionData, TransactionExecute,
};
use crate::{AccountId, BoxGrpcFuture, Error, ToProtobuf, TokenId, Transaction, ValidateChecksums};

/// Associates the provided account with the provided tokens. Must be signed by the provided Account's key.
///
/// - If the provided account is not found, the transaction will resolve to `INVALID_ACCOUNT_ID`.
/// - If the provided account has been deleted, the transaction will resolve to `ACCOUNT_DELETED`.
/// - If any of the provided tokens are not found, the transaction will resolve to `INVALID_TOKEN_REF`.
/// - If any of the provided tokens have been deleted, the transaction will resolve to
/// `TOKEN_WAS_DELETED`.
/// - If an association between the provided account and any of the tokens already exists, the
/// transaction will resolve to `TOKEN_ALREADY_ASSOCIATED_TO_ACCOUNT`.
/// - If the provided account's associations count exceed the constraint of maximum token associations
/// per account, the transaction will resolve to `TOKENS_PER_ACCOUNT_LIMIT_EXCEEDED`.
/// - On success, associations between the provided account and tokens are made and the account is
/// ready to interact with the tokens.
pub type TokenAssociateTransaction = Transaction<TokenAssociateTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TokenAssociateTransactionData {
    /// The account to be associated with the provided tokens.
    account_id: Option<AccountId>,

    /// The tokens to be associated with the provided account.
    token_ids: Vec<TokenId>,
}

impl TokenAssociateTransaction {
    /// Returns the account to be associated with the provided tokens.
    #[must_use]
    pub fn get_account_id(&self) -> Option<AccountId> {
        self.data().account_id
    }

    /// Sets the account to be associated with the provided tokens.
    pub fn account_id(&mut self, account_id: AccountId) -> &mut Self {
        self.data_mut().account_id = Some(account_id);
        self
    }

    /// Returns the tokens to be associated with the provided account.
    #[must_use]
    pub fn get_token_ids(&self) -> &[TokenId] {
        &self.data().token_ids
    }

    /// Sets the tokens to be associated with the provided account.
    pub fn token_ids(&mut self, token_ids: impl IntoIterator<Item = TokenId>) -> &mut Self {
        self.data_mut().token_ids = token_ids.into_iter().collect();
        self
    }
}

impl TransactionData for TokenAssociateTransactionData {}

impl TransactionExecute for TokenAssociateTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { TokenServiceClient::new(channel).associate_tokens(request).await })
    }
}

impl ValidateChecksums for TokenAssociateTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> Result<(), Error> {
        self.account_id.validate_checksums(ledger_id)?;
        for token_id in &self.token_ids {
            token_id.validate_checksums(ledger_id)?;
        }
        Ok(())
    }
}

impl ToTransactionDataProtobuf for TokenAssociateTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::TokenAssociate(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TokenAssociateTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::TokenAssociate(self.to_protobuf())
    }
}

impl From<TokenAssociateTransactionData> for AnyTransactionData {
    fn from(transaction: TokenAssociateTransactionData) -> Self {
        Self::TokenAssociate(transaction)
    }
}

impl FromProtobuf<services::TokenAssociateTransactionBody> for TokenAssociateTransactionData {
    fn from_protobuf(pb: services::TokenAssociateTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            account_id: Option::from_protobuf(pb.account)?,
            token_ids: Vec::from_protobuf(pb.tokens)?,
        })
    }
}

impl ToProtobuf for TokenAssociateTransactionData {
    type Protobuf = services::TokenAssociateTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        let account = self.account_id.to_protobuf();
        let tokens = self.token_ids.to_protobuf();

        services::TokenAssociateTransactionBody { account, tokens }
    }
}

#[cfg(test)]
mod tests {
    use crate::transaction::test_helpers::{
        test_account_id, test_token_id, transaction_body, unused_private_key, VALID_START,
    };
    use crate::{AnyTransaction, Hbar, TokenAssociateTransaction, TransactionId};
    use expect_test::expect_file;

    fn make_transaction() -> TokenAssociateTransaction {
        let mut tx = TokenAssociateTransaction::new();

        tx.node_account_ids(["0.0.5005".parse().unwrap(), "0.0.5006".parse().unwrap()])
            .transaction_id(TransactionId {
                account_id: "5006".parse().unwrap(),
                valid_start: VALID_START,
                nonce: None,
                scheduled: false,
            })
            .account_id(test_account_id())
            .token_ids(vec![test_token_id()])
            .max_transaction_fee(Hbar::new(1))
            .freeze()
            .unwrap()
            .sign(unused_private_key());

        tx
    }

    #[test]
    fn serialize() {
        let tx = make_transaction();

        expect_file!["./snapshots/token_associate_transaction/serialize.txt"].assert_debug_eq(&tx);
    }

    #[test]
    fn to_from_bytes() {
        let tx = make_transaction();

        let tx2 = AnyTransaction::from_bytes(&tx.to_bytes().unwrap()).unwrap();

        let tx = transaction_body(tx);
        let tx2 = transaction_body(tx2);

        assert_eq!(tx, tx2)
    }

    #[test]
    fn get_set_token_id() {
        let token_ids = vec![test_token_id()];
        let mut tx = TokenAssociateTransaction::new();

        let tx2 = tx.token_ids(token_ids.to_owned());

        assert_eq!(tx2.get_token_ids(), &token_ids[..]);
    }

    #[test]
    fn get_set_account_id() {
        let account_id = test_account_id();

        let mut tx = TokenAssociateTransaction::new();

        let tx2 = tx.account_id(account_id).to_owned();

        assert_eq!(tx2.get_account_id(), Some(account_id));
    }
}
