use crate::{
    AccountId,
    PrivateKey,
    PublicKey,
    TransactionId,
};

#[derive(Debug, Clone)]
pub(crate) struct Operator {
    pub account_id: AccountId,
    pub signer: PrivateKey,
}

impl Operator {
    pub(crate) fn sign(&self, body_bytes: &[u8]) -> (PublicKey, Vec<u8>) {
        (self.signer.public_key(), self.signer.sign(body_bytes))
    }

    pub(crate) fn generate_transaction_id(&self) -> TransactionId {
        TransactionId::generate(self.account_id)
    }
}
