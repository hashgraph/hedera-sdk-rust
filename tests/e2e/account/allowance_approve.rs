use hedera::{
    AccountAllowanceApproveTransaction,
    Hbar,
    TokenAssociateTransaction,
    TransactionId,
    TransferTransaction,
};

use crate::account::Account;
use crate::common::{
    setup_nonfree,
    TestEnvironment,
};

#[tokio::test]
async fn spend() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let (alice, bob) = tokio::try_join!(
        Account::create(Hbar::new(10), &client),
        Account::create(Hbar::new(10), &client)
    )?;

    AccountAllowanceApproveTransaction::new()
        .approve_hbar_allowance(bob.id, alice.id, Hbar::new(10))
        .freeze_with(&client)?
        .sign(bob.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let transfer_record = TransferTransaction::new()
        .hbar_transfer(client.get_operator_account_id().unwrap(), Hbar::new(5))
        .approved_hbar_transfer(bob.id, Hbar::new(-5))
        .transaction_id(TransactionId::generate(alice.id))
        .freeze_with(&client)?
        .sign(alice.key.clone())
        .execute(&client)
        .await?
        .get_record(&client)
        .await?;

    assert!(transfer_record
        .transfers
        .iter()
        .any(|it| it.account_id == client.get_operator_account_id().unwrap()
            && it.amount == Hbar::new(5)));

    let _ = tokio::try_join!(alice.delete(&client), bob.delete(&client))?;

    Ok(())
}

#[tokio::test]
async fn missing_nft_allowance_approval_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let (treasury, spender) = tokio::try_join!(
        Account::create(Hbar::new(0), &client),
        Account::create(Hbar::new(1), &client),
    )?;

    let nft_collection = crate::token::Nft::create(&client, &treasury).await?;

    TokenAssociateTransaction::new()
        .account_id(spender.id)
        .token_ids([nft_collection.id])
        .sign(spender.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let serials = nft_collection.mint(&client, [b"asd"]).await?;

    let nft1 = nft_collection.id.nft(serials[0] as u64);

    let res = TransferTransaction::new()
        .approved_nft_transfer(nft1, treasury.id, spender.id)
        .transaction_id(TransactionId::generate(spender.id))
        .sign(spender.key.clone())
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches::assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus {
            status: hedera::Status::SpenderDoesNotHaveAllowance,
            transaction_id: _
        })
    );

    nft_collection.burn(&client, serials).await?;
    nft_collection.delete(&client).await?;

    let _ = tokio::try_join!(treasury.delete(&client), spender.delete(&client),)?;

    Ok(())
}
