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

use assert_matches::assert_matches;
use hedera::{
    Hbar,
    Status,
    TokenUnpauseTransaction,
};

use crate::account::Account;
use crate::common::{
    setup_nonfree,
    TestEnvironment,
};
use crate::token::{
    CreateFungibleToken,
    FungibleToken,
    Key,
    TokenKeys,
};

const TOKEN_PARAMS: CreateFungibleToken = CreateFungibleToken {
    initial_supply: 0,
    keys: TokenKeys { pause: Some(Key::Owner), ..TokenKeys::DEFAULT },
};

#[tokio::test]
async fn basic() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let owner = Account::create(Hbar::new(0), &client).await?;

    let token = FungibleToken::create(&client, &owner, TOKEN_PARAMS).await?;

    TokenUnpauseTransaction::new()
        .token_id(token.id)
        .sign(token.owner.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    Ok(())
}

#[tokio::test]
async fn missing_token_id_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let res = TokenUnpauseTransaction::new().execute(&client).await;

    assert_matches!(
        res,
        Err(hedera::Error::TransactionPreCheckStatus { status: Status::InvalidTokenId, transaction_id: _ })
    );

    Ok(())
}

#[tokio::test]
async fn missing_pause_key_sig_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let owner = Account::create(Hbar::new(0), &client).await?;

    let token = FungibleToken::create(&client, &owner, TOKEN_PARAMS).await?;

    let res = TokenUnpauseTransaction::new()
        .token_id(token.id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidSignature, transaction_id: _ })
    );

    Ok(())
}

#[tokio::test]
async fn missing_pause_key_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let owner = Account::create(Hbar::new(0), &client).await?;

    let token = FungibleToken::create(&client, &owner, CreateFungibleToken::default()).await?;

    let res = TokenUnpauseTransaction::new()
        .token_id(token.id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::TokenHasNoPauseKey, transaction_id: _ })
    );

    Ok(())
}
