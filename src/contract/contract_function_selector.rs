use sha3::{
    Digest,
    Keccak256,
};

/// Builder class for Solidity function selectors.
#[derive(Debug, Clone)]
pub struct ContractFunctionSelector(ContractFunctionSelectorState);

// this isn't intended for storage on the heap, so, boxing the hasher is... futile.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone)]
enum ContractFunctionSelectorState {
    Building { digest: sha3::Keccak256, needs_comma: bool },
    Finished([u8; 4]),
}

use ContractFunctionSelectorState::{
    Building,
    Finished,
};

impl From<[u8; 4]> for ContractFunctionSelector {
    fn from(value: [u8; 4]) -> Self {
        Self(Finished(value))
    }
}

impl ContractFunctionSelector {
    pub fn new(func_name: &str) -> Self {
        let mut digest = <Keccak256 as Digest>::new_with_prefix(func_name.as_bytes());
        digest.update(b"(");
        Self(Building { digest, needs_comma: false })
    }

    pub(crate) fn add_param_type(&mut self, param_type_name: &str) -> &mut Self {
        if let Building { digest, needs_comma } = &mut self.0 {
            if *needs_comma {
                digest.update(b",");
            }
            digest.update(param_type_name.as_bytes());
            *needs_comma = true;
        } else {
            panic!("Cannot add param type to finished ContractFunctionSelector")
        }
        self
    }

    pub fn finish(&mut self) -> [u8; 4] {
        match &mut self.0 {
            Building { digest, .. } => {
                digest.update(b")");
                let finished_bytes = digest.clone().finalize()[0..4].try_into().unwrap();
                self.0 = Finished(finished_bytes);
                finished_bytes
            }
            Finished(finished_bytes) => *finished_bytes,
        }
    }

    pub fn add_string(&mut self) -> &mut Self {
        self.add_param_type("string")
    }

    pub fn add_string_array(&mut self) -> &mut Self {
        self.add_param_type("string[]")
    }

    pub fn add_bytes(&mut self) -> &mut Self {
        self.add_param_type("bytes")
    }

    pub fn add_bytes_array(&mut self) -> &mut Self {
        self.add_param_type("bytes[]")
    }

    pub fn add_bytes32(&mut self) -> &mut Self {
        self.add_param_type("bytes32")
    }

    pub fn add_bytes32_array(&mut self) -> &mut Self {
        self.add_param_type("bytes32[]")
    }

    pub fn add_bool(&mut self) -> &mut Self {
        self.add_param_type("bool")
    }

    pub fn add_int8(&mut self) -> &mut Self {
        self.add_param_type("int8")
    }

    pub fn add_int16(&mut self) -> &mut Self {
        self.add_param_type("int16")
    }

    pub fn add_int32(&mut self) -> &mut Self {
        self.add_param_type("int32")
    }

    pub fn add_int64(&mut self) -> &mut Self {
        self.add_param_type("int64")
    }

    pub fn add_int256(&mut self) -> &mut Self {
        self.add_param_type("int256")
    }

    pub fn add_uint8(&mut self) -> &mut Self {
        self.add_param_type("uint8")
    }

    pub fn add_uint16(&mut self) -> &mut Self {
        self.add_param_type("uint16")
    }

    pub fn add_uint32(&mut self) -> &mut Self {
        self.add_param_type("uint32")
    }

    pub fn add_uint64(&mut self) -> &mut Self {
        self.add_param_type("uint64")
    }

    pub fn add_uint256(&mut self) -> &mut Self {
        self.add_param_type("uint256")
    }

    pub fn add_int8_array(&mut self) -> &mut Self {
        self.add_param_type("int8[]")
    }

    pub fn add_int16_array(&mut self) -> &mut Self {
        self.add_param_type("int16[]")
    }

    pub fn add_int32_array(&mut self) -> &mut Self {
        self.add_param_type("int32[]")
    }

    pub fn add_int64_array(&mut self) -> &mut Self {
        self.add_param_type("int64[]")
    }

    pub fn add_int256_array(&mut self) -> &mut Self {
        self.add_param_type("int256[]")
    }

    pub fn add_uint8_array(&mut self) -> &mut Self {
        self.add_param_type("uint8[]")
    }

    pub fn add_uint16_array(&mut self) -> &mut Self {
        self.add_param_type("uint16[]")
    }

    pub fn add_uint32_array(&mut self) -> &mut Self {
        self.add_param_type("uint32[]")
    }

    pub fn add_uint64_array(&mut self) -> &mut Self {
        self.add_param_type("uint64[]")
    }

    pub fn add_uint256_array(&mut self) -> &mut Self {
        self.add_param_type("uint256[]")
    }

    pub fn add_address(&mut self) -> &mut Self {
        self.add_param_type("address")
    }

    pub fn add_address_array(&mut self) -> &mut Self {
        self.add_param_type("address[]")
    }

    pub fn add_function(&mut self) -> &mut Self {
        self.add_param_type("function")
    }
}
