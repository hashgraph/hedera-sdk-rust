use crate::PrivateKey;

/// An opaque signer that can sign Hedera transactions.
///
/// Intended to be a temporary object that is generalized and passed into
/// a function accepting a `HederaSigner*`. Failure to do so will result in
/// a memory of leak.
pub struct Signer(pub(super) AnySigner);

pub(super) enum AnySigner {
    PrivateKey(PrivateKey),
}

/// Create an opaque signer from a `HederaPrivateKey`.
#[no_mangle]
pub extern "C" fn hedera_signer_private_key(key: *mut PrivateKey) -> *mut Signer {
    assert!(!key.is_null());

    let key = unsafe { &*key };
    let key = key.clone();

    let signer = Signer(AnySigner::PrivateKey(key));
    let signer = Box::into_raw(Box::new(signer));

    signer
}
