use hedera_proto::services;

use crate::{Error, FromProtobuf, PublicKey};

#[derive(Debug, Clone)]
pub enum Key {
    Primitive(PublicKey),
    // TODO: ContractId(ContractId),
    // TODO: DelegatableContractId(ContractId),
    // TODO: Rsa3072
    // TODO: Ecdsa384
    // TODO: EcdsaSecp256k1
    // TODO: KeyList
    // TODO: ThresholdKey
}

impl FromProtobuf for Key {
    type Protobuf = services::Key;

    fn from_protobuf(pb: Self::Protobuf) -> crate::Result<Self>
    where
        Self: Sized,
    {
        use services::key::Key::*;

        match pb.key {
            Some(Ed25519(bytes)) => Ok(Self::Primitive(PublicKey::from_bytes_ed25519(&bytes)?)),
            Some(ContractId(_)) => todo!(),
            Some(Rsa3072(_)) => {
                Err(Error::from_protobuf("unexpected unsupported RSA-3072 key in Key"))
            }
            Some(Ecdsa384(_)) => {
                Err(Error::from_protobuf("unexpected unsupported ECDSA-384 key in Key"))
            }
            Some(ThresholdKey(_)) => todo!(),
            Some(KeyList(_)) => todo!(),
            Some(EcdsaSecp256k1(bytes)) => {
                Ok(Self::Primitive(PublicKey::from_bytes_ecdsa_secp256k1(&bytes)?))
            }
            Some(DelegatableContractId(_)) => todo!(),
            None => Err(Error::from_protobuf("unexpected empty key in Key")),
        }
    }
}
