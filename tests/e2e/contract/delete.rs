use assert_matches::assert_matches;
use hedera::{
    ContractCreateTransaction,
    ContractDeleteTransaction,
    ContractFunctionParameters,
    ContractInfoQuery,
    Status,
};

use crate::common::{
    setup_nonfree,
    TestEnvironment,
};

#[tokio::test]
async fn admin_key() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else {
        return Ok(())
    };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(())
    };

    let file_id = super::bytecode_file_id(&client, op.private_key.public_key()).await?;

    let contract_id = ContractCreateTransaction::new()
        .admin_key(op.private_key.public_key())
        .gas(100000)
        .constructor_parameters(
            ContractFunctionParameters::new().add_string("Hello from Hedera.").to_bytes(None),
        )
        .bytecode_file_id(file_id)
        .contract_memo("[e2e::ContractCreateTransaction]")
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .contract_id
        .unwrap();

    ContractDeleteTransaction::new()
        .transfer_account_id(op.account_id)
        .contract_id(contract_id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    let res = ContractInfoQuery::new().contract_id(contract_id).execute(&client).await?;

    assert!(res.is_deleted);

    Ok(())
}

#[tokio::test]
async fn missing_admin_key_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else {
        return Ok(())
    };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(())
    };

    let file_id = super::bytecode_file_id(&client, op.private_key.public_key()).await?;

    let contract_id = ContractCreateTransaction::new()
        .gas(100000)
        .constructor_parameters(
            ContractFunctionParameters::new().add_string("Hello from Hedera.").to_bytes(None),
        )
        .bytecode_file_id(file_id)
        .contract_memo("[e2e::ContractCreateTransaction]")
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?
        .contract_id
        .unwrap();

    let res = ContractDeleteTransaction::new()
        .contract_id(contract_id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus {
            status: Status::ModifyingImmutableContract,
            transaction_id: _
        })
    );

    Ok(())
}

#[tokio::test]
async fn missing_contract_id_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let res = ContractDeleteTransaction::new().execute(&client).await;

    assert_matches!(
        res,
        Err(hedera::Error::TransactionPreCheckStatus {
            status: Status::InvalidContractId,
            transaction_id: _
        })
    );

    Ok(())
}
