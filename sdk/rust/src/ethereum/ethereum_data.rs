use rlp::Rlp;

use crate::Error;

/// Data for an [`EthereumTransaction`](crate::EthereumTransaction).
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum EthereumData {
    /// Data for a legacy ethereum transaction.
    Legacy(LegacyEthereumData),

    /// Data for an Eip 1559 ethereum transaction.
    Eip1559(Eip1559EthereumData),
}

impl EthereumData {
    pub(super) fn call_data_mut(&mut self) -> &mut Vec<u8> {
        match self {
            EthereumData::Legacy(it) => &mut it.call_data,
            EthereumData::Eip1559(it) => &mut it.call_data,
        }
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        let rlp = Rlp::new(bytes);

        // fixme: probably not basic_parse error
        let rlp_inner = rlp.iter().next().ok_or_else(|| Error::basic_parse("Invalid RLP"))?;
        match rlp_inner.is_list() {
            true => Ok(Self::Legacy(LegacyEthereumData::from_bytes(bytes)?)),
            false => Ok(Self::Eip1559(Eip1559EthereumData::from_bytes(bytes)?)),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        match self {
            EthereumData::Legacy(it) => it.to_bytes(),
            EthereumData::Eip1559(it) => it.to_bytes(),
        }
    }
}

/// Data for a legacy ethereum transaction.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct LegacyEthereumData {
    /// Transaction's nonce.
    pub nonce: Vec<u8>,

    /// Price for 1 gas.
    pub gas_price: Vec<u8>,

    /// The amount of gas available for the transaction.
    pub gas_limit: Vec<u8>,

    /// The receiver of the transaction.
    pub to: Vec<u8>,

    /// The transaction value.
    pub value: Vec<u8>,

    /// The V value of the signature.
    pub v: Vec<u8>,

    /// The raw call data.
    pub call_data: Vec<u8>,

    /// The R value of the signature.
    pub r: Vec<u8>,

    /// The S value of the signature.
    pub s: Vec<u8>,
}

impl LegacyEthereumData {
    fn decode_rlp(rlp: &Rlp) -> Result<Self, rlp::DecoderError> {
        if rlp.item_count()? != 9 {
            return Err(rlp::DecoderError::RlpIncorrectListLen);
        }

        Ok(Self {
            nonce: rlp.val_at(0)?,
            gas_price: rlp.val_at(1)?,
            gas_limit: rlp.val_at(2)?,
            to: rlp.val_at(3)?,
            value: rlp.val_at(4)?,
            call_data: rlp.val_at(5)?,
            v: rlp.val_at(6)?,
            r: rlp.val_at(7)?,
            s: rlp.val_at(8)?,
        })
    }

    /// Deserialize this data from rlp encoded bytes.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        // todo: test this.
        Self::decode_rlp(&Rlp::new(bytes)).map_err(Error::basic_parse)
    }

    /// Encode this data to rlp encoded bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        // todo: test this.
        let mut rlp = rlp::RlpStream::new_list(9);

        rlp.append(&self.nonce)
            .append(&self.gas_price)
            .append(&self.gas_limit)
            .append(&self.to)
            .append(&self.value)
            .append(&self.call_data)
            .append(&self.v)
            .append(&self.r)
            .append(&self.s);

        rlp.out().to_vec()
    }
}

/// Data for an Eip 1559 ethereum transaction.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct Eip1559EthereumData {
    /// ID of the chain.
    pub chain_id: Vec<u8>,

    /// Transaction's nonce.
    pub nonce: Vec<u8>,

    /// An 'optional' additional fee in Ethereum that is paid directly to miners in order to incentivize
    /// them to include your transaction in a block. Not used in Hedera.
    pub max_priority_gas: Vec<u8>,

    /// The maximum amount, in tinybars, that the payer of the hedera transaction
    /// is willing to pay to complete the transaction.
    pub max_gas: Vec<u8>,

    /// The amount of gas available for the transaction.
    pub gas_limit: Vec<u8>,

    /// The receiver of the transaction.
    pub to: Vec<u8>,

    /// The transaction value.
    pub value: Vec<u8>,

    /// The raw call data.
    pub call_data: Vec<u8>,

    /// Specifies an array of addresses and storage keys that the transaction plans to access.
    pub access_list: Vec<u8>,

    /// Recovery parameter used to ease the signature verification.
    pub recovery_id: Vec<u8>,

    /// The R value of the signature.
    pub r: Vec<u8>,

    /// The S value of the signature.
    pub s: Vec<u8>,
}

impl Eip1559EthereumData {
    fn decode_rlp(rlp: &Rlp) -> Result<Self, rlp::DecoderError> {
        if rlp.item_count()? != 2 {
            return Err(rlp::DecoderError::RlpIncorrectListLen);
        }

        let kind: u8 = rlp.val_at(0)?;

        if kind != 2 {
            return Err(rlp::DecoderError::Custom("Invalid kind"));
        }

        let rlp = rlp.at(1)?;

        if rlp.item_count()? != 12 {
            return Err(rlp::DecoderError::RlpIncorrectListLen);
        }

        Ok(Self {
            chain_id: rlp.val_at(0)?,
            nonce: rlp.val_at(1)?,
            max_priority_gas: rlp.val_at(2)?,
            max_gas: rlp.val_at(3)?,
            gas_limit: rlp.val_at(4)?,
            to: rlp.val_at(5)?,
            value: rlp.val_at(6)?,
            call_data: rlp.val_at(7)?,
            access_list: rlp.val_at(8)?,
            recovery_id: rlp.val_at(9)?,
            r: rlp.val_at(10)?,
            s: rlp.val_at(11)?,
        })
    }

    /// Deserialize this data from rlp encoded bytes.
    pub fn from_bytes(bytes: &[u8]) -> crate::Result<Self> {
        // todo: test this.
        Self::decode_rlp(&Rlp::new(bytes)).map_err(Error::basic_parse)
    }

    /// Encode this data to rlp encoded bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        // todo: test this.
        let mut rlp = rlp::RlpStream::new_list(2);

        rlp.append(&2_u8)
            .begin_list(12)
            .append(&self.chain_id)
            .append(&self.nonce)
            .append(&self.max_priority_gas)
            .append(&self.max_gas)
            .append(&self.gas_limit)
            .append(&self.to)
            .append(&self.value)
            .append(&self.call_data)
            .append(&self.access_list)
            .append(&self.recovery_id)
            .append(&self.r)
            .append(&self.s);

        rlp.out().to_vec()
    }
}
