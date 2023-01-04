#!/usr/bin/env python3

int_versions = []
uint_versions = []

int_array_versions = []
uint_array_versions = []

# does not generate 8 bit versions because those require some special treatment.

def add_methods_for_bit_width(solidity_bit_width, rust_signed_type, rust_unsigned_type):
    int_versions.append(
        "/// Add an `int" + str(solidity_bit_width) + "` argument to the `ContractFunctionParameters`\n"
        "#[allow(dead_code)]\n"
        "pub fn add_int" + str(solidity_bit_width) + "(&mut self, val: " + rust_signed_type + ") -> &mut Self {\n"
        "    self.add_int(&val, \"int" + str(solidity_bit_width) + "\", " + str(solidity_bit_width//8) + ")\n"
        "}\n"
    )
    uint_versions.append(
        "/// Add a `uint" + str(solidity_bit_width) + "` argument to the `ContractFunctionParameters`\n"
        "#[allow(dead_code)]\n"
        "pub fn add_uint" + str(solidity_bit_width) + "(&mut self, val: " + rust_unsigned_type + ") -> &mut Self {\n"
        "    self.add_int(&val, \"uint" + str(solidity_bit_width) + "\", " + str(solidity_bit_width//8) + ")\n"
        "}\n"
    )
    int_array_versions.append(
        "/// Add an `int" + str(solidity_bit_width) + "[]` argument to the `ContractFunctionParameters`\n"
        "#[allow(dead_code)]\n"
        "pub fn add_int" + str(solidity_bit_width) + "_array(&mut self, values: &[" + rust_signed_type + "]) -> &mut Self {\n"
        "    self.add_int_array(values, \"int" + str(solidity_bit_width) + "[]\", " + str(solidity_bit_width//8) + ")\n"
        "}\n"
    )
    uint_array_versions.append(
        "/// Add a `uint" + str(solidity_bit_width) + "[]` argument to the `ContractFunctionParameters`\n"
        "#[allow(dead_code)]\n"
        "pub fn add_uint" + str(solidity_bit_width) + "_array(&mut self, values: &[" + rust_unsigned_type + "]) -> &mut Self {\n"
        "    self.add_int_array(values, \"uint" + str(solidity_bit_width) + "[]\", " + str(solidity_bit_width//8) + ")\n"
        "}\n"
    )


for bit_width in range(8, 257, 8):
    if bit_width <= 8:
        add_methods_for_bit_width(bit_width, "i8", "u8")
    elif bit_width <= 16:
        add_methods_for_bit_width(bit_width, "i16", "u16")
    elif bit_width <= 32:
        add_methods_for_bit_width(bit_width, "i32", "u32")
    elif bit_width <= 64:
        add_methods_for_bit_width(bit_width, "i64", "u64")
    elif bit_width <= 128:
        add_methods_for_bit_width(bit_width, "i128", "u128")
    else:
        add_methods_for_bit_width(bit_width, "BigInt", "BigUint")

f = open("output.txt", "w")

for v in int_versions:
    f.write(v + "\n");

for v in int_array_versions:
    f.write(v + "\n");

for v in uint_versions:
    f.write(v + "\n");

for v in uint_array_versions:
    f.write(v + "\n");

f.close()
