use wasm_bindgen_test::*;
use hedera_proto::proto::{AccountId, Timestamp, TransactionBody, TransferList, AccountAmount, TransactionId, CryptoTransferTransactionBody, Duration};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_protobuf_types_in_wasm() {
    let account = AccountId {
        shard_num: 0,
        realm_num: 0,
        account: Some(hedera_proto::proto::account_id::Account::AccountNum(3)),
    };

    let timestamp = Timestamp {
        seconds: 1640995200,
        nanos: 0,
    };

    assert_eq!(account.shard_num, 0);
    assert_eq!(timestamp.seconds, 1640995200);
}

#[wasm_bindgen_test]
fn test_transaction_body_in_wasm() {
    let sender = AccountId {
        shard_num: 0,
        realm_num: 0,
        account: Some(hedera_proto::proto::account_id::Account::AccountNum(1001)),
    };

    let receiver = AccountId {
        shard_num: 0,
        realm_num: 0,
        account: Some(hedera_proto::proto::account_id::Account::AccountNum(1002)),
    };

    let transaction_id = TransactionId {
        transaction_valid_start: Some(Timestamp {
            seconds: 1640995200,
            nanos: 0,
        }),
        account_id: Some(sender.clone()),
        scheduled: false,
        nonce: 0,
    };

    let transfer = AccountAmount {
        account_id: Some(receiver.clone()),
        amount: 1000,
        is_approval: false,
    };

    let transfers = TransferList {
        account_amounts: vec![transfer],
    };

    let crypto_transfer = CryptoTransferTransactionBody {
        transfers: Some(transfers),
        token_transfers: Vec::new(),
    };

    let transaction_body = TransactionBody {
        transaction_id: Some(transaction_id),
        node_account_id: Some(AccountId {
            shard_num: 0,
            realm_num: 0,
            account: Some(hedera_proto::proto::account_id::Account::AccountNum(3)),
        }),
        transaction_fee: 100000,
        transaction_valid_duration: Some(Duration {
            seconds: 120,
        }),
        generate_record: false,
        memo: String::new(),
        data: Some(hedera_proto::proto::transaction_body::Data::CryptoTransfer(crypto_transfer)),
    };

    assert_eq!(transaction_body.transaction_fee, 100000);
    assert!(matches!(transaction_body.data, Some(hedera_proto::proto::transaction_body::Data::CryptoTransfer(_))));
    if let Some(hedera_proto::proto::transaction_body::Data::CryptoTransfer(transfer_body)) = transaction_body.data {
        let transfers = transfer_body.transfers.unwrap();
        assert_eq!(transfers.account_amounts.len(), 1);
        assert_eq!(transfers.account_amounts[0].amount, 1000);
    }
}
