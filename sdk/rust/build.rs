use std::env;

use cbindgen::{
    Config,
    EnumConfig,
    Language,
    RenameRule,
};

fn main() -> anyhow::Result<()> {
    cbindgen::Builder::new()
        .with_config(Config {
            cpp_compat: true,
            enumeration: EnumConfig {
                rename_variants: RenameRule::QualifiedScreamingSnakeCase,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_crate(env::var("CARGO_MANIFEST_DIR")?)
        .with_include_version(true)
        .with_include_guard("_HEDERA_H")
        .with_language(Language::C)
        .with_item_prefix("Hedera")
        .generate()?
        .write_to_file("../c/include/hedera.h");

    Ok(())
}
