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

use crate::protobuf::{
    FromProtobuf,
    ToProtobuf,
};
use crate::transaction::{
    AnyTransactionData,
    ChunkInfo,
    ToSchedulableTransactionDataProtobuf,
    ToTransactionDataProtobuf,
    TransactionData,
    TransactionExecute,
};
use crate::{
    BoxGrpcFuture,
    Error,
    TokenId,
    Transaction,
    ValidateChecksums,
};

/// Burns tokens from the Token's treasury Account.
///
/// The operation decreases the Total Supply of the Token. Total supply cannot go below zero.
///
/// The amount provided must be in the lowest denomination possible. Example:
/// Token A has 2 decimals. In order to burn 100 tokens, one must provide amount of 10000. In order
/// to burn 100.55 tokens, one must provide amount of 10055.
///
/// For non-fungible tokens the transaction body accepts a `serials` list of integers as a parameter.
///
/// - If no Supply Key is defined, the transaction will resolve to `TOKEN_HAS_NO_SUPPLY_KEY`.
///
/// - If neither the amount nor the `serials` get filled, a `INVALID_TOKEN_BURN_AMOUNT` response code
/// will be returned.
///
/// - If both amount and `serials` get filled, a `INVALID_TRANSACTION_BODY` response code will be
/// returned.
///
/// - If the `serials` list count is greater than the batch size limit global dynamic property, a
/// `BATCH_SIZE_LIMIT_EXCEEDED` response code will be returned.
///
/// - If the `serials` list contains a non-positive integer as a serial number, a `INVALID_NFT_ID`
/// response code will be returned.
pub type TokenBurnTransaction = Transaction<TokenBurnTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TokenBurnTransactionData {
    /// The token for which to burn tokens.
    token_id: Option<TokenId>,

    /// The amount of a fungible token to burn from the treasury account.
    amount: u64,

    /// The serial numbers of a non-fungible token to burn from the treasury account.
    serials: Vec<i64>,
}

impl TokenBurnTransaction {
    /// Returns the token for which to burn tokens.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data().token_id
    }

    /// Sets the token for which to burn tokens.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.data_mut().token_id = Some(token_id.into());
        self
    }

    /// Returns the amount of a fungible token to burn from the treasury account.
    #[must_use]
    pub fn get_amount(&self) -> u64 {
        self.data().amount
    }

    /// Sets the amount of a fungible token to burn from the treasury account.
    pub fn amount(&mut self, amount: impl Into<u64>) -> &mut Self {
        self.data_mut().amount = amount.into();
        self
    }

    /// Returns the serial numbers of a non-fungible token to burn from the treasury account.
    #[must_use]
    pub fn get_serials(&self) -> &[i64] {
        &self.data().serials
    }

    /// Sets the serial numbers of a non-fungible token to burn from the treasury account.
    pub fn serials(&mut self, serials: impl IntoIterator<Item = i64>) -> &mut Self {
        self.data_mut().serials = serials.into_iter().collect();
        self
    }
}

impl TransactionData for TokenBurnTransactionData {}

impl TransactionExecute for TokenBurnTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { TokenServiceClient::new(channel).burn_token(request).await })
    }
}

impl ValidateChecksums for TokenBurnTransactionData {
    fn validate_checksums(&self, ledger_id: &crate::ledger_id::RefLedgerId) -> Result<(), Error> {
        self.token_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TokenBurnTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::TokenBurn(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TokenBurnTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::TokenBurn(self.to_protobuf())
    }
}

impl From<TokenBurnTransactionData> for AnyTransactionData {
    fn from(transaction: TokenBurnTransactionData) -> Self {
        Self::TokenBurn(transaction)
    }
}

impl FromProtobuf<services::TokenBurnTransactionBody> for TokenBurnTransactionData {
    fn from_protobuf(pb: services::TokenBurnTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            token_id: Option::from_protobuf(pb.token)?,
            amount: pb.amount,
            serials: pb.serial_numbers,
        })
    }
}

impl ToProtobuf for TokenBurnTransactionData {
    type Protobuf = services::TokenBurnTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        let token = self.token_id.to_protobuf();
        let amount = self.amount;
        let serial_numbers = self.serials.clone();

        services::TokenBurnTransactionBody { token, amount, serial_numbers }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect_file;

    use crate::transaction::test_helpers::{
        test_token_id,
        transaction_body,
        unused_private_key,
        VALID_START,
    };
    use crate::{
        AnyTransaction,
        TokenBurnTransaction,
        TransactionId,
    };

    fn make_transaction() -> TokenBurnTransaction {
        let mut tx = TokenBurnTransaction::new();

        tx.node_account_ids(["0.0.5005".parse().unwrap(), "0.0.5006".parse().unwrap()])
            .transaction_id(TransactionId {
                account_id: "5006".parse().unwrap(),
                valid_start: VALID_START,
                nonce: None,
                scheduled: false,
            })
            .token_id(test_token_id())
            .amount(6 as u64)
            .freeze()
            .unwrap()
            .sign(unused_private_key());

        tx
    }

    fn make_transaction_nft() -> TokenBurnTransaction {
        let mut tx = TokenBurnTransaction::new();

        let vec1 = vec![1, 2, 64];

        tx.node_account_ids(["0.0.5005".parse().unwrap(), "0.0.5006".parse().unwrap()])
            .transaction_id(TransactionId {
                account_id: "5006".parse().unwrap(),
                valid_start: VALID_START,
                nonce: None,
                scheduled: false,
            })
            .token_id(test_token_id())
            .serials(vec1)
            .freeze()
            .unwrap()
            .sign(unused_private_key());

        tx
    }

    #[test]
    fn serialize_fungible() {
        let tx = make_transaction();
        let tx = transaction_body(tx);

        expect_file!["./snapshots/token_burn_transaction/serialize_fungible.txt"]
            .assert_debug_eq(&tx);
    }

    #[test]
    fn serialize_nft() {
        let tx = make_transaction_nft();
        let tx = transaction_body(tx);

        expect_file!["./snapshots/token_burn_transaction/serialize_nft.txt"].assert_debug_eq(&tx);
    }

    #[test]
    fn to_from_bytes() {
        let tx = make_transaction();
        let tx2 = AnyTransaction::from_bytes(&tx.to_bytes().unwrap()).unwrap();
        let tx = transaction_body(tx);
        let tx2 = transaction_body(tx2);

        assert_eq!(tx, tx2);
    }

    #[test]
    fn to_from_bytes_nft() {
        let tx = make_transaction_nft();
        let tx2 = AnyTransaction::from_bytes(&tx.to_bytes().unwrap()).unwrap();
        let tx = transaction_body(tx);
        let tx2 = transaction_body(tx2);

        assert_eq!(tx, tx2);
    }

    #[test]
    fn get_set_token_id() {
        let mut tx = TokenBurnTransaction::new();

        let tx2 = tx.token_id(test_token_id());

        assert_eq!(tx2.get_token_id(), Some(test_token_id()));
    }

    #[test]
    fn get_set_amount() {
        let mut tx = TokenBurnTransaction::new();

        let tx2 = tx.amount(23 as u64);

        assert_eq!(tx2.get_amount(), 23);
    }

    #[test]
    fn get_set_serial() {
        let serials = vec![1, 2, 64];

        let mut tx = TokenBurnTransaction::new();

        let tx2 = tx.serials(serials.to_owned());

        assert_eq!(tx2.get_serials(), serials);
    }
}
