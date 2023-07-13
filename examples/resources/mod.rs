#![allow(dead_code)]

pub const BIG_CONTENTS: &str = include_str!("big_contents.txt");

/// Bytecode for the simple contract example.
pub fn simple_bytecode() -> String {
    const FILE: &str = include_str!("hello-world.json");

    bytecode(FILE)
}
/// Bytecode for the stateful contract example.
pub fn stateful_bytecode() -> String {
    const FILE: &str = include_str!("stateful.json");

    bytecode(FILE)
}

fn bytecode(file: &'static str) -> String {
    let mut obj: miniserde::json::Object = miniserde::json::from_str(file).unwrap();

    let value = obj
        .remove("object")
        .or_else(|| obj.remove("bytecode"))
        .unwrap();

    match value {
        miniserde::json::Value::String(it) => it.clone(),
        _ => unimplemented!(),
    }
}
