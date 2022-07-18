use hedera_proto::services;

use crate::{
    ContractId,
    Error,
    FromProtobuf,
    PublicKey,
    ToProtobuf,
};

/// Any method that can be used to authorize an operation on Hedera.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Key {
    Single(PublicKey),
    ContractId(ContractId),
    DelegatableContractId(ContractId),
    // TODO: KeyList
    // TODO: ThresholdKey
}

impl ToProtobuf for Key {
    type Protobuf = services::Key;

    fn to_protobuf(&self) -> Self::Protobuf {
        use services::key::Key::*;

        services::Key {
            key: Some(match self {
                Self::Single(key) => {
                    let bytes = key.to_bytes_raw();

                    if key.is_ed25519() {
                        Ed25519(bytes)
                    } else {
                        EcdsaSecp256k1(bytes)
                    }
                }

                Self::ContractId(id) => ContractId(id.to_protobuf()),
                Self::DelegatableContractId(id) => DelegatableContractId(id.to_protobuf()),
            }),
        }
    }
}

impl From<PublicKey> for Key {
    fn from(key: PublicKey) -> Self {
        Self::Single(key)
    }
}

impl From<ContractId> for Key {
    fn from(id: ContractId) -> Self {
        Self::ContractId(id)
    }
}

impl FromProtobuf<services::Key> for Key {
    fn from_protobuf(pb: services::Key) -> crate::Result<Self>
    where
        Self: Sized,
    {
        use services::key::Key::*;

        match pb.key {
            Some(Ed25519(bytes)) => Ok(Self::Single(PublicKey::from_bytes_ed25519(&bytes)?)),
            Some(ContractId(id)) => Ok(Self::ContractId(crate::ContractId::from_protobuf(id)?)),
            Some(DelegatableContractId(id)) => {
                Ok(Self::DelegatableContractId(crate::ContractId::from_protobuf(id)?))
            }
            Some(Rsa3072(_)) => {
                Err(Error::from_protobuf("unexpected unsupported RSA-3072 key in Key"))
            }
            Some(Ecdsa384(_)) => {
                Err(Error::from_protobuf("unexpected unsupported ECDSA-384 key in Key"))
            }
            Some(ThresholdKey(_)) => todo!(),
            Some(KeyList(_)) => todo!(),
            Some(EcdsaSecp256k1(bytes)) => {
                Ok(Self::Single(PublicKey::from_bytes_ecdsa_secp256k1(&bytes)?))
            }
            None => Err(Error::from_protobuf("unexpected empty key in Key")),
        }
    }
}
