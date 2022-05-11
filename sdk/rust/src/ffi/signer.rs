use crate::PrivateKey;

/// An opaque signer that can sign Hedera transactions.
///
/// Intended to be a temporary object that is generalized and passed into
/// a function accepting a `HederaSigner*`. Failure to do so will result in
/// a memory of leak.
#[repr(C)]
pub struct Signer(pub Box<dyn crate::Signer>);

/// Create an opaque signer from a `HederaPrivateKey`.
#[no_mangle]
pub extern "C" fn hedera_signer_private_key(key: *mut PrivateKey) -> *mut Signer {
    assert!(!key.is_null());

    let key = unsafe { &*key };
    let key = key.clone();

    let signer = Signer(Box::new(key));
    let signer = Box::into_raw(Box::new(signer));

    signer
}
