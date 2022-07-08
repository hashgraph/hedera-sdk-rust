use crate::ffi::signer::{
    AnySigner,
    Signer,
};
use crate::{
    AccountId,
    Client,
};

/// Construct a Hedera client pre-configured for testnet access.
#[no_mangle]
pub extern "C" fn hedera_client_for_testnet() -> *mut Client {
    let client = Client::for_testnet();
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
pub extern "C" fn hedera_client_set_payer_account_id(
    client: *mut Client,
    id_shard: u64,
    id_realm: u64,
    id_num: u64,
) {
    assert!(!client.is_null());

    let client = unsafe { &*client };
    client.set_payer_account_id(AccountId { shard: id_shard, realm: id_realm, num: id_num });
}

/// Gets the account that is, by default, paying for transactions and queries built with
/// this client.
#[no_mangle]
pub extern "C" fn hedera_client_get_payer_account_id(
    client: *mut Client,
    id_shard: *mut u64,
    id_realm: *mut u64,
    id_num: *mut u64,
) {
    assert!(!client.is_null());

    let client = unsafe { &*client };

    if let Some(payer_account_id) = client.payer_account_id() {
        unsafe {
            *id_shard = payer_account_id.shard;
            *id_realm = payer_account_id.realm;
            *id_num = payer_account_id.num;
        }
    }
}

/// Adds a signer that will, by default, sign for all transactions and queries built
/// with this client.
///
/// Takes ownership of the passed signer.
///
#[no_mangle]
pub extern "C" fn hedera_client_add_default_signer(client: *mut Client, signer: *mut Signer) {
    assert!(!client.is_null());
    assert!(!signer.is_null());

    let client = unsafe { &*client };
    let signer = unsafe { Box::from_raw(signer) };

    match signer.0 {
        AnySigner::PrivateKey(key) => client.add_default_signer(key),
    }
}
