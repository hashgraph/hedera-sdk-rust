use std::str::FromStr;
use std::sync::Arc;

use assert_matches::assert_matches;
use expect_test::expect;
use hex_literal::hex;
use pkcs8::AssociatedOid;

use super::{
    PrivateKey,
    PrivateKeyDataWrapper,
};
use crate::Error;

#[test]
fn ed25519_from_str() {
    const S: &str = "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312";
    let pk = PrivateKey::from_str(S).unwrap();
    // ensure round-tripping works.
    assert_eq!(pk.to_string(), S);
}

#[test]
fn ecdsa_from_str() {
    const S: &str = "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048";
    let pk = PrivateKey::from_str(S).unwrap();

    assert_eq!(pk.algorithm().oid, k256::Secp256k1::OID);

    assert_eq!(pk.to_string(), S);
}

#[test]
fn ed25519_sign() {
    let private_key = PrivateKey::from_str(
        "302e020100300506032b657004220420db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10",
    )
    .unwrap();

    let signature = private_key.sign(b"hello, world");
    expect![[r#"
        "9d04bfed7baa97c80d29a6ae48c0d896ce8463a7ea0c16197d55a563c73996ef062b2adf507f416c108422c0310fc6fb21886e11ce3de3e951d7a56049743f07"
    "#]]
        .assert_debug_eq(&hex::encode(signature));
}

#[test]
fn ecdsa_sign() {
    let private_key = PrivateKey::from_str(
        "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048"
    )
    .unwrap();

    // notice that this doesn't match other impls
    // this is to avoid signature malleability.
    // see: https://github.com/bitcoin/bips/blob/43da5dec5eaf0d8194baa66ba3dd976f923f9d07/bip-0032.mediawiki
    let signature = private_key.sign(b"hello world");
    expect![[r#"
        "f3a13a555f1f8cd6532716b8f388bd4e9d8ed0b252743e923114c0c6cbfe414c086e3717a6502c3edff6130d34df252fb94b6f662d0cd27e2110903320563851"
    "#]]
    .assert_debug_eq(&hex::encode(signature));
}

#[test]
fn ed25519_legacy_derive() {
    // private key was lifted from a Mnemonic test.
    let private_key = PrivateKey::from_str(
        "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312",
    )
    .unwrap();

    let private_key_0 = private_key.legacy_derive(0).unwrap();

    assert_eq!(private_key_0.to_string(), "302e020100300506032b6570042204202b7345f302a10c2a6d55bf8b7af40f125ec41d780957826006d30776f0c441fb");

    let private_key_neg_1 = private_key.legacy_derive(-1).unwrap();

    assert_eq!(private_key_neg_1.to_string(), "302e020100300506032b657004220420caffc03fdb9853e6a91a5b3c57a5c0031d164ce1c464dea88f3114786b5199e5");
}

#[test]
fn ed25519_legacy_derive_2() {
    let private_key = PrivateKey::from_str(
        "302e020100300506032b65700422042000c2f59212cb3417f0ee0d38e7bd876810d04f2dd2cb5c2d8f26ff406573f2bd",
    )
    .unwrap();

    let private_key_mhw = private_key.legacy_derive(0xffffffffff).unwrap();

    assert_eq!(private_key_mhw.to_string(), "302e020100300506032b6570042204206890dc311754ce9d3fc36bdf83301aa1c8f2556e035a6d0d13c2cccdbbab1242")
}

/// This is for testing purposes only.
///
/// # Panics
/// If `data` and `chain_code` don't make a valid [`PrivateKey`].
fn key_with_chain(data: &str, chain_code: [u8; 32]) -> PrivateKey {
    let key_without_chain = PrivateKey::from_str(data).unwrap();

    let data = match Arc::try_unwrap(key_without_chain.0) {
        Ok(it) => it.data,
        Err(_) => unreachable!(),
    };

    PrivateKey(Arc::new(PrivateKeyDataWrapper::new_derivable(data, chain_code)))
}

// "iosKey"
#[test]
fn ed25519_derive_1() {
    // have to create a private key with a chain code, which means... Hacks!
    // luckily, we're a unit test, so, we can access private fields.
    let key = key_with_chain(
        "302e020100300506032b657004220420a6b9548d7e123ad4c8bc6fee58301e9b96360000df9d03785c07b620569e7728",
        hex!("cde7f535264f1db4e2ded409396f8c72f8075cc43757bd5a205c97699ea40271"),
    );

    let child_key = key.derive(0).unwrap();

    expect![[r#"
            PrivateKeyData {
                algorithm: Ed25519,
                key: "5f66a51931e8c99089472e0d70516b6272b94dd772b967f8221e1077f966dbda",
                chain_code: Some(
                    "0e5c869c1cf9daecd03edb2d49cf2621412578a352578a4bb7ef4eef2942b7c9",
                ),
            }
        "#]]
    .assert_debug_eq(&*child_key.0);
}

// "androidKey"
#[test]
fn ed25519_derive_2() {
    let key = key_with_chain(
        "302e020100300506032b65700422042097dbce1988ef8caf5cf0fd13a5374969e2be5f50650abd19314db6b32f96f18e",
        hex!("b7b406314eb2224f172c1907fe39f807e306655e81f2b3bc4766486f42ef1433")
    );
    let child_key = key.derive(0).unwrap();

    expect![[r#"
            PrivateKeyData {
                algorithm: Ed25519,
                key: "c284c25b3a1458b59423bc289e83703b125c8eefec4d5aa1b393c2beb9f2bae6",
                chain_code: Some(
                    "a7a1c2d115a988e51efc12c23692188a4796b312a4a700d6c703e4de4cf1a7f6",
                ),
            }
        "#]]
    .assert_debug_eq(&child_key.0);
}

#[test]
fn ed25519_from_pem() {
    const PEM: &[u8] = br#"-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEINtIS4KOZLLY8SzjwKDpOguMznrxu485yXcyOUSCU44Q
-----END PRIVATE KEY-----"#;

    let pk = PrivateKey::from_pem(PEM).unwrap();

    assert_eq!(pk.to_string(), "302e020100300506032b657004220420db484b828e64b2d8f12ce3c0a0e93a0b8cce7af1bb8f39c97732394482538e10");
}

#[test]
fn ecdsa_from_pem() {
    const PEM: &[u8] = br#"-----BEGIN PRIVATE KEY-----
MDACAQAwBwYFK4EEAAoEIgQgh3bGuDGhthrBDawDBKKEPeRxb1SxkZu5GiaF0P4/
MEg=
-----END PRIVATE KEY-----"#;

    let pk = PrivateKey::from_pem(PEM).unwrap();

    assert_eq!(pk.to_string(), "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048");
}

#[test]
fn ed25519_from_pem_invalid_type_label() {
    // extra `S` in the type label
    const PEM: &[u8] = br#"-----BEGIN PRIVATE KEYS-----
MC4CAQAwBQYDK2VwBCIEINtIS4KOZLLY8SzjwKDpOguMznrxu485yXcyOUSCU44Q
-----END PRIVATE KEYS-----"#;

    assert_matches!(PrivateKey::from_pem(PEM), Err(Error::KeyParse(_)));
}
