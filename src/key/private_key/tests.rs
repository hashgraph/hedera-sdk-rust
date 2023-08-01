use std::str::FromStr;

use assert_matches::assert_matches;
use expect_test::expect;
use hex_literal::hex;
use triomphe::Arc;

use super::PrivateKey;
use crate::key::private_key::{
    ED25519_OID,
    K256_OID,
};
use crate::Error;

#[test]
fn ed25519_from_str() {
    const S: &str = "302e020100300506032b65700422042098aa82d6125b5efa04bf8372be7931d05cd77f5ef3330b97d6ee7c006eaaf312";
    let pk = PrivateKey::from_str(S).unwrap();

    assert_eq!(pk.algorithm().oid, ED25519_OID);

    // ensure round-tripping works.
    assert_eq!(pk.to_string(), S);
}

#[test]
fn ecdsa_from_str() {
    const S: &str = "3030020100300706052b8104000a042204208776c6b831a1b61ac10dac0304a2843de4716f54b1919bb91a2685d0fe3f3048";
    let pk = PrivateKey::from_str(S).unwrap();

    assert_eq!(pk.algorithm().oid, K256_OID);

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
    // see: https://github.com/bitcoin/bips/blob/43da5dec5eaf0d8194baa66ba3dd976f923f9d07/bip-0062.mediawiki
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

    let private_key_mhw = private_key.legacy_derive(0x00ff_ffff_ffff).unwrap();

    assert_eq!(private_key_mhw.to_string(), "302e020100300506032b6570042204206890dc311754ce9d3fc36bdf83301aa1c8f2556e035a6d0d13c2cccdbbab1242")
}

/// This is for testing purposes only.
///
/// # Panics
/// If `data` and `chain_code` don't make a valid [`PrivateKey`].
fn key_with_chain(data: &str, chain_code: [u8; 32]) -> PrivateKey {
    let mut key = PrivateKey::from_str(data).unwrap();

    // note: we create the key here, so, there shouldn't be any other references to it.
    Arc::get_mut(&mut key.0).unwrap().chain_code = Some(chain_code);

    key
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
fn ed25519_from_pem_with_password() {
    const PEM: &[u8] = b"-----BEGIN ENCRYPTED PRIVATE KEY-----
MIGbMFcGCSqGSIb3DQEFDTBKMCkGCSqGSIb3DQEFDDAcBAjeB6TNNQX+1gICCAAw
DAYIKoZIhvcNAgkFADAdBglghkgBZQMEAQIEENfMacg1/Txd/LhKkxZtJe0EQEVL
mez3xb+sfUIF3TKEIDJtw7H0xBNlbAfLxTV11pofiar0z1/WRBHFFUuGIYSiKjlU
V9RQhAnemO84zcZfTYs=
-----END ENCRYPTED PRIVATE KEY-----";

    let pk = PrivateKey::from_pem_with_password(PEM, "test").unwrap();

    assert_eq!(pk.to_string(), "302e020100300506032b6570042204208d8df406a762e36dfbf6dda2239f38a266db369e09bca6a8569e9e79b4826152");
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

#[test]
fn ed25519_pkcs8_unencrypted_pem() {
    const S: &str = r"-----BEGIN PRIVATE KEY-----
MC4CAQAwBQYDK2VwBCIEIOgbjaHgEqF7PY0t2dUf2VU0u1MRoKii/fywDlze4lvl
-----END PRIVATE KEY-----";

    let pk = PrivateKey::from_pem(S).unwrap();

    assert_eq!(pk.algorithm().oid, ED25519_OID);
    assert_eq!(
        pk.to_string_raw(),
        "e81b8da1e012a17b3d8d2dd9d51fd95534bb5311a0a8a2fdfcb00e5cdee25be5"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "f7b9aa4a8e4eee94e4277dfe757d8d7cde027e7cd5349b7d8e6ee21c9b9395be"
    );
}

#[test]
fn ecdsa_ec_unencrypted_uncompressed_pem() {
    const S: &str = r"-----BEGIN EC PRIVATE KEY-----
MHQCAQEEIG8I+jKi+iGVa7ttbfnlnML5AdvPugbgBWnseYjrle6qoAcGBSuBBAAK
oUQDQgAEqf5BmMeBzkU1Ra9UAbZJo3tytVOlb7erTc36LRLP20mOLU7+mFY+3Cfe
fAZgBtPXRAmDtRvYGODswAalW85GKA==
-----END EC PRIVATE KEY-----";

    let pk = PrivateKey::from_pem(S).unwrap();

    assert_eq!(pk.algorithm().oid, K256_OID);
    assert_eq!(
        pk.to_string_raw(),
        "6f08fa32a2fa21956bbb6d6df9e59cc2f901dbcfba06e00569ec7988eb95eeaa"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "02a9fe4198c781ce453545af5401b649a37b72b553a56fb7ab4dcdfa2d12cfdb49"
    );
}

#[test]
fn ecdsa_ec_unencrypted_compressed_pem() {
    const S: &str = r"-----BEGIN EC PRIVATE KEY-----
MFQCAQEEIOHyhclwHbha3f281Kvd884rhBzltxGJxCZyaQCagH9joAcGBSuBBAAK
oSQDIgACREr6gFZa4K7hBP+bA25VdgQ+0ABFgM+g5RYw/W6T1Og=
-----END EC PRIVATE KEY-----";
    let pk = PrivateKey::from_pem(S).unwrap();

    assert_eq!(pk.algorithm().oid, K256_OID);
    assert_eq!(
        pk.to_string_raw(),
        "e1f285c9701db85addfdbcd4abddf3ce2b841ce5b71189c4267269009a807f63"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "02444afa80565ae0aee104ff9b036e5576043ed0004580cfa0e51630fd6e93d4e8"
    );
}

#[test]
fn ed25519_pkcs8_encrypted_pem() {
    const S: &str = r"-----BEGIN ENCRYPTED PRIVATE KEY-----
MIGbMFcGCSqGSIb3DQEFDTBKMCkGCSqGSIb3DQEFDDAcBAiho4GvPxvL6wICCAAw
DAYIKoZIhvcNAgkFADAdBglghkgBZQMEAQIEEIdsubXR0QvxXGSprqDuDXwEQJZl
OBtwm2p2P7WrWE0OnjGxUe24fWwdrvJUuguFtH3FVWc8C5Jbxgbyxsuzbf+utNL6
0ey+WdbGL06Bw0HGqs8=
-----END ENCRYPTED PRIVATE KEY-----";
    let pk = PrivateKey::from_pem_with_password(S, "asdasd123").unwrap();

    assert_eq!(pk.algorithm().oid, ED25519_OID);
    assert_eq!(
        pk.to_string_raw(),
        "fa0857e963946d5f5e035684c40354d3cd3dcc80c0fb77beac2ef7c4b5271599"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "202af61e141465d4bf2c356d37d18bd026c246bde4eb73258722ad11f790be4e"
    );
}

#[test]
fn ecdsa_ec_encrypted_uncompressed_pem() {
    const S: &str = r"-----BEGIN EC PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-128-CBC,0046A9EED8D16F0CAA66A197CE8BE8BD

9VU9gReUmrn4XywjMx0F0A3oGzpHIksEXma72TCSdcxI7zHy0mtzuGq4Wd25O38s
H9c6kvhTPS1N/c6iNhx154B0HUoND8jvAvfxbGR/R87vpZJsOoKCmRxGqrxG8HER
FIHQ1jy16DrAbU95kDyLsiF1dy2vUY/HoqFZwxl/IVc=
-----END EC PRIVATE KEY-----";

    let pk = PrivateKey::from_pem_with_password(S, "asdasd123").unwrap();

    assert_eq!(pk.algorithm().oid, K256_OID);
    assert_eq!(
        pk.to_string_raw(),
        "cf49eb5206c1b0468854d6ea7b370590619625514f71ff93608a18465e4012ad"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "025f0d14a7562d6319e5b8f91620d2ce9ad13d9abf21cfe9bd0a092c0f35bf1701"
    );
}

#[test]
fn ecdsa_ec_encrypted_compressed_pem() {
    const S: &str = r"-----BEGIN EC PRIVATE KEY-----
Proc-Type: 4,ENCRYPTED
DEK-Info: AES-128-CBC,4A9B3B987EC2EFFA405818327D14FFF7

Wh756RkK5fn1Ke2denR1OYfqE9Kr4BXhgrEMTU/6o0SNhMULUhWGHrCWvmNeEQwp
ZVZYUxgYoTlJBeREzKAZithcvxIcTbQfLABo1NZbjA6YKqAqlGpM6owwL/f9e2ST
-----END EC PRIVATE KEY-----";
    let pk = PrivateKey::from_pem_with_password(S, "asdasd123").unwrap();

    assert_eq!(pk.algorithm().oid, K256_OID);
    assert_eq!(
        pk.to_string_raw(),
        "c0d3e16ba5a1abbeac4cd327a3c3c1cc10438431d0bac019054e573e67768bb5"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "02065f736378134c53c7a2ee46f199fb93b9b32337be4e95660677046476995544"
    );
}

#[test]
fn ed25519_pkcs8_der_private_key() {
    const S: &str = "302e020100300506032b657004220420feb858a4a69600a5eef2d9c76f7fb84fc0b6627f29e0ab17e160f640c267d404";

    let pk = PrivateKey::from_str_der(S).unwrap();

    assert_eq!(pk.algorithm().oid, ED25519_OID);
    assert_eq!(
        pk.to_string_raw(),
        "feb858a4a69600a5eef2d9c76f7fb84fc0b6627f29e0ab17e160f640c267d404"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "8ccd31b53d1835b467aac795dab19b274dd3b37e3daf12fcec6bc02bac87b53d"
    );
}

#[test]
fn ecdsa_pkcs8_private_key_der() {
    const S: &str = "3030020100300706052b8104000a042204208c2cdc9575fe67493443967d74958fd7808a3787fd3337e99cfeebbc7566b586";

    let pk = PrivateKey::from_str_der(S).unwrap();

    assert_eq!(pk.algorithm().oid, K256_OID);
    assert_eq!(
        pk.to_string_raw(),
        "8c2cdc9575fe67493443967d74958fd7808a3787fd3337e99cfeebbc7566b586"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "028173079d2e996ef6b2d064fc82d5fc7094367211e28422bec50a2f75c365f5fd"
    );
}

#[test]
fn ecdsa_ec_private_key_compressed_der() {
    const S: &str = "30540201010420ac318ea8ff8d991ab2f16172b4738e74dc35a56681199cfb1c0cb2e7cb560ffda00706052b8104000aa124032200036843f5cb338bbb4cdb21b0da4ea739d910951d6e8a5f703d313efe31afe788f4";

    let pk = PrivateKey::from_str_der(S).unwrap();

    assert_eq!(pk.algorithm().oid, K256_OID);
    assert_eq!(
        pk.to_string_raw(),
        "ac318ea8ff8d991ab2f16172b4738e74dc35a56681199cfb1c0cb2e7cb560ffd"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "036843f5cb338bbb4cdb21b0da4ea739d910951d6e8a5f703d313efe31afe788f4"
    );
}

#[test]
fn ecdsa_ec_private_key_uncompressed_der() {
    const S: &str = "307402010104208927647ad12b29646a1d051da8453462937bb2c813c6815cac6c0b720526ffc6a00706052b8104000aa14403420004aaac1c3ac1bea0245b8e00ce1e2018f9eab61b6331fbef7266f2287750a6597795f855ddcad2377e22259d1fcb4e0f1d35e8f2056300c15070bcbfce3759cc9d";

    let pk = PrivateKey::from_str_der(S).unwrap();

    assert_eq!(pk.algorithm().oid, K256_OID);
    assert_eq!(
        pk.to_string_raw(),
        "8927647ad12b29646a1d051da8453462937bb2c813c6815cac6c0b720526ffc6"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "03aaac1c3ac1bea0245b8e00ce1e2018f9eab61b6331fbef7266f2287750a65977"
    );
}

#[test]
fn ecdsa_ec_private_key_no_public_key_der() {
    const S: &str = "302e0201010420a6170a6aa6389a5bd3a3a8f9375f57bd91aa7f7d8b8b46ce0b702e000a21a5fea00706052b8104000a";

    let pk = PrivateKey::from_str_der(S).unwrap();

    assert_eq!(pk.algorithm().oid, K256_OID);
    assert_eq!(
        pk.to_string_raw(),
        "a6170a6aa6389a5bd3a3a8f9375f57bd91aa7f7d8b8b46ce0b702e000a21a5fe"
    );
    assert_eq!(
        pk.public_key().to_string_raw(),
        "03b69a75a5ddb1c0747e995d47555019e5d8a28003ab5202bd92f534361fb4ec8a"
    );
}
