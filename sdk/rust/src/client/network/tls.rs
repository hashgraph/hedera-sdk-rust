use std::time::SystemTime;

use der::{
    Decode,
    Encode,
};
use p384::ecdsa;
use pkcs8::{
    AssociatedOid,
    ObjectIdentifier,
    SubjectPublicKeyInfo,
};
use sha2::digest::Digest;
use signature::DigestVerifier;
use tokio_rustls::rustls;

use super::{
    AddressBook,
    NodeEndpointAddress,
};
use crate::AccountId;

/// A Per-endpoint certificate verifier.
///
/// This ensures that:
/// - The `address` is in the listing for the given node account ID (ie).
/// - The node ID matches the expected value in the address book.
/// - The `address` matches the server name
/// - The hash of the certificate is exactly as expected
/// - The public key matches the expected public key (yes, this is redundant with the previous step, doing it anyway.)
/// - The algorithm is *exactly* OID `1.2.840.10045.4.3.3`
/// - The public key's algorithm is EcdsaSecp256r1 (not k1, r1)
/// - The signature verifies correctly.
/// - The certificate is not expired, and has entered its valid state.
/// - The certificate is self-signed
pub(super) struct HederaTlsCertVerfier {
    pub(super) node_address_book: arc_swap::ArcSwap<AddressBook>,
    pub(super) account_id: AccountId,
    // (almost?) always equal to the node account ID.
    pub(super) node_id: u64,
    pub(super) address: (NodeEndpointAddress, u16),
}

#[derive(Debug)]
enum AnyAlgorithm {
    Sha384WithEcdsa,
}

#[derive(Debug)]
enum AnyKey {
    EcdsaP384(p384::ecdsa::VerifyingKey),
}

fn parse_key(spki: &SubjectPublicKeyInfo<'_>) -> Result<AnyKey, rustls::Error> {
    match spki.algorithm.oids().map_err(|_| rustls::Error::InvalidCertificateSignatureType)? {
        (elliptic_curve::ALGORITHM_OID, Some(parameters_oid)) => match parameters_oid {
            p384::NistP384::OID => Ok(AnyKey::EcdsaP384(
                p384::PublicKey::from_sec1_bytes(spki.subject_public_key)
                    .map_err(|_| {
                        rustls::Error::InvalidCertificateData("Invalid public key".to_owned())
                    })?
                    .into(),
            )),

            _ => Err(rustls::Error::InvalidCertificateSignatureType),
        },
        _ => Err(rustls::Error::InvalidCertificateSignatureType),
    }
}

fn verify_signature(
    key: AnyKey,
    algorithm: AnyAlgorithm,
    signature: &[u8],
    message: &[u8],
) -> Result<(), rustls::Error> {
    match (key, algorithm) {
        (AnyKey::EcdsaP384(verifier), AnyAlgorithm::Sha384WithEcdsa) => {
            let signature: p384::ecdsa::Signature = p384::ecdsa::DerSignature::try_from(signature)
                .map_err(|_| rustls::Error::InvalidCertificateSignature)?
                .try_into()
                .map_err(|_| rustls::Error::InvalidCertificateSignature)?;
            verifier
                .verify_digest(sha2::Sha384::new_with_prefix(message), &signature)
                .map_err(|_| rustls::Error::InvalidCertificateSignature)
        }
    }
}

// to be absolutely sure we check every field always
struct Certificate<State> {
    state: State,
}

impl<'a> Certificate<cert_state::Unverified<'a>> {
    fn verify_signature(
        self,
    ) -> Result<Certificate<cert_state::SignatureMatches<'a>>, rustls::Error> {
        const ECDSA_WITH_SHA384: ObjectIdentifier =
            ObjectIdentifier::new_unwrap("1.2.840.10045.4.3.3");
        if self.state.cert.signature_algorithm != self.state.cert.tbs_certificate.signature {
            return Err(rustls::Error::InvalidCertificateSignatureType);
        }

        let oids = self
            .state
            .cert
            .signature_algorithm
            .oids()
            .map_err(|_| rustls::Error::InvalidCertificateSignatureType)?;

        // fixme: I'm not sure this is how this is supposed to go.

        let sig_algorithm = match oids {
            (ECDSA_WITH_SHA384, None) => AnyAlgorithm::Sha384WithEcdsa,
            _ => return Err(rustls::Error::InvalidCertificateSignatureType),
        };

        let pk = parse_key(&self.state.cert.tbs_certificate.subject_public_key_info)?;

        // fixme: no unwrap
        verify_signature(
            pk,
            sig_algorithm,
            &self.state.cert.signature.as_bytes().unwrap(),
            &self.state.raw_tbs,
        )?;

        Ok(Certificate {
            state: cert_state::SignatureMatches { tbs_cert: self.state.cert.tbs_certificate },
        })
    }
}

mod cert_state {
    use der::asn1::BitStringRef;
    use pkcs8::AlgorithmIdentifier;

    pub(super) struct Unverified<'a> {
        pub(super) raw_tbs: &'a [u8],
        pub(super) cert: x509_cert::Certificate<'a>,
    }

    pub(super) struct SignatureMatches<'a> {
        pub(super) tbs_cert: x509_cert::TbsCertificate<'a>,
    }

    // pub(super) struct Valid<'a> {
    //     pub(super) signature_algorithm: AlgorithmIdentifier<'a>,
    // }
}

impl rustls::client::ServerCertVerifier for HederaTlsCertVerfier {
    fn verify_server_cert(
        &self,
        end_entity: &rustls::Certificate,
        intermediates: &[rustls::Certificate],
        server_name: &rustls::ServerName,
        scts: &mut dyn Iterator<Item = &[u8]>,
        ocsp_response: &[u8],
        now: SystemTime,
    ) -> Result<rustls::client::ServerCertVerified, rustls::Error> {
        if !dbg!(intermediates).is_empty() {
            // what error goes here?
            // anyway, I don't think we handle intermediates because how could we?
            return Err(rustls::Error::General("???".to_owned()));
        }

        {
            let cert = x509_cert::Certificate::from_der(&end_entity.0)
                .map_err(|_| rustls::Error::InvalidCertificateEncoding)?;

            let digest = {
                let pem = pem_rfc7468::encode_string(
                    "CERTIFICATE",
                    pem_rfc7468::LineEnding::LF,
                    &end_entity.0,
                )
                .unwrap();

                sha2::Sha384::digest(dbg!(&pem))
            };

            dbg!(hex::encode(&digest));

            let tbs_bytes = {
                let mut buf = Vec::new();
                cert.tbs_certificate.encode_to_vec(&mut buf).unwrap();
                buf
            };

            let cert = Certificate { state: cert_state::Unverified { raw_tbs: &tbs_bytes, cert } };

            let cert = cert.verify_signature()?;
        }

        dbg!(hex::encode(&end_entity.0));
        dbg!(server_name);

        // end_cert.

        // todo: at least part of the below link is probably applicable.
        // https://docs.rs/rustls/latest/src/rustls/verify.rs.html#337-377
        // let cert = x509_cert::Certificate::from_der(&end_entity.0)
        //     .map_err(|_| tokio_rustls::rustls::Error::InvalidCertificateEncoding)?;

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use der::Decode;
    use pkcs8::SubjectPublicKeyInfo;

    #[test]
    fn parse_ecdsa_p384_key() {
        const DER_BYTES: [u8; 120] = hex_literal::hex!(
            "3076301006072a8648ce3d020106052b8104002203620004f0d84318afdd816b"
            "c3447109f1a7fb0810dd86fba31f95c8acb7a7df71932d8f036c9c095b68ddb0"
            "b1b1dfe0d155599e35067109b7068f47b6232ca9906eacd26b6799eca60fc988"
            "dc953c9b56998ae229b59487dc1c3504d9f684a538c78d9c"
        );

        let key = super::parse_key(&SubjectPublicKeyInfo::from_der(&DER_BYTES).unwrap()).unwrap();

        let key = assert_matches!(key, super::AnyKey::EcdsaP384(key) => key);

        todo!("how tf do I test this");
    }
}
