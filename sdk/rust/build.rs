use cbindgen::Language;
use std::env;

fn main() -> anyhow::Result<()> {
    cbindgen::Builder::new()
        .with_crate(env::var("CARGO_MANIFEST_DIR")?)
        .with_include_version(true)
        .with_include_guard("_HEDERA_H")
        .with_language(Language::C)
        .with_item_prefix("Hedera")
        .generate()?
        .write_to_file("../c/include/hedera.h");

    Ok(())
}
