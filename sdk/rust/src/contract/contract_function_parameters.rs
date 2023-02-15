use std::cmp::max;
use std::str::FromStr;

use num_bigint::{
    BigInt,
    BigUint,
    Sign,
};

use crate::contract::contract_function_selector::ContractFunctionSelector;
use crate::evm_address::IdEvmAddress;

/// Builder for encoding parameters for a Solidity contract constructor/function call.
#[derive(Debug, Clone, Default)]
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
        (false, self.to_be_bytes().to_vec())
    }
}

impl IntEncode for i8 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.is_negative(), self.to_be_bytes().to_vec())
    }
}

impl IntEncode for u16 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_be_bytes().to_vec())
    }
}

impl IntEncode for i16 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.is_negative(), self.to_be_bytes().to_vec())
    }
}

impl IntEncode for u32 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_be_bytes().to_vec())
    }
}

impl IntEncode for i32 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.is_negative(), self.to_be_bytes().to_vec())
    }
}

impl IntEncode for u64 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_be_bytes().to_vec())
    }
}

impl IntEncode for i64 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.is_negative(), self.to_be_bytes().to_vec())
    }
}

impl IntEncode for u128 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_be_bytes().to_vec())
    }
}

impl IntEncode for i128 {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.is_negative(), self.to_be_bytes().to_vec())
    }
}

impl IntEncode for BigUint {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (false, self.to_bytes_be())
    }
}

impl IntEncode for BigInt {
    fn get_is_negative_and_be_bytes(&self) -> (bool, Vec<u8>) {
        (self.sign() == Sign::Minus, self.to_signed_bytes_be())
    }
}

// todo: remove this
#[allow(clippy::needless_pass_by_value)]
impl ContractFunctionParameters {
    /// Create a new, empty `ContractFunctionParameters`
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the encoding of the currently added parameters as bytes.
    ///
    /// You can continue adding arguments after calling this function.
    // passing an `&Optiuon<A>` or an `Option<&A>` would just be pointlessly more restrictive,
    // since downstream code can just...
    // Call this with `Option<&A>` anyway if they want to keep ownership of it.
    #[allow(clippy::needless_pass_by_value)]
    pub fn to_bytes<A>(&self, func_name: Option<A>) -> Vec<u8>
    where
        A: AsRef<str>,
    {
        // This function exists to alleviate monomorphization costs.
        // Generic functions are instantiated once per type per code gen unit (which is at least once per crate)
        // That can lead to a *lot* of mostly the same generic function.
        // This isn't always worth an inner function, namely, the whole thing would get inlined,
        // the inner function doesn't matter at all.
        // However, this function is quite large, and might not get inlined, especially in -Copt-level=z
        //
        // see: https://www.possiblerust.com/pattern/non-generic-inner-functions
        fn inner(args: &[Argument], func_name: Option<&str>) -> Vec<u8> {
            let mut current_dynamic_offset = args.len() * 32;
            let mut arg_bytes = Vec::new();
            let mut dynamic_arg_bytes = Vec::new();
            let mut function_selector = func_name.map(ContractFunctionSelector::new);
            for arg in args {
                if let Some(selector) = &mut function_selector {
                    selector.add_param_type(arg.type_name);
                }
                if arg.is_dynamic {
                    arg_bytes.extend_from_slice(
                        left_pad_32_bytes(current_dynamic_offset.to_be_bytes().as_slice(), false)
                            .as_slice(),
                    );
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

        inner(&self.args, func_name.as_ref().map(A::as_ref))
    }

    /// Add a `string` argument to the `ContractFunctionParameters`
    pub fn add_string<T: AsRef<str>>(&mut self, val: T) -> &mut Self {
        self.args.push(Argument {
            type_name: "string",
            value_bytes: encode_dynamic_bytes(val.as_ref().as_bytes()),
            is_dynamic: true,
        });
        self
    }

    /// Add a `string[]` argument to the `ContractFunctionParameters`
    pub fn add_string_array<T: AsRef<str>>(&mut self, val: &[T]) -> &mut Self {
        self.args.push(Argument {
            type_name: "string[]",
            value_bytes: encode_array_of_dynamic_byte_arrays(
                val.iter().map(|s| s.as_ref().as_bytes()),
                val.len(),
            ),
            is_dynamic: true,
        });
        self
    }

    /// Add a `bytes` argument to the `ContractFunctionParameters`
    pub fn add_bytes(&mut self, val: &[u8]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes",
            value_bytes: encode_dynamic_bytes(val),
            is_dynamic: true,
        });
        self
    }

    /// Add a `bytes[]` argument to the `ContractFunctionParameters`
    pub fn add_bytes_array(&mut self, val: &[&[u8]]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes[]",
            value_bytes: encode_array_of_dynamic_byte_arrays(val, val.len()),
            is_dynamic: true,
        });
        self
    }

    /// Add a `bytes32` argument to the `ContractFunctionParameters`
    pub fn add_bytes32(&mut self, val: &[u8; 32]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes32",
            value_bytes: val.to_vec(),
            is_dynamic: false,
        });
        self
    }

    /// Add a `bytes32[]` argument to the `ContractFunctionParameters`
    pub fn add_bytes32_array(&mut self, val: &[[u8; 32]]) -> &mut Self {
        self.args.push(Argument {
            type_name: "bytes32",
            value_bytes: encode_array_of_32_byte_elements(val.iter().copied(), val.len()),
            is_dynamic: true,
        });
        self
    }

    /// Add a `bool` argument to the `ContractFunctionParameters`
    pub fn add_bool(&mut self, val: bool) -> &mut Self {
        self.args.push(Argument {
            type_name: "bool",
            value_bytes: left_pad_32_bytes(
                // a bool in rust is guaranteed to be of value 0 or 1
                u32::from(val).to_be_bytes().as_slice(),
                false,
            )
            .to_vec(),
            is_dynamic: false,
        });
        self
    }

    fn add_int<T>(&mut self, val: &T, type_name: &'static str, byte_count: isize) -> &mut Self
    where
        T: IntEncode,
    {
        self.args.push(Argument {
            type_name,
            value_bytes: truncate_and_left_pad_32_bytes(val, byte_count).to_vec(),
            is_dynamic: false,
        });
        self
    }

    fn add_int_array<T>(
        &mut self,
        values: &[T],
        type_name: &'static str,
        byte_count: isize,
    ) -> &mut Self
    where
        T: IntEncode,
    {
        self.args.push(Argument {
            type_name,
            value_bytes: encode_array_of_32_byte_elements(
                values.iter().map(|val| truncate_and_left_pad_32_bytes(val, byte_count)),
                values.len(),
            ),
            is_dynamic: true,
        });
        self
    }

    /// Add an `int8` argument to the `ContractFunctionParameters`
    pub fn add_int8(&mut self, val: i8) -> &mut Self {
        self.add_int(&val, "int8", 1)
    }

    /// Add an `int16` argument to the `ContractFunctionParameters`
    pub fn add_int16(&mut self, val: i16) -> &mut Self {
        self.add_int(&val, "int16", 2)
    }

    /// Add an `int24` argument to the `ContractFunctionParameters`
    pub fn add_int24(&mut self, val: i32) -> &mut Self {
        self.add_int(&val, "int24", 3)
    }

    /// Add an `int32` argument to the `ContractFunctionParameters`
    pub fn add_int32(&mut self, val: i32) -> &mut Self {
        self.add_int(&val, "int32", 4)
    }

    /// Add an `int40` argument to the `ContractFunctionParameters`
    pub fn add_int40(&mut self, val: i64) -> &mut Self {
        self.add_int(&val, "int40", 5)
    }

    /// Add an `int48` argument to the `ContractFunctionParameters`
    pub fn add_int48(&mut self, val: i64) -> &mut Self {
        self.add_int(&val, "int48", 6)
    }

    /// Add an `int56` argument to the `ContractFunctionParameters`
    pub fn add_int56(&mut self, val: i64) -> &mut Self {
        self.add_int(&val, "int56", 7)
    }

    /// Add an `int64` argument to the `ContractFunctionParameters`
    pub fn add_int64(&mut self, val: i64) -> &mut Self {
        self.add_int(&val, "int64", 8)
    }

    /// Add an `int72` argument to the `ContractFunctionParameters`
    pub fn add_int72(&mut self, val: i128) -> &mut Self {
        self.add_int(&val, "int72", 9)
    }

    /// Add an `int80` argument to the `ContractFunctionParameters`
    pub fn add_int80(&mut self, val: i128) -> &mut Self {
        self.add_int(&val, "int80", 10)
    }

    /// Add an `int88` argument to the `ContractFunctionParameters`
    pub fn add_int88(&mut self, val: i128) -> &mut Self {
        self.add_int(&val, "int88", 11)
    }

    /// Add an `int96` argument to the `ContractFunctionParameters`
    pub fn add_int96(&mut self, val: i128) -> &mut Self {
        self.add_int(&val, "int96", 12)
    }

    /// Add an `int104` argument to the `ContractFunctionParameters`
    pub fn add_int104(&mut self, val: i128) -> &mut Self {
        self.add_int(&val, "int104", 13)
    }

    /// Add an `int112` argument to the `ContractFunctionParameters`
    pub fn add_int112(&mut self, val: i128) -> &mut Self {
        self.add_int(&val, "int112", 14)
    }

    /// Add an `int120` argument to the `ContractFunctionParameters`
    pub fn add_int120(&mut self, val: i128) -> &mut Self {
        self.add_int(&val, "int120", 15)
    }

    /// Add an `int128` argument to the `ContractFunctionParameters`
    pub fn add_int128(&mut self, val: i128) -> &mut Self {
        self.add_int(&val, "int128", 16)
    }

    /// Add an `int136` argument to the `ContractFunctionParameters`
    pub fn add_int136(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int136", 17)
    }

    /// Add an `int144` argument to the `ContractFunctionParameters`
    pub fn add_int144(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int144", 18)
    }

    /// Add an `int152` argument to the `ContractFunctionParameters`
    pub fn add_int152(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int152", 19)
    }

    /// Add an `int160` argument to the `ContractFunctionParameters`
    pub fn add_int160(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int160", 20)
    }

    /// Add an `int168` argument to the `ContractFunctionParameters`
    pub fn add_int168(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int168", 21)
    }

    /// Add an `int176` argument to the `ContractFunctionParameters`
    pub fn add_int176(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int176", 22)
    }

    /// Add an `int184` argument to the `ContractFunctionParameters`
    pub fn add_int184(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int184", 23)
    }

    /// Add an `int192` argument to the `ContractFunctionParameters`
    pub fn add_int192(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int192", 24)
    }

    /// Add an `int200` argument to the `ContractFunctionParameters`
    pub fn add_int200(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int200", 25)
    }

    /// Add an `int208` argument to the `ContractFunctionParameters`
    pub fn add_int208(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int208", 26)
    }

    /// Add an `int216` argument to the `ContractFunctionParameters`
    pub fn add_int216(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int216", 27)
    }

    /// Add an `int224` argument to the `ContractFunctionParameters`
    pub fn add_int224(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int224", 28)
    }

    /// Add an `int232` argument to the `ContractFunctionParameters`
    pub fn add_int232(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int232", 29)
    }

    /// Add an `int240` argument to the `ContractFunctionParameters`
    pub fn add_int240(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int240", 30)
    }

    /// Add an `int248` argument to the `ContractFunctionParameters`
    pub fn add_int248(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int248", 31)
    }

    /// Add an `int256` argument to the `ContractFunctionParameters`
    pub fn add_int256(&mut self, val: BigInt) -> &mut Self {
        self.add_int(&val, "int256", 32)
    }

    /// Add an `int8[]` argument to the `ContractFunctionParameters`
    pub fn add_int8_array(&mut self, values: &[i8]) -> &mut Self {
        self.add_int_array(values, "int8[]", 1)
    }

    /// Add an `int16[]` argument to the `ContractFunctionParameters`
    pub fn add_int16_array(&mut self, values: &[i16]) -> &mut Self {
        self.add_int_array(values, "int16[]", 2)
    }

    /// Add an `int24[]` argument to the `ContractFunctionParameters`
    pub fn add_int24_array(&mut self, values: &[i32]) -> &mut Self {
        self.add_int_array(values, "int24[]", 3)
    }

    /// Add an `int32[]` argument to the `ContractFunctionParameters`
    pub fn add_int32_array(&mut self, values: &[i32]) -> &mut Self {
        self.add_int_array(values, "int32[]", 4)
    }

    /// Add an `int40[]` argument to the `ContractFunctionParameters`
    pub fn add_int40_array(&mut self, values: &[i64]) -> &mut Self {
        self.add_int_array(values, "int40[]", 5)
    }

    /// Add an `int48[]` argument to the `ContractFunctionParameters`
    pub fn add_int48_array(&mut self, values: &[i64]) -> &mut Self {
        self.add_int_array(values, "int48[]", 6)
    }

    /// Add an `int56[]` argument to the `ContractFunctionParameters`
    pub fn add_int56_array(&mut self, values: &[i64]) -> &mut Self {
        self.add_int_array(values, "int56[]", 7)
    }

    /// Add an `int64[]` argument to the `ContractFunctionParameters`
    pub fn add_int64_array(&mut self, values: &[i64]) -> &mut Self {
        self.add_int_array(values, "int64[]", 8)
    }

    /// Add an `int72[]` argument to the `ContractFunctionParameters`
    pub fn add_int72_array(&mut self, values: &[i128]) -> &mut Self {
        self.add_int_array(values, "int72[]", 9)
    }

    /// Add an `int80[]` argument to the `ContractFunctionParameters`
    pub fn add_int80_array(&mut self, values: &[i128]) -> &mut Self {
        self.add_int_array(values, "int80[]", 10)
    }

    /// Add an `int88[]` argument to the `ContractFunctionParameters`
    pub fn add_int88_array(&mut self, values: &[i128]) -> &mut Self {
        self.add_int_array(values, "int88[]", 11)
    }

    /// Add an `int96[]` argument to the `ContractFunctionParameters`
    pub fn add_int96_array(&mut self, values: &[i128]) -> &mut Self {
        self.add_int_array(values, "int96[]", 12)
    }

    /// Add an `int104[]` argument to the `ContractFunctionParameters`
    pub fn add_int104_array(&mut self, values: &[i128]) -> &mut Self {
        self.add_int_array(values, "int104[]", 13)
    }

    /// Add an `int112[]` argument to the `ContractFunctionParameters`
    pub fn add_int112_array(&mut self, values: &[i128]) -> &mut Self {
        self.add_int_array(values, "int112[]", 14)
    }

    /// Add an `int120[]` argument to the `ContractFunctionParameters`
    pub fn add_int120_array(&mut self, values: &[i128]) -> &mut Self {
        self.add_int_array(values, "int120[]", 15)
    }

    /// Add an `int128[]` argument to the `ContractFunctionParameters`
    pub fn add_int128_array(&mut self, values: &[i128]) -> &mut Self {
        self.add_int_array(values, "int128[]", 16)
    }

    /// Add an `int136[]` argument to the `ContractFunctionParameters`
    pub fn add_int136_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int136[]", 17)
    }

    /// Add an `int144[]` argument to the `ContractFunctionParameters`
    pub fn add_int144_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int144[]", 18)
    }

    /// Add an `int152[]` argument to the `ContractFunctionParameters`
    pub fn add_int152_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int152[]", 19)
    }

    /// Add an `int160[]` argument to the `ContractFunctionParameters`
    pub fn add_int160_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int160[]", 20)
    }

    /// Add an `int168[]` argument to the `ContractFunctionParameters`
    pub fn add_int168_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int168[]", 21)
    }

    /// Add an `int176[]` argument to the `ContractFunctionParameters`
    pub fn add_int176_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int176[]", 22)
    }

    /// Add an `int184[]` argument to the `ContractFunctionParameters`
    pub fn add_int184_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int184[]", 23)
    }

    /// Add an `int192[]` argument to the `ContractFunctionParameters`
    pub fn add_int192_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int192[]", 24)
    }

    /// Add an `int200[]` argument to the `ContractFunctionParameters`
    pub fn add_int200_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int200[]", 25)
    }

    /// Add an `int208[]` argument to the `ContractFunctionParameters`
    pub fn add_int208_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int208[]", 26)
    }

    /// Add an `int216[]` argument to the `ContractFunctionParameters`
    pub fn add_int216_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int216[]", 27)
    }

    /// Add an `int224[]` argument to the `ContractFunctionParameters`
    pub fn add_int224_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int224[]", 28)
    }

    /// Add an `int232[]` argument to the `ContractFunctionParameters`
    pub fn add_int232_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int232[]", 29)
    }

    /// Add an `int240[]` argument to the `ContractFunctionParameters`
    pub fn add_int240_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int240[]", 30)
    }

    /// Add an `int248[]` argument to the `ContractFunctionParameters`
    pub fn add_int248_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int248[]", 31)
    }

    /// Add an `int256[]` argument to the `ContractFunctionParameters`
    pub fn add_int256_array(&mut self, values: &[BigInt]) -> &mut Self {
        self.add_int_array(values, "int256[]", 32)
    }

    /// Add a `uint8` argument to the `ContractFunctionParameters`
    pub fn add_uint8(&mut self, val: u8) -> &mut Self {
        self.add_int(&val, "uint8", 1)
    }

    /// Add a `uint16` argument to the `ContractFunctionParameters`
    pub fn add_uint16(&mut self, val: u16) -> &mut Self {
        self.add_int(&val, "uint16", 2)
    }

    /// Add a `uint24` argument to the `ContractFunctionParameters`
    pub fn add_uint24(&mut self, val: u32) -> &mut Self {
        self.add_int(&val, "uint24", 3)
    }

    /// Add a `uint32` argument to the `ContractFunctionParameters`
    pub fn add_uint32(&mut self, val: u32) -> &mut Self {
        self.add_int(&val, "uint32", 4)
    }

    /// Add a `uint40` argument to the `ContractFunctionParameters`
    pub fn add_uint40(&mut self, val: u64) -> &mut Self {
        self.add_int(&val, "uint40", 5)
    }

    /// Add a `uint48` argument to the `ContractFunctionParameters`
    pub fn add_uint48(&mut self, val: u64) -> &mut Self {
        self.add_int(&val, "uint48", 6)
    }

    /// Add a `uint56` argument to the `ContractFunctionParameters`
    pub fn add_uint56(&mut self, val: u64) -> &mut Self {
        self.add_int(&val, "uint56", 7)
    }

    /// Add a `uint64` argument to the `ContractFunctionParameters`
    pub fn add_uint64(&mut self, val: u64) -> &mut Self {
        self.add_int(&val, "uint64", 8)
    }

    /// Add a `uint72` argument to the `ContractFunctionParameters`
    pub fn add_uint72(&mut self, val: u128) -> &mut Self {
        self.add_int(&val, "uint72", 9)
    }

    /// Add a `uint80` argument to the `ContractFunctionParameters`
    pub fn add_uint80(&mut self, val: u128) -> &mut Self {
        self.add_int(&val, "uint80", 10)
    }

    /// Add a `uint88` argument to the `ContractFunctionParameters`
    pub fn add_uint88(&mut self, val: u128) -> &mut Self {
        self.add_int(&val, "uint88", 11)
    }

    /// Add a `uint96` argument to the `ContractFunctionParameters`
    pub fn add_uint96(&mut self, val: u128) -> &mut Self {
        self.add_int(&val, "uint96", 12)
    }

    /// Add a `uint104` argument to the `ContractFunctionParameters`
    pub fn add_uint104(&mut self, val: u128) -> &mut Self {
        self.add_int(&val, "uint104", 13)
    }

    /// Add a `uint112` argument to the `ContractFunctionParameters`
    pub fn add_uint112(&mut self, val: u128) -> &mut Self {
        self.add_int(&val, "uint112", 14)
    }

    /// Add a `uint120` argument to the `ContractFunctionParameters`
    pub fn add_uint120(&mut self, val: u128) -> &mut Self {
        self.add_int(&val, "uint120", 15)
    }

    /// Add a `uint128` argument to the `ContractFunctionParameters`
    pub fn add_uint128(&mut self, val: u128) -> &mut Self {
        self.add_int(&val, "uint128", 16)
    }

    /// Add a `uint136` argument to the `ContractFunctionParameters`
    pub fn add_uint136(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint136", 17)
    }

    /// Add a `uint144` argument to the `ContractFunctionParameters`
    pub fn add_uint144(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint144", 18)
    }

    /// Add a `uint152` argument to the `ContractFunctionParameters`
    pub fn add_uint152(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint152", 19)
    }

    /// Add a `uint160` argument to the `ContractFunctionParameters`
    pub fn add_uint160(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint160", 20)
    }

    /// Add a `uint168` argument to the `ContractFunctionParameters`
    pub fn add_uint168(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint168", 21)
    }

    /// Add a `uint176` argument to the `ContractFunctionParameters`
    pub fn add_uint176(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint176", 22)
    }

    /// Add a `uint184` argument to the `ContractFunctionParameters`
    pub fn add_uint184(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint184", 23)
    }

    /// Add a `uint192` argument to the `ContractFunctionParameters`
    pub fn add_uint192(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint192", 24)
    }

    /// Add a `uint200` argument to the `ContractFunctionParameters`
    pub fn add_uint200(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint200", 25)
    }

    /// Add a `uint208` argument to the `ContractFunctionParameters`
    pub fn add_uint208(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint208", 26)
    }

    /// Add a `uint216` argument to the `ContractFunctionParameters`
    pub fn add_uint216(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint216", 27)
    }

    /// Add a `uint224` argument to the `ContractFunctionParameters`
    pub fn add_uint224(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint224", 28)
    }

    /// Add a `uint232` argument to the `ContractFunctionParameters`
    pub fn add_uint232(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint232", 29)
    }

    /// Add a `uint240` argument to the `ContractFunctionParameters`
    pub fn add_uint240(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint240", 30)
    }

    /// Add a `uint248` argument to the `ContractFunctionParameters`
    pub fn add_uint248(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint248", 31)
    }

    /// Add a `uint256` argument to the `ContractFunctionParameters`
    pub fn add_uint256(&mut self, val: BigUint) -> &mut Self {
        self.add_int(&val, "uint256", 32)
    }

    /// Add a `uint8[]` argument to the `ContractFunctionParameters`
    pub fn add_uint8_array(&mut self, values: &[u8]) -> &mut Self {
        self.add_int_array(values, "uint8[]", 1)
    }

    /// Add a `uint16[]` argument to the `ContractFunctionParameters`
    pub fn add_uint16_array(&mut self, values: &[u16]) -> &mut Self {
        self.add_int_array(values, "uint16[]", 2)
    }

    /// Add a `uint24[]` argument to the `ContractFunctionParameters`
    pub fn add_uint24_array(&mut self, values: &[u32]) -> &mut Self {
        self.add_int_array(values, "uint24[]", 3)
    }

    /// Add a `uint32[]` argument to the `ContractFunctionParameters`
    pub fn add_uint32_array(&mut self, values: &[u32]) -> &mut Self {
        self.add_int_array(values, "uint32[]", 4)
    }

    /// Add a `uint40[]` argument to the `ContractFunctionParameters`
    pub fn add_uint40_array(&mut self, values: &[u64]) -> &mut Self {
        self.add_int_array(values, "uint40[]", 5)
    }

    /// Add a `uint48[]` argument to the `ContractFunctionParameters`
    pub fn add_uint48_array(&mut self, values: &[u64]) -> &mut Self {
        self.add_int_array(values, "uint48[]", 6)
    }

    /// Add a `uint56[]` argument to the `ContractFunctionParameters`
    pub fn add_uint56_array(&mut self, values: &[u64]) -> &mut Self {
        self.add_int_array(values, "uint56[]", 7)
    }

    /// Add a `uint64[]` argument to the `ContractFunctionParameters`
    pub fn add_uint64_array(&mut self, values: &[u64]) -> &mut Self {
        self.add_int_array(values, "uint64[]", 8)
    }

    /// Add a `uint72[]` argument to the `ContractFunctionParameters`
    pub fn add_uint72_array(&mut self, values: &[u128]) -> &mut Self {
        self.add_int_array(values, "uint72[]", 9)
    }

    /// Add a `uint80[]` argument to the `ContractFunctionParameters`
    pub fn add_uint80_array(&mut self, values: &[u128]) -> &mut Self {
        self.add_int_array(values, "uint80[]", 10)
    }

    /// Add a `uint88[]` argument to the `ContractFunctionParameters`
    pub fn add_uint88_array(&mut self, values: &[u128]) -> &mut Self {
        self.add_int_array(values, "uint88[]", 11)
    }

    /// Add a `uint96[]` argument to the `ContractFunctionParameters`
    pub fn add_uint96_array(&mut self, values: &[u128]) -> &mut Self {
        self.add_int_array(values, "uint96[]", 12)
    }

    /// Add a `uint104[]` argument to the `ContractFunctionParameters`
    pub fn add_uint104_array(&mut self, values: &[u128]) -> &mut Self {
        self.add_int_array(values, "uint104[]", 13)
    }

    /// Add a `uint112[]` argument to the `ContractFunctionParameters`
    pub fn add_uint112_array(&mut self, values: &[u128]) -> &mut Self {
        self.add_int_array(values, "uint112[]", 14)
    }

    /// Add a `uint120[]` argument to the `ContractFunctionParameters`
    pub fn add_uint120_array(&mut self, values: &[u128]) -> &mut Self {
        self.add_int_array(values, "uint120[]", 15)
    }

    /// Add a `uint128[]` argument to the `ContractFunctionParameters`
    pub fn add_uint128_array(&mut self, values: &[u128]) -> &mut Self {
        self.add_int_array(values, "uint128[]", 16)
    }

    /// Add a `uint136[]` argument to the `ContractFunctionParameters`
    pub fn add_uint136_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint136[]", 17)
    }

    /// Add a `uint144[]` argument to the `ContractFunctionParameters`
    pub fn add_uint144_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint144[]", 18)
    }

    /// Add a `uint152[]` argument to the `ContractFunctionParameters`
    pub fn add_uint152_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint152[]", 19)
    }

    /// Add a `uint160[]` argument to the `ContractFunctionParameters`
    pub fn add_uint160_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint160[]", 20)
    }

    /// Add a `uint168[]` argument to the `ContractFunctionParameters`
    pub fn add_uint168_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint168[]", 21)
    }

    /// Add a `uint176[]` argument to the `ContractFunctionParameters`
    pub fn add_uint176_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint176[]", 22)
    }

    /// Add a `uint184[]` argument to the `ContractFunctionParameters`
    pub fn add_uint184_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint184[]", 23)
    }

    /// Add a `uint192[]` argument to the `ContractFunctionParameters`
    pub fn add_uint192_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint192[]", 24)
    }

    /// Add a `uint200[]` argument to the `ContractFunctionParameters`
    pub fn add_uint200_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint200[]", 25)
    }

    /// Add a `uint208[]` argument to the `ContractFunctionParameters`
    pub fn add_uint208_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint208[]", 26)
    }

    /// Add a `uint216[]` argument to the `ContractFunctionParameters`
    pub fn add_uint216_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint216[]", 27)
    }

    /// Add a `uint224[]` argument to the `ContractFunctionParameters`
    pub fn add_uint224_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint224[]", 28)
    }

    /// Add a `uint232[]` argument to the `ContractFunctionParameters`
    pub fn add_uint232_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint232[]", 29)
    }

    /// Add a `uint240[]` argument to the `ContractFunctionParameters`
    pub fn add_uint240_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint240[]", 30)
    }

    /// Add a `uint248[]` argument to the `ContractFunctionParameters`
    pub fn add_uint248_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint248[]", 31)
    }

    /// Add a `uint256[]` argument to the `ContractFunctionParameters`
    pub fn add_uint256_array(&mut self, values: &[BigUint]) -> &mut Self {
        self.add_int_array(values, "uint256[]", 32)
    }

    /// Add an `address` argument to the `ContractFunctionParameters`
    pub fn add_address(&mut self, address: &str) -> &mut Self {
        self.args.push(Argument {
            type_name: "address",
            value_bytes: encode_address(address).to_vec(),
            is_dynamic: false,
        });
        self
    }

    /// Add an `address[]` argument to the `ContractFunctionParameters`
    pub fn add_address_array(&mut self, addresses: &[&str]) -> &mut Self {
        self.args.push(Argument {
            type_name: "address[]",
            value_bytes: encode_array_of_32_byte_elements(
                addresses.iter().map(|addr| encode_address(addr)),
                addresses.len(),
            ),
            is_dynamic: true,
        });
        self
    }

    /// Add a `function` argument to the `ContractFunctionParameters`
    pub fn add_function(
        &mut self,
        address: &str,
        mut selector: ContractFunctionSelector,
    ) -> &mut Self {
        let mut value_bytes = IdEvmAddress::from_str(address).unwrap().to_bytes().to_vec();
        value_bytes.extend(selector.finish());
        self.args.push(Argument {
            type_name: "function",
            value_bytes: right_pad_32_bytes(value_bytes.as_slice()).to_vec(),
            is_dynamic: false,
        });
        self
    }
}

fn left_pad_32_bytes(bytes: &[u8], is_negative: bool) -> [u8; 32] {
    let pad_byte = if is_negative { 0xFF } else { 0x00 };

    let mut result = [pad_byte; 32];
    result[(32 - bytes.len())..].copy_from_slice(bytes);
    result
}

fn truncate_and_left_pad_32_bytes<T>(val: &T, byte_count: isize) -> [u8; 32]
where
    T: IntEncode,
{
    let (is_negative, full_value_bytes) = val.get_is_negative_and_be_bytes();
    let truncated_value_bytes =
        &full_value_bytes[max((full_value_bytes.len() as isize) - byte_count, 0) as usize..];
    left_pad_32_bytes(truncated_value_bytes, is_negative)
}

fn right_pad_32_bytes(bytes: &[u8]) -> [u8; 32] {
    let mut result = [0_u8; 32];
    result[..bytes.len()].copy_from_slice(bytes);
    result
}

fn encode_address(address: &str) -> [u8; 32] {
    left_pad_32_bytes(IdEvmAddress::from_str(address).unwrap().0 .0.as_slice(), false)
}

fn encode_dynamic_bytes(bytes: &[u8]) -> Vec<u8> {
    let mut out_bytes = left_pad_32_bytes(bytes.len().to_be_bytes().as_slice(), false).to_vec();
    out_bytes.extend_from_slice(right_pad_32_bytes(bytes).as_slice());
    out_bytes
}

fn encode_array_of_dynamic_byte_arrays<I>(elements: I, elements_len: usize) -> Vec<u8>
where
    I: IntoIterator,
    I::Item: AsRef<[u8]>,
{
    let head_len = (elements_len + 1) * 32;
    let mut out_bytes = Vec::with_capacity(head_len);
    out_bytes.extend_from_slice(
        left_pad_32_bytes(elements_len.to_be_bytes().as_slice(), false).as_slice(),
    );
    let mut current_offset = elements_len * 32;
    let mut body_bytes: Vec<u8> = Vec::new();
    for element in elements {
        let element = element.as_ref();
        out_bytes.extend_from_slice(
            left_pad_32_bytes(current_offset.to_be_bytes().as_slice(), false).as_slice(),
        );
        current_offset += element.len();
        body_bytes.extend(element);
    }
    out_bytes.extend(body_bytes);
    out_bytes
}

fn encode_array_of_32_byte_elements<I>(elements: I, elements_len: usize) -> Vec<u8>
where
    I: IntoIterator<Item = [u8; 32]>,
{
    let mut out_bytes = left_pad_32_bytes(elements_len.to_be_bytes().as_slice(), false).to_vec();
    out_bytes.reserve(out_bytes.len() + (elements_len * 32));
    out_bytes.extend(elements.into_iter().flatten());
    out_bytes
}

#[cfg(test)]
mod tests {
    use num_bigint::{
        BigInt,
        BigUint,
    };

    use crate::contract::contract_function_parameters::ContractFunctionParameters;
    use crate::contract::contract_function_selector::ContractFunctionSelector;

    #[test]
    fn misc_params() {
        let param_bytes = ContractFunctionParameters::new()
            .add_uint8(0x1)
            .add_int8(-0x2)
            .add_uint32(0x3)
            .add_int32(-0x4)
            .add_uint64(0x4)
            .add_int64(-0x5)
            .add_uint256(BigUint::from(0x6_u32))
            .add_int256(BigInt::from(-0x7))
            .add_uint8_array([0x1, 0x2, 0x3, 0x4].as_slice())
            .add_int8_array([-0x5, 0x6, 0x7, -0x8].as_slice())
            .add_uint32_array([0x9, 0xA, 0xB, 0xC].as_slice())
            .add_int32_array([-0xD, 0xE, 0xF, -0x10].as_slice())
            .add_uint64_array([0x11, 0x12, 0x13, 0x14].as_slice())
            .add_int64_array([-0x15, 0x16, 0x17, -0x18].as_slice())
            .add_uint256_array([BigUint::from(0x19_u32)].as_slice())
            .add_int256_array([BigInt::from(-0x1A)].as_slice())
            .to_bytes(Some("foo"));

        assert_eq!(
            hex::encode(param_bytes),
            "11bcd903\
                0000000000000000000000000000000000000000000000000000000000000001\
                fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe\
                0000000000000000000000000000000000000000000000000000000000000003\
                fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc\
                0000000000000000000000000000000000000000000000000000000000000004\
                fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb\
                0000000000000000000000000000000000000000000000000000000000000006\
                fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff9\
                0000000000000000000000000000000000000000000000000000000000000200\
                00000000000000000000000000000000000000000000000000000000000002a0\
                0000000000000000000000000000000000000000000000000000000000000340\
                00000000000000000000000000000000000000000000000000000000000003e0\
                0000000000000000000000000000000000000000000000000000000000000480\
                0000000000000000000000000000000000000000000000000000000000000520\
                00000000000000000000000000000000000000000000000000000000000005c0\
                0000000000000000000000000000000000000000000000000000000000000600\
                0000000000000000000000000000000000000000000000000000000000000004\
                0000000000000000000000000000000000000000000000000000000000000001\
                0000000000000000000000000000000000000000000000000000000000000002\
                0000000000000000000000000000000000000000000000000000000000000003\
                0000000000000000000000000000000000000000000000000000000000000004\
                0000000000000000000000000000000000000000000000000000000000000004\
                fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffb\
                0000000000000000000000000000000000000000000000000000000000000006\
                0000000000000000000000000000000000000000000000000000000000000007\
                fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8\
                0000000000000000000000000000000000000000000000000000000000000004\
                0000000000000000000000000000000000000000000000000000000000000009\
                000000000000000000000000000000000000000000000000000000000000000a\
                000000000000000000000000000000000000000000000000000000000000000b\
                000000000000000000000000000000000000000000000000000000000000000c\
                0000000000000000000000000000000000000000000000000000000000000004\
                fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff3\
                000000000000000000000000000000000000000000000000000000000000000e\
                000000000000000000000000000000000000000000000000000000000000000f\
                fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff0\
                0000000000000000000000000000000000000000000000000000000000000004\
                0000000000000000000000000000000000000000000000000000000000000011\
                0000000000000000000000000000000000000000000000000000000000000012\
                0000000000000000000000000000000000000000000000000000000000000013\
                0000000000000000000000000000000000000000000000000000000000000014\
                0000000000000000000000000000000000000000000000000000000000000004\
                ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffeb\
                0000000000000000000000000000000000000000000000000000000000000016\
                0000000000000000000000000000000000000000000000000000000000000017\
                ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe8\
                0000000000000000000000000000000000000000000000000000000000000001\
                0000000000000000000000000000000000000000000000000000000000000019\
                0000000000000000000000000000000000000000000000000000000000000001\
                ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe6"
        )
    }

    #[test]
    fn address_params() {
        let param_bytes = ContractFunctionParameters::new()
            .add_address("1122334455667788990011223344556677889900")
            .add_address("0x1122334455667788990011223344556677889900")
            .add_address_array(
                [
                    "1122334455667788990011223344556677889900",
                    "1122334455667788990011223344556677889900",
                ]
                .as_slice(),
            )
            .to_bytes(Some("foo"));

        assert_eq!(
            hex::encode(param_bytes),
            "7d48c86d\
                0000000000000000000000001122334455667788990011223344556677889900\
                0000000000000000000000001122334455667788990011223344556677889900\
                0000000000000000000000000000000000000000000000000000000000000060\
                0000000000000000000000000000000000000000000000000000000000000002\
                0000000000000000000000001122334455667788990011223344556677889900\
                0000000000000000000000001122334455667788990011223344556677889900"
        )
    }

    #[test]
    fn function_params() {
        let param_bytes = ContractFunctionParameters::new()
            .add_function(
                "1122334455667788990011223344556677889900",
                ContractFunctionSelector::from([1, 2, 3, 4]),
            )
            .add_function(
                "0x1122334455667788990011223344556677889900",
                ContractFunctionSelector::new("randomFunction").add_bool().clone(),
            )
            .to_bytes(Some("foo"));
        assert_eq!(
            hex::encode(param_bytes),
            "c99c40cd\
                1122334455667788990011223344556677889900010203040000000000000000\
                112233445566778899001122334455667788990063441d820000000000000000"
        );
    }
}
