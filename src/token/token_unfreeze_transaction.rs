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
    AccountId,
    BoxGrpcFuture,
    TokenId,
    Transaction,
    ValidateChecksums,
};

/// Unfreezes transfers of the specified token for the account. Must be signed by the Token's freezeKey.
///
/// Once executed the Account is marked as Unfrozen and will be able to receive or send tokens.
/// The operation is idempotent.
///
/// - If the provided account is not found, the transaction will resolve to `INVALID_ACCOUNT_ID`.
/// - If the provided account has been deleted, the transaction will resolve to `ACCOUNT_DELETED`.
/// - If the provided token is not found, the transaction will resolve to `INVALID_TOKEN_ID`.
/// - If the provided token has been deleted, the transaction will resolve to `TOKEN_WAS_DELETED`.
/// - If an Association between the provided token and account is not found, the transaction will
/// resolve to `TOKEN_NOT_ASSOCIATED_TO_ACCOUNT`.
/// - If no Freeze Key is defined, the transaction will resolve to `TOKEN_HAS_NO_FREEZE_KEY`.
pub type TokenUnfreezeTransaction = Transaction<TokenUnfreezeTransactionData>;

#[derive(Debug, Clone, Default)]
pub struct TokenUnfreezeTransactionData {
    /// The account to be unfrozen.
    account_id: Option<AccountId>,

    /// The token for which this account will be unfrozen.
    token_id: Option<TokenId>,
}

impl TokenUnfreezeTransaction {
    /// Returns the account to be unfrozen.
    #[must_use]
    pub fn get_account_id(&self) -> Option<AccountId> {
        self.data().account_id
    }

    /// Sets the account to be unfrozen.
    pub fn account_id(&mut self, account_id: AccountId) -> &mut Self {
        self.data_mut().account_id = Some(account_id);
        self
    }

    /// Returns the token for which the account will be unfrozen.
    #[must_use]
    pub fn get_token_id(&self) -> Option<TokenId> {
        self.data().token_id
    }

    /// Sets the token for which this account will be unfrozen.
    pub fn token_id(&mut self, token_id: impl Into<TokenId>) -> &mut Self {
        self.data_mut().token_id = Some(token_id.into());
        self
    }
}

impl TransactionData for TokenUnfreezeTransactionData {}

impl TransactionExecute for TokenUnfreezeTransactionData {
    fn execute(
        &self,
        channel: Channel,
        request: services::Transaction,
    ) -> BoxGrpcFuture<'_, services::TransactionResponse> {
        Box::pin(async { TokenServiceClient::new(channel).unfreeze_token_account(request).await })
    }
}

impl ValidateChecksums for TokenUnfreezeTransactionData {
    fn validate_checksums(&self, ledger_id: &RefLedgerId) -> crate::Result<()> {
        self.token_id.validate_checksums(ledger_id)?;
        self.account_id.validate_checksums(ledger_id)
    }
}

impl ToTransactionDataProtobuf for TokenUnfreezeTransactionData {
    fn to_transaction_data_protobuf(
        &self,
        chunk_info: &ChunkInfo,
    ) -> services::transaction_body::Data {
        let _ = chunk_info.assert_single_transaction();

        services::transaction_body::Data::TokenUnfreeze(self.to_protobuf())
    }
}

impl ToSchedulableTransactionDataProtobuf for TokenUnfreezeTransactionData {
    fn to_schedulable_transaction_data_protobuf(
        &self,
    ) -> services::schedulable_transaction_body::Data {
        services::schedulable_transaction_body::Data::TokenUnfreeze(self.to_protobuf())
    }
}

impl From<TokenUnfreezeTransactionData> for AnyTransactionData {
    fn from(transaction: TokenUnfreezeTransactionData) -> Self {
        Self::TokenUnfreeze(transaction)
    }
}

impl FromProtobuf<services::TokenUnfreezeAccountTransactionBody> for TokenUnfreezeTransactionData {
    fn from_protobuf(pb: services::TokenUnfreezeAccountTransactionBody) -> crate::Result<Self> {
        Ok(Self {
            account_id: Option::from_protobuf(pb.account)?,
            token_id: Option::from_protobuf(pb.token)?,
        })
    }
}

impl ToProtobuf for TokenUnfreezeTransactionData {
    type Protobuf = services::TokenUnfreezeAccountTransactionBody;

    fn to_protobuf(&self) -> Self::Protobuf {
        let account = self.account_id.to_protobuf();
        let token = self.token_id.to_protobuf();

        services::TokenUnfreezeAccountTransactionBody { token, account }
    }
}

#[cfg(test)]
mod tests {
    use expect_test::expect;

    use crate::transaction::test_helpers::{
        check_body,
        transaction_body,
    };
    use crate::{
        AccountId,
        AnyTransaction,
        TokenId,
        TokenUnfreezeTransaction,
    };

    fn make_transaction() -> TokenUnfreezeTransaction {
        let mut tx = TokenUnfreezeTransaction::new_for_tests();

        tx.token_id(TokenId::new(6, 5, 4)).account_id(AccountId::new(0, 0, 222)).freeze().unwrap();

        tx
    }

    #[test]
    fn seriralize() {
        let tx = make_transaction();

        let tx = transaction_body(tx);

        let tx = check_body(tx);

        expect![[r#"
            TokenUnfreeze(
                TokenUnfreezeAccountTransactionBody {
                    token: Some(
                        TokenId {
                            shard_num: 6,
                            realm_num: 5,
                            token_num: 4,
                        },
                    ),
                    account: Some(
                        AccountId {
                            shard_num: 0,
                            realm_num: 0,
                            account: Some(
                                AccountNum(
                                    222,
                                ),
                            ),
                        },
                    ),
                },
            )
        "#]]
        .assert_debug_eq(&tx);
    }

    #[test]
    fn to_from_bytes() {
        let tx = make_transaction();

        let tx2 = AnyTransaction::from_bytes(&tx.to_bytes().unwrap()).unwrap();

        let tx = transaction_body(tx);
        let tx2 = transaction_body(tx2);

        assert_eq!(tx, tx2);
    }
}
