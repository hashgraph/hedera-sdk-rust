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
            Some(Ed25519(bytes)) => Ok(Self::Primitive(PublicKey::from_bytes_raw_ed25519(&bytes)?)),
            Some(ContractId(_)) => todo!(),
            Some(Rsa3072(_)) => todo!(),
            Some(Ecdsa384(_)) => todo!(),
            Some(ThresholdKey(_)) => todo!(),
            Some(KeyList(_)) => todo!(),
            Some(EcdsaSecp256k1(_)) => todo!(),
            Some(DelegatableContractId(_)) => todo!(),

            None => {
                return Err(Error::from_protobuf("unexpected empty key in Key"));
            }
        }
    }
}
