use std::cmp::max;
use std::iter;
use std::str::FromStr;

use num_bigint::{
    BigInt,
    BigUint,
    Sign,
};
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

trait IntEncode {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>);
}

impl IntEncode for u8 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_be_bytes().to_vec().unwrap())
    }
}

impl IntEncode for i8 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.is_negative(), self.to_be_bytes().to_vec().unwrap())
    }
}

impl IntEncode for u16 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_be_bytes().to_vec().unwrap())
    }
}

impl IntEncode for i16 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.is_negative(), self.to_be_bytes().to_vec().unwrap())
    }
}

impl IntEncode for u32 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_be_bytes().to_vec().unwrap())
    }
}

impl IntEncode for i32 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.is_negative(), self.to_be_bytes().to_vec().unwrap())
    }
}

impl IntEncode for u64 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_be_bytes().to_vec().unwrap())
    }
}

impl IntEncode for i64 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.is_negative(), self.to_be_bytes().to_vec().unwrap())
    }
}

impl IntEncode for u128 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_be_bytes().to_vec().unwrap())
    }
}

impl IntEncode for i128 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.is_negative(), self.to_be_bytes().to_vec().unwrap())
    }
}

impl IntEncode for BigUint {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_bytes_be())
    }
}

impl IntEncode for BigInt {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        let (sign, bytes) = self.to_bytes_be();
        (sign == Sign::Minus, bytes)
    }
}

impl ContractFunctionParameters {
    /// Create a new, empty `ContractFunctionParameters`
    #[allow(dead_code)]
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

    /// Add a `string` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_string(&mut self, val: &str) -> &mut Self {
        self.args.push(Argument {
            type_name: "string",
            value_bytes: Self::encode_dynamic_bytes(val.as_bytes()),
            is_dynamic: true,
        });
        self
    }

    /// Add a `string[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
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

    /// Add a `bytes` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_bytes(&mut self, val: &[u8]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes",
            value_bytes: Self::encode_dynamic_bytes(val),
            is_dynamic: true,
        });
        self
    }

    /// Add a `bytes[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_bytes_array(&mut self, val: &[&[u8]]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes[]",
            value_bytes: Self::encode_array_of_dynamic_byte_arrays(val, val.len()),
            is_dynamic: false,
        });
        self
    }

    /// Add a `bytes32` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_bytes32(&mut self, val: &[u8; 32]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes32",
            value_bytes: val.to_vec().unwrap(),
            is_dynamic: false,
        });
        self
    }

    /// Add a `bytes32[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
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

    /// Add a `bool` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
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

    fn truncate_and_left_pad_32_bytes<T>(val: &T, byte_count: isize) -> Vec<u8>
    where
        T: IntEncode,
    {
        let (is_negative, full_value_bytes) = val.get_is_negative_and_be_bytes();
        let truncated_value_bytes =
            &full_value_bytes[max((full_value_bytes.len() as isize) - byte_count, 0) as usize..];
        Self::left_pad_32_bytes(truncated_value_bytes, is_negative)
    }

    fn add_int<T>(&mut self, val: &T, type_name: &'static str, byte_count: isize) -> &mut Self
    where
        T: IntEncode,
    {
        self.args.push(Argument {
            type_name,
            value_bytes: Self::truncate_and_left_pad_32_bytes(val, byte_count),
            is_dynamic: false,
        });
        self
    }

    fn add_int_array<T>(
        &mut self,
        val_array: &[T],
        type_name: &'static str,
        byte_count: isize,
    ) -> &mut Self
    where
        T: IntEncode,
    {
        self.args.push(Argument {
            type_name,
            value_bytes: Self::encode_array_of_32_byte_elements(
                val_array
                    .into_iter()
                    .map(|val| Self::truncate_and_left_pad_32_bytes(val, byte_count)),
                val_array.len(),
            ),
            is_dynamic: true,
        });
        self
    }

    /// Add an `int8` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int8(&mut self, val: &i8) -> &mut Self {
        self.add_int(val, "int8", 1)
    }

    /// Add an `int16` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int16(&mut self, val: &i16) -> &mut Self {
        self.add_int(val, "int16", 2)
    }

    /// Add an `int24` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int24(&mut self, val: &i32) -> &mut Self {
        self.add_int(val, "int24", 3)
    }

    /// Add an `int32` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int32(&mut self, val: &i32) -> &mut Self {
        self.add_int(val, "int32", 4)
    }

    /// Add an `int40` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int40(&mut self, val: &i64) -> &mut Self {
        self.add_int(val, "int40", 5)
    }

    /// Add an `int48` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int48(&mut self, val: &i64) -> &mut Self {
        self.add_int(val, "int48", 6)
    }

    /// Add an `int56` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int56(&mut self, val: &i64) -> &mut Self {
        self.add_int(val, "int56", 7)
    }

    /// Add an `int64` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int64(&mut self, val: &i64) -> &mut Self {
        self.add_int(val, "int64", 8)
    }

    /// Add an `int72` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int72(&mut self, val: &i128) -> &mut Self {
        self.add_int(val, "int72", 9)
    }

    /// Add an `int80` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int80(&mut self, val: &i128) -> &mut Self {
        self.add_int(val, "int80", 10)
    }

    /// Add an `int88` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int88(&mut self, val: &i128) -> &mut Self {
        self.add_int(val, "int88", 11)
    }

    /// Add an `int96` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int96(&mut self, val: &i128) -> &mut Self {
        self.add_int(val, "int96", 12)
    }

    /// Add an `int104` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int104(&mut self, val: &i128) -> &mut Self {
        self.add_int(val, "int104", 13)
    }

    /// Add an `int112` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int112(&mut self, val: &i128) -> &mut Self {
        self.add_int(val, "int112", 14)
    }

    /// Add an `int120` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int120(&mut self, val: &i128) -> &mut Self {
        self.add_int(val, "int120", 15)
    }

    /// Add an `int128` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int128(&mut self, val: &i128) -> &mut Self {
        self.add_int(val, "int128", 16)
    }

    /// Add an `int136` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int136(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int136", 17)
    }

    /// Add an `int144` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int144(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int144", 18)
    }

    /// Add an `int152` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int152(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int152", 19)
    }

    /// Add an `int160` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int160(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int160", 20)
    }

    /// Add an `int168` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int168(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int168", 21)
    }

    /// Add an `int176` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int176(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int176", 22)
    }

    /// Add an `int184` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int184(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int184", 23)
    }

    /// Add an `int192` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int192(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int192", 24)
    }

    /// Add an `int200` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int200(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int200", 25)
    }

    /// Add an `int208` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int208(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int208", 26)
    }

    /// Add an `int216` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int216(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int216", 27)
    }

    /// Add an `int224` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int224(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int224", 28)
    }

    /// Add an `int232` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int232(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int232", 29)
    }

    /// Add an `int240` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int240(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int240", 30)
    }

    /// Add an `int248` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int248(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int248", 31)
    }

    /// Add an `int256` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int256(&mut self, val: &BigInt) -> &mut Self {
        self.add_int(val, "int256", 32)
    }

    /// Add an `int8[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int8_array(&mut self, val_array: &[i8]) -> &mut Self {
        self.add_int_array(val_array, "int8[]", 1)
    }

    /// Add an `int16[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int16_array(&mut self, val_array: &[i16]) -> &mut Self {
        self.add_int_array(val_array, "int16[]", 2)
    }

    /// Add an `int24[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int24_array(&mut self, val_array: &[i32]) -> &mut Self {
        self.add_int_array(val_array, "int24[]", 3)
    }

    /// Add an `int32[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int32_array(&mut self, val_array: &[i32]) -> &mut Self {
        self.add_int_array(val_array, "int32[]", 4)
    }

    /// Add an `int40[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int40_array(&mut self, val_array: &[i64]) -> &mut Self {
        self.add_int_array(val_array, "int40[]", 5)
    }

    /// Add an `int48[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int48_array(&mut self, val_array: &[i64]) -> &mut Self {
        self.add_int_array(val_array, "int48[]", 6)
    }

    /// Add an `int56[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int56_array(&mut self, val_array: &[i64]) -> &mut Self {
        self.add_int_array(val_array, "int56[]", 7)
    }

    /// Add an `int64[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int64_array(&mut self, val_array: &[i64]) -> &mut Self {
        self.add_int_array(val_array, "int64[]", 8)
    }

    /// Add an `int72[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int72_array(&mut self, val_array: &[i128]) -> &mut Self {
        self.add_int_array(val_array, "int72[]", 9)
    }

    /// Add an `int80[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int80_array(&mut self, val_array: &[i128]) -> &mut Self {
        self.add_int_array(val_array, "int80[]", 10)
    }

    /// Add an `int88[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int88_array(&mut self, val_array: &[i128]) -> &mut Self {
        self.add_int_array(val_array, "int88[]", 11)
    }

    /// Add an `int96[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int96_array(&mut self, val_array: &[i128]) -> &mut Self {
        self.add_int_array(val_array, "int96[]", 12)
    }

    /// Add an `int104[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int104_array(&mut self, val_array: &[i128]) -> &mut Self {
        self.add_int_array(val_array, "int104[]", 13)
    }

    /// Add an `int112[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int112_array(&mut self, val_array: &[i128]) -> &mut Self {
        self.add_int_array(val_array, "int112[]", 14)
    }

    /// Add an `int120[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int120_array(&mut self, val_array: &[i128]) -> &mut Self {
        self.add_int_array(val_array, "int120[]", 15)
    }

    /// Add an `int128[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int128_array(&mut self, val_array: &[i128]) -> &mut Self {
        self.add_int_array(val_array, "int128[]", 16)
    }

    /// Add an `int136[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int136_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int136[]", 17)
    }

    /// Add an `int144[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int144_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int144[]", 18)
    }

    /// Add an `int152[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int152_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int152[]", 19)
    }

    /// Add an `int160[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int160_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int160[]", 20)
    }

    /// Add an `int168[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int168_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int168[]", 21)
    }

    /// Add an `int176[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int176_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int176[]", 22)
    }

    /// Add an `int184[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int184_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int184[]", 23)
    }

    /// Add an `int192[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int192_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int192[]", 24)
    }

    /// Add an `int200[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int200_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int200[]", 25)
    }

    /// Add an `int208[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int208_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int208[]", 26)
    }

    /// Add an `int216[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int216_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int216[]", 27)
    }

    /// Add an `int224[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int224_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int224[]", 28)
    }

    /// Add an `int232[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int232_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int232[]", 29)
    }

    /// Add an `int240[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int240_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int240[]", 30)
    }

    /// Add an `int248[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int248_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int248[]", 31)
    }

    /// Add an `int256[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_int256_array(&mut self, val_array: &[BigInt]) -> &mut Self {
        self.add_int_array(val_array, "int256[]", 32)
    }

    /// Add a `uint8` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint8(&mut self, val: &u8) -> &mut Self {
        self.add_int(val, "uint8", 1)
    }

    /// Add a `uint16` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint16(&mut self, val: &u16) -> &mut Self {
        self.add_int(val, "uint16", 2)
    }

    /// Add a `uint24` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint24(&mut self, val: &u32) -> &mut Self {
        self.add_int(val, "uint24", 3)
    }

    /// Add a `uint32` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint32(&mut self, val: &u32) -> &mut Self {
        self.add_int(val, "uint32", 4)
    }

    /// Add a `uint40` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint40(&mut self, val: &u64) -> &mut Self {
        self.add_int(val, "uint40", 5)
    }

    /// Add a `uint48` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint48(&mut self, val: &u64) -> &mut Self {
        self.add_int(val, "uint48", 6)
    }

    /// Add a `uint56` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint56(&mut self, val: &u64) -> &mut Self {
        self.add_int(val, "uint56", 7)
    }

    /// Add a `uint64` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint64(&mut self, val: &u64) -> &mut Self {
        self.add_int(val, "uint64", 8)
    }

    /// Add a `uint72` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint72(&mut self, val: &u128) -> &mut Self {
        self.add_int(val, "uint72", 9)
    }

    /// Add a `uint80` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint80(&mut self, val: &u128) -> &mut Self {
        self.add_int(val, "uint80", 10)
    }

    /// Add a `uint88` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint88(&mut self, val: &u128) -> &mut Self {
        self.add_int(val, "uint88", 11)
    }

    /// Add a `uint96` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint96(&mut self, val: &u128) -> &mut Self {
        self.add_int(val, "uint96", 12)
    }

    /// Add a `uint104` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint104(&mut self, val: &u128) -> &mut Self {
        self.add_int(val, "uint104", 13)
    }

    /// Add a `uint112` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint112(&mut self, val: &u128) -> &mut Self {
        self.add_int(val, "uint112", 14)
    }

    /// Add a `uint120` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint120(&mut self, val: &u128) -> &mut Self {
        self.add_int(val, "uint120", 15)
    }

    /// Add a `uint128` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint128(&mut self, val: &u128) -> &mut Self {
        self.add_int(val, "uint128", 16)
    }

    /// Add a `uint136` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint136(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint136", 17)
    }

    /// Add a `uint144` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint144(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint144", 18)
    }

    /// Add a `uint152` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint152(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint152", 19)
    }

    /// Add a `uint160` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint160(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint160", 20)
    }

    /// Add a `uint168` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint168(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint168", 21)
    }

    /// Add a `uint176` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint176(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint176", 22)
    }

    /// Add a `uint184` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint184(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint184", 23)
    }

    /// Add a `uint192` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint192(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint192", 24)
    }

    /// Add a `uint200` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint200(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint200", 25)
    }

    /// Add a `uint208` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint208(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint208", 26)
    }

    /// Add a `uint216` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint216(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint216", 27)
    }

    /// Add a `uint224` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint224(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint224", 28)
    }

    /// Add a `uint232` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint232(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint232", 29)
    }

    /// Add a `uint240` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint240(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint240", 30)
    }

    /// Add a `uint248` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint248(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint248", 31)
    }

    /// Add a `uint256` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint256(&mut self, val: &BigUint) -> &mut Self {
        self.add_int(val, "uint256", 32)
    }

    /// Add a `uint8[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint8_array(&mut self, val_array: &[u8]) -> &mut Self {
        self.add_int_array(val_array, "uint8[]", 1)
    }

    /// Add a `uint16[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint16_array(&mut self, val_array: &[u16]) -> &mut Self {
        self.add_int_array(val_array, "uint16[]", 2)
    }

    /// Add a `uint24[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint24_array(&mut self, val_array: &[u32]) -> &mut Self {
        self.add_int_array(val_array, "uint24[]", 3)
    }

    /// Add a `uint32[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint32_array(&mut self, val_array: &[u32]) -> &mut Self {
        self.add_int_array(val_array, "uint32[]", 4)
    }

    /// Add a `uint40[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint40_array(&mut self, val_array: &[u64]) -> &mut Self {
        self.add_int_array(val_array, "uint40[]", 5)
    }

    /// Add a `uint48[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint48_array(&mut self, val_array: &[u64]) -> &mut Self {
        self.add_int_array(val_array, "uint48[]", 6)
    }

    /// Add a `uint56[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint56_array(&mut self, val_array: &[u64]) -> &mut Self {
        self.add_int_array(val_array, "uint56[]", 7)
    }

    /// Add a `uint64[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint64_array(&mut self, val_array: &[u64]) -> &mut Self {
        self.add_int_array(val_array, "uint64[]", 8)
    }

    /// Add a `uint72[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint72_array(&mut self, val_array: &[u128]) -> &mut Self {
        self.add_int_array(val_array, "uint72[]", 9)
    }

    /// Add a `uint80[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint80_array(&mut self, val_array: &[u128]) -> &mut Self {
        self.add_int_array(val_array, "uint80[]", 10)
    }

    /// Add a `uint88[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint88_array(&mut self, val_array: &[u128]) -> &mut Self {
        self.add_int_array(val_array, "uint88[]", 11)
    }

    /// Add a `uint96[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint96_array(&mut self, val_array: &[u128]) -> &mut Self {
        self.add_int_array(val_array, "uint96[]", 12)
    }

    /// Add a `uint104[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint104_array(&mut self, val_array: &[u128]) -> &mut Self {
        self.add_int_array(val_array, "uint104[]", 13)
    }

    /// Add a `uint112[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint112_array(&mut self, val_array: &[u128]) -> &mut Self {
        self.add_int_array(val_array, "uint112[]", 14)
    }

    /// Add a `uint120[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint120_array(&mut self, val_array: &[u128]) -> &mut Self {
        self.add_int_array(val_array, "uint120[]", 15)
    }

    /// Add a `uint128[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint128_array(&mut self, val_array: &[u128]) -> &mut Self {
        self.add_int_array(val_array, "uint128[]", 16)
    }

    /// Add a `uint136[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint136_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint136[]", 17)
    }

    /// Add a `uint144[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint144_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint144[]", 18)
    }

    /// Add a `uint152[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint152_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint152[]", 19)
    }

    /// Add a `uint160[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint160_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint160[]", 20)
    }

    /// Add a `uint168[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint168_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint168[]", 21)
    }

    /// Add a `uint176[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint176_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint176[]", 22)
    }

    /// Add a `uint184[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint184_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint184[]", 23)
    }

    /// Add a `uint192[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint192_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint192[]", 24)
    }

    /// Add a `uint200[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint200_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint200[]", 25)
    }

    /// Add a `uint208[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint208_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint208[]", 26)
    }

    /// Add a `uint216[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint216_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint216[]", 27)
    }

    /// Add a `uint224[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint224_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint224[]", 28)
    }

    /// Add a `uint232[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint232_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint232[]", 29)
    }

    /// Add a `uint240[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint240_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint240[]", 30)
    }

    /// Add a `uint248[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint248_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint248[]", 31)
    }

    /// Add a `uint256[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_uint256_array(&mut self, val_array: &[BigUint]) -> &mut Self {
        self.add_int_array(val_array, "uint256[]", 32)
    }

    /// Add an `address` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
    pub fn add_address(&mut self, address_string: &str) -> &mut Self {
        self.args.push(Argument {
            type_name: "address",
            value_bytes: Self::encode_address(address_string),
            is_dynamic: false,
        });
        self
    }

    /// Add an `address[]` argument to the `ContractFunctionParameters`
    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
