use hedera::{
    AccountId,
    ContractCreateTransaction,
    ContractFunctionParameters,
    ContractId,
    ContractInfoQuery,
    FileCreateTransaction,
};

use crate::common::{
    setup_nonfree,
    TestEnvironment,
};

const CONTRACT_BYTE_CODE: &str = "608060405234801561001057600080fd5b50336000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff1602179055506101cb806100606000396000f3fe608060405260043610610046576000357c01000000000000000000000000000000000000000000000000000000009004806341c0e1b51461004b578063cfae321714610062575b600080fd5b34801561005757600080fd5b506100606100f2565b005b34801561006e57600080fd5b50610077610162565b6040518080602001828103825283818151815260200191508051906020019080838360005b838110156100b757808201518184015260208101905061009c565b50505050905090810190601f1680156100e45780820380516001836020036101000a031916815260200191505b509250505060405180910390f35b6000809054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff161415610160573373ffffffffffffffffffffffffffffffffffffffff16ff5b565b60606040805190810160405280600d81526020017f48656c6c6f2c20776f726c64210000000000000000000000000000000000000081525090509056fea165627a7a72305820ae96fb3af7cde9c0abfe365272441894ab717f816f07f41f07b1cbede54e256e0029";

#[tokio::test]
async fn can_populate_contract_id_num() -> anyhow::Result<()> {
    let Some(TestEnvironment { config, client }) = setup_nonfree() else {
        return Ok(());
    };

    let Some(op) = &config.operator else {
        log::debug!("skipping test due to missing operator");
        return Ok(());
    };

    let receipt = FileCreateTransaction::new()
        .keys([client.get_operator_public_key().unwrap()])
        .contents(CONTRACT_BYTE_CODE)
        .execute(&client)
        .await?
        .validate_status(true)
        .get_receipt(&client)
        .await?;

    let file_id = receipt.file_id.unwrap();

    let receipt = ContractCreateTransaction::new()
        .admin_key(op.private_key.public_key())
        .gas(100000)
        .constructor_parameters(
            ContractFunctionParameters::new().add_string("Hello from Hedera.").to_bytes(None),
        )
        .bytecode_file_id(file_id)
        .contract_memo("[e2e::can_populate_contract_id_num")
        .execute(&client)
        .await?
        .validate_status(true)
        .get_receipt(&client)
        .await?;

    let contract_id = receipt.contract_id.unwrap();

    let info = ContractInfoQuery::new().contract_id(contract_id).execute(&client).await?;

    let id_mirror = ContractId::from_evm_address(0, 0, &info.contract_account_id).unwrap();

    let new_contract_id = id_mirror.populate_contract_num(&client).await?;

    assert_eq!(contract_id, new_contract_id);
    Ok(())
}
