use crate::{
    AccountId,
    Client,
    PrivateKey,
};

/// Construct a Hedera client pre-configured for mainnet access.
#[no_mangle]
pub extern "C" fn hedera_client_for_mainnet() -> *mut Client {
    let client = Client::for_mainnet();
    let client = Box::into_raw(Box::new(client));

    client
}

/// Construct a Hedera client pre-configured for testnet access.
#[no_mangle]
pub extern "C" fn hedera_client_for_testnet() -> *mut Client {
    let client = Client::for_testnet();
    let client = Box::into_raw(Box::new(client));

    client
}

/// Construct a Hedera client pre-configured for previewnet access.
#[no_mangle]
pub extern "C" fn hedera_client_for_previewnet() -> *mut Client {
    let client = Client::for_previewnet();
    let client = Box::into_raw(Box::new(client));

    client
}

/// Release memory associated with the previously-opened Hedera client.
#[no_mangle]
pub extern "C" fn hedera_client_free(client: *mut Client) {
    assert!(!client.is_null());

    let _client = unsafe { Box::from_raw(client) };
}

/// Sets the account that will, by default, be paying for transactions and queries built with
/// this client.
#[no_mangle]
pub extern "C" fn hedera_client_set_operator(
    client: *mut Client,
    id_shard: u64,
    id_realm: u64,
    id_num: u64,
    key: *mut PrivateKey,
) {
    assert!(!client.is_null());
    assert!(!key.is_null());

    let client = unsafe { &*client };

    let key = unsafe { &*key };
    let key = key.clone();

    client.set_operator(
        AccountId { shard: id_shard, realm: id_realm, num: id_num, alias: None },
        key,
    );
}
