use assert_matches::assert_matches;
use hedera::{
    ContractCreateTransaction, ContractDeleteTransaction, ContractExecuteTransaction,
    ContractFunctionParameters, Status,
};

use crate::common::{setup_nonfree, TestEnvironment};
use crate::contract::bytecode_file_id;

#[tokio::test]
async fn basic() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else {
        return Ok(())
    };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(())
    };

    let file_id = bytecode_file_id(&client, op.private_key.public_key()).await?;

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

    let _ = ContractExecuteTransaction::new()
        .contract_id(contract_id)
        .gas(100000)
        .function_with_parameters(
            "setMessage",
            ContractFunctionParameters::new().add_string("new message"),
        )
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    ContractDeleteTransaction::new()
        .transfer_account_id(op.account_id)
        .contract_id(contract_id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    Ok(())
}

#[tokio::test]
async fn missing_contract_id_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config: _, client }) = setup_nonfree() else {
        return Ok(())
    };

    let res = ContractExecuteTransaction::new()
        .gas(100000)
        .function_with_parameters(
            "setMessage",
            ContractFunctionParameters::new().add_string("new message"),
        )
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::InvalidContractId, transaction_id: _ })
    );

    Ok(())
}

#[tokio::test]
async fn missing_function_parameters_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else {
        return Ok(())
    };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(())
    };

    let file_id = bytecode_file_id(&client, op.private_key.public_key()).await?;

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

    let res = ContractExecuteTransaction::new()
        .contract_id(contract_id)
        .gas(100000)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus {
            status: Status::ContractRevertExecuted,
            transaction_id: _
        })
    );

    ContractDeleteTransaction::new()
        .transfer_account_id(op.account_id)
        .contract_id(contract_id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    Ok(())
}

#[tokio::test]
async fn missing_gas_fails() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else {
        return Ok(())
    };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(())
    };

    let file_id = bytecode_file_id(&client, op.private_key.public_key()).await?;

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

    let res = ContractExecuteTransaction::new()
        .contract_id(contract_id)
        .function_with_parameters(
            "setMessage",
            ContractFunctionParameters::new().add_string("new message"),
        )
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await;

    assert_matches!(
        res,
        Err(hedera::Error::ReceiptStatus { status: Status::InsufficientGas, transaction_id: _ })
    );

    ContractDeleteTransaction::new()
        .transfer_account_id(op.account_id)
        .contract_id(contract_id)
        .execute(&client)
        .await?
        .get_receipt(&client)
        .await?;

    Ok(())
}
