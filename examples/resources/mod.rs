#![allow(dead_code)]

pub const BIG_CONTENTS: &str = include_str!("big_contents.txt");

/// Bytecode for the precompile example.
pub fn precompile_bytecode() -> String {
    const FILE: &'static str = include_str!("solidity_precompile/PrecompileExample.json");

    bytecode(FILE)
}

fn bytecode(file: &'static str) -> String {
    let mut obj: miniserde::json::Object = miniserde::json::from_str(file).unwrap();

    let value = obj
        .remove("object")
        .or_else(|| obj.remove("bytecode"))
        .unwrap();

    match value {
        miniserde::json::Value::String(it) => return it.clone(),
        _ => unimplemented!(),
    }
}
