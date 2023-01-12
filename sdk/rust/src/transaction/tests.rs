use std::collections::HashSet;

use assert_matches::assert_matches;
use time::OffsetDateTime;

use crate::transaction::AnyTransactionData;
use crate::{
    AnyTransaction,
    Hbar,
    PrivateKey,
    TransactionId,
    TransferTransaction,
};

#[test]
fn to_bytes_from_bytes() -> crate::Result<()> {
    let mut tx = TransferTransaction::new();

    let bytes = tx
        .max_transaction_fee(Hbar::new(10))
        .transaction_valid_duration(time::Duration::seconds(119))
        .transaction_memo("hi hashgraph")
        .hbar_transfer(2.into(), Hbar::new(2))
        .hbar_transfer(101.into(), Hbar::new(-2))
        .transaction_id(TransactionId {
            account_id: 101.into(),
            valid_start: OffsetDateTime::now_utc(),
            nonce: None,
            scheduled: false,
        })
        .node_account_ids([6.into(), 7.into()])
        .freeze()?
        .to_bytes()?;

    let tx = tx;

    let lhs = tx.data();

    let tx2 = AnyTransaction::from_bytes(&bytes)?;

    let rhs = assert_matches!(tx2.data(), AnyTransactionData::Transfer(it) => it);

    assert_eq!(tx.get_max_transaction_fee(), tx2.get_max_transaction_fee());

    // note: they have *no* guaranteed order, so we have to convert to a `HashSet`...
    // `HashSet` makes this hard on us.
    {
        let lhs: Option<HashSet<_>> = tx.get_node_account_ids().map(|it| it.iter().collect());
        let rhs: Option<HashSet<_>> = tx2.get_node_account_ids().map(|it| it.iter().collect());
        assert_eq!(lhs, rhs);
    }

    assert_eq!(tx.get_transaction_id(), tx2.get_transaction_id());
    assert_eq!(tx.get_transaction_memo(), tx2.get_transaction_memo());
    assert_eq!(tx.get_transaction_valid_duration(), tx2.get_transaction_valid_duration());
    assert_eq!(lhs, rhs);
    assert!(tx2.sources.is_some());

    Ok(())
}

#[test]
fn from_bytes_sign_to_bytes() -> crate::Result<()> {
    let mut tx = TransferTransaction::new();

    let bytes = tx
        .max_transaction_fee(Hbar::new(10))
        .transaction_valid_duration(time::Duration::seconds(119))
        .transaction_memo("hi hashgraph")
        .hbar_transfer(2.into(), Hbar::new(2))
        .hbar_transfer(101.into(), Hbar::new(-2))
        .transaction_id(TransactionId {
            account_id: 101.into(),
            valid_start: OffsetDateTime::now_utc(),
            nonce: None,
            scheduled: false,
        })
        .node_account_ids([6.into(), 7.into()])
        .freeze()?
        .to_bytes()?;

    let mut tx2 = AnyTransaction::from_bytes(&bytes)?;
    dbg!(PrivateKey::generate_ed25519());

    tx2.sign(PrivateKey::generate_ed25519());

    let _bytes2 = tx2.to_bytes()?;

    // todo: check properties (but what properties?)

    Ok(())
}
