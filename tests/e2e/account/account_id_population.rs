use hedera::{
    AccountId, Hbar, PrivateKey, TransactionReceiptQuery, TransferTransaction
};

use crate::common::{
    setup_nonfree,
    TestEnvironment,
};

#[tokio::test]
async fn can_populate_account_id_num() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(());
    };

    let private_key = PrivateKey::generate_ecdsa();
    let public_key = private_key.public_key();

    let evm_address = public_key.to_evm_address().unwrap();
    let evm_address_account = AccountId::from_evm_address(&evm_address);

    println!("evm_address: {evm_address_account:?}");
    let tx = TransferTransaction::new()
        .hbar_transfer(evm_address_account, Hbar::new(1))
        .hbar_transfer(client.get_operator_account_id().unwrap(), Hbar::new(-1))
        .execute(&client)
        .await?;

    println!("here1");
    let receipt = TransactionReceiptQuery::new()
        .transaction_id(tx.transaction_id)
        .include_children(true)
        .execute(&client)
        .await?;

    println!("here2");
    let new_account_id = receipt.children.get(0).unwrap().account_id.unwrap();
    
    println!("here3");
    let id_mirror = AccountId::from_evm_address(&evm_address);
    let account_id = id_mirror.populate_account_num(&client).await?;
    println!("here4");

    assert_eq!(new_account_id.num, account_id.num);

    Ok(())
}
