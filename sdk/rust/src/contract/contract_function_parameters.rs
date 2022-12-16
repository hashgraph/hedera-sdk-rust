use std::iter;
use std::str::FromStr;

use itertools::Itertools;
use pkcs8::der::Encode;

use crate::contract::contract_function_selector::ContractFunctionSelector;
use crate::evm_address::IdEvmAddress;

#[derive(Debug, Clone)]
pub struct ContractFunctionParameters {
    args: Vec<Argument>,
}

#[derive(Debug, Clone)]
struct Argument {
    type_name: &'static str,
    value_bytes: Vec<u8>,
    is_dynamic: bool,
}

impl ContractFunctionParameters {
    pub fn new() -> Self {
        ContractFunctionParameters { args: vec![] }
    }

    fn left_pad_32_bytes(bytes: &[u8], is_negative: bool) -> Vec<u8> {
        let pad_byte = if is_negative { 0xFF } else { 0x00 };
        iter::repeat(pad_byte).take(32 - bytes.len()).chain(bytes.iter().map(|b| *b)).collect()
    }

    fn right_pad_32_bytes(bytes: &[u8]) -> Vec<u8> {
        bytes.iter().map(|b| *b).chain(iter::repeat(0x00).take(32 - (bytes.len() % 32))).collect()
    }

    fn encode_address(address_string: &str) -> Vec<u8> {
        Self::left_pad_32_bytes(
            IdEvmAddress::from_str(address_string).unwrap().0 .0.as_slice(),
            false,
        )
    }

    fn encode_dynamic_bytes(bytes: &[u8]) -> Vec<u8> {
        let mut out_bytes = Self::left_pad_32_bytes(bytes.len().to_be_bytes().as_slice(), false);
        out_bytes.append(&mut Self::right_pad_32_bytes(bytes));
        out_bytes
    }

    fn encode_array_of_dynamic_byte_arrays<I>(elements: I, elements_len: usize) -> Vec<u8>
    where
        I: IntoIterator,
        I::Item: AsRef<[u8]>,
    {
        let head_len = (elements_len + 1) * 32;
        let mut out_bytes = Vec::with_capacity(head_len);
        out_bytes
            .append(&mut Self::left_pad_32_bytes(elements_len.to_be_bytes().as_slice(), false));
        let mut current_offset = elements_len * 32;
        let mut body_bytes: Vec<u8> = Vec::new();
        for element in elements {
            let element = element.as_ref();
            out_bytes.append(&mut Self::left_pad_32_bytes(
                current_offset.to_be_bytes().as_slice(),
                false,
            ));
            current_offset += element.len();
            body_bytes.extend(element)
        }
        out_bytes.extend(body_bytes);
        out_bytes
    }

    fn encode_array_of_32_byte_elements<I>(elements: I, elements_len: usize) -> Vec<u8>
    where
        I: IntoIterator<Item = Vec<u8>>,
    {
        let mut out_bytes = Self::left_pad_32_bytes(elements_len.to_be_bytes().as_slice(), false);
        out_bytes.reserve(out_bytes.len() + (elements_len * 32));
        out_bytes.extend(elements.into_iter().flatten());
        out_bytes
    }

    pub(crate) fn to_bytes(&self, func_name: Option<&str>) -> Vec<u8> {
        let mut current_dynamic_offset = self.args.len() * 32;
        let mut arg_bytes = Vec::new();
        let mut dynamic_arg_bytes = Vec::new();
        let mut function_selector = func_name.map(ContractFunctionSelector::new);
        for arg in &self.args {
            if let Some(selector) = &mut function_selector {
                selector.add_param_type(arg.type_name);
            }
            if arg.is_dynamic {
                arg_bytes.append(&mut Self::left_pad_32_bytes(
                    current_dynamic_offset.to_be_bytes().as_slice(),
                    false,
                ));
                dynamic_arg_bytes.extend_from_slice(arg.value_bytes.as_slice());
                current_dynamic_offset += arg.value_bytes.len();
            } else {
                arg_bytes.extend_from_slice(arg.value_bytes.as_slice());
            }
        }
        arg_bytes.append(&mut dynamic_arg_bytes);
        if let Some(selector) = &mut function_selector {
            let mut out_bytes = Vec::from(selector.finish());
            out_bytes.append(&mut arg_bytes);
            out_bytes
        } else {
            arg_bytes
        }
    }

    pub fn add_string(&mut self, val: &str) -> &mut Self {
        self.args.push(Argument {
            type_name: "string",
            value_bytes: Self::encode_dynamic_bytes(val.as_bytes()),
            is_dynamic: true,
        });
        self
    }

    pub fn add_string_array(&mut self, val: &[&str]) -> &mut Self {
        self.args.push(Argument {
            type_name: "string[]",
            value_bytes: Self::encode_array_of_dynamic_byte_arrays(
                val.into_iter().map(|s| s.as_bytes()),
                val.len(),
            ),
            is_dynamic: true,
        });
        self
    }

    pub fn add_bytes(&mut self, val: &[u8]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes",
            value_bytes: Self::encode_dynamic_bytes(val),
            is_dynamic: true,
        });
        self
    }

    pub fn add_bytes_array(&mut self, val: &[&[u8]]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes[]",
            value_bytes: Self::encode_array_of_dynamic_byte_arrays(val, val.len()),
            is_dynamic: false,
        });
        self
    }

    pub fn add_bytes32(&mut self, val: &[u8; 32]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes32",
            value_bytes: val.to_vec().unwrap(),
            is_dynamic: false,
        });
        self
    }

    pub fn add_bytes32_array(&mut self, val: &[[u8; 32]]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes32",
            value_bytes: Self::encode_array_of_32_byte_elements(
                val.into_iter().map(|b| b.to_vec().unwrap()),
                val.len(),
            ),
            is_dynamic: true,
        });
        self
    }

    pub fn add_bool(&mut self, val: bool) -> &mut Self {
        self.args.push(Argument {
            type_name: "bool",
            value_bytes: Self::left_pad_32_bytes(
                // a bool in rust is guaranteed to be of value 0 or 1
                (val as u32).to_be_bytes().as_slice(),
                false,
            ),
            is_dynamic: false,
        });
        self
    }

    // TODO: add_intN
    // TODO: add_intN_array
    // TODO: add_uintN
    // TODO: add_uintN_array

    pub fn add_address(&mut self, address_string: &str) -> &mut Self {
        self.args.push(Argument {
            type_name: "address",
            value_bytes: Self::encode_address(address_string),
            is_dynamic: false,
        });
        self
    }

    pub fn add_address_array(&mut self, address_strings: &[&str]) -> &mut Self {
        self.args.push(Argument {
            type_name: "address[]",
            value_bytes: Self::encode_array_of_32_byte_elements(
                address_strings.into_iter().map(|addr| Self::encode_address(addr)),
                address_strings.len(),
            ),
            is_dynamic: false,
        });
        self
    }

    pub fn add_function(
        &mut self,
        address_string: &str,
        mut selector: ContractFunctionSelector,
    ) -> &mut Self {
        let mut value_bytes =
            IdEvmAddress::from_str(address_string).unwrap().0 .0.to_vec().unwrap();
        value_bytes.extend(selector.finish());
        self.args.push(Argument {
            type_name: "function",
            value_bytes: Self::right_pad_32_bytes(value_bytes.as_slice()),
            is_dynamic: false,
        });
        self
    }
}
