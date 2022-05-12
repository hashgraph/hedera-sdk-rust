
# https://github.com/messense/homebrew-macos-cross-toolchains#macos-cross-toolchains
export CC_x86_64_unknown_linux_gnu := if os() == "macos" { "x86_64-unknown-linux-gnu-gcc" } else { "" }
export CXX_x86_64_unknown_linux_gnu := if os() == "macos" { "x86_64-unknown-linux-gnu-g++" } else { "" }
export AR_x86_64_unknown_linux_gnu := if os() == "macos" { "x86_64-unknown-linux-gnu-ar" } else { "" }
export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER := if os() == "macos" { "x86_64-unknown-linux-gnu-gcc" } else { "" }

build:
    @ # Ensure we have rust-src
    @ rustup component add rust-src 2> /dev/null

    @ # Build for all targets except Windows
    @ for TARGET in x86_64-apple-darwin x86_64-apple-ios aarch64-apple-darwin aarch64-apple-ios x86_64-unknown-linux-gnu x86_64-pc-windows-gnu; do \
        rustup target add $TARGET 2> /dev/null; \
        cargo build --features ffi --release -p hedera -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target $TARGET; \
    done

    @ # Strip libraries

    @ for TARGET in x86_64-apple-darwin x86_64-apple-ios aarch64-apple-darwin aarch64-apple-ios; do \
        strip -X -S -N -x ./target/$TARGET/release/libhedera.a 2> /dev/null; \
    done

    @ for TARGET in x86_64-apple-darwin aarch64-apple-darwin; do \
        strip -X -S -N -x ./target/$TARGET/release/libhedera.dylib 2> /dev/null; \
    done

    @ x86_64-unknown-linux-gnu-strip --strip-unneeded target/x86_64-unknown-linux-gnu/release/libhedera.a 2> /dev/null
    @ x86_64-w64-mingw32-strip --strip-unneeded target/x86_64-pc-windows-gnu/release/libhedera.a 2> /dev/null

    @ x86_64-unknown-linux-gnu-strip --strip-unneeded target/x86_64-unknown-linux-gnu/release/libhedera.so 2> /dev/null
    @ x86_64-w64-mingw32-strip --strip-unneeded target/x86_64-pc-windows-gnu/release/hedera.dll 2> /dev/null

    @ # --------------------------------------------------------------------------------------------
    @ # C
    @ # --------------------------------------------------------------------------------------------

    @ # Copy libraries into C SDK

    @ mkdir -p sdk/c/lib/macos-x86_64/
    @ mkdir -p sdk/c/lib/ios-x86_64/
    @ mkdir -p sdk/c/lib/macos-arm64/
    @ mkdir -p sdk/c/lib/ios-arm64/
    @ mkdir -p sdk/c/lib/linux-x86_64/
    @ mkdir -p sdk/c/lib/windows-x86_64/

    @ cp target/x86_64-apple-darwin/release/libhedera.a sdk/c/lib/macos-x86_64/
    @ cp target/x86_64-apple-ios/release/libhedera.a sdk/c/lib/ios-x86_64/
    @ cp target/aarch64-apple-darwin/release/libhedera.a sdk/c/lib/macos-arm64/
    @ cp target/aarch64-apple-ios/release/libhedera.a sdk/c/lib/ios-arm64/
    @ cp target/x86_64-unknown-linux-gnu/release/libhedera.a sdk/c/lib/linux-x86_64/
    @ cp target/x86_64-pc-windows-gnu/release/libhedera.a sdk/c/lib/windows-x86_64/

    @ cp target/x86_64-apple-darwin/release/libhedera.dylib sdk/c/lib/macos-x86_64/
    @ cp target/aarch64-apple-darwin/release/libhedera.dylib sdk/c/lib/macos-arm64/
    @ cp target/x86_64-unknown-linux-gnu/release/libhedera.so sdk/c/lib/linux-x86_64/
    @ cp target/x86_64-pc-windows-gnu/release/hedera.dll sdk/c/lib/windows-x86_64/

    @ # --------------------------------------------------------------------------------------------
    @ # Swift
    @ # --------------------------------------------------------------------------------------------

    @ # Copy libraries into Swift SDK

    @ mkdir -p sdk/swift/CHedera.xcframework/macos-{x86_64,arm64}/Headers/
    @ mkdir -p sdk/swift/CHedera.xcframework/ios-{x86_64-simulator,arm64}/Headers/

    @ cp sdk/c/lib/macos-x86_64/libhedera.a sdk/swift/CHedera.xcframework/macos-x86_64/
    @ cp sdk/c/lib/macos-arm64/libhedera.a sdk/swift/CHedera.xcframework/macos-arm64/
    @ cp sdk/c/lib/ios-x86_64/libhedera.a sdk/swift/CHedera.xcframework/ios-x86_64-simulator/
    @ cp sdk/c/lib/ios-arm64/libhedera.a sdk/swift/CHedera.xcframework/ios-arm64/

    @ # Copy headers into Swift SDK

    @ cp sdk/c/include/hedera.h sdk/swift/CHedera.xcframework/macos-x86_64/Headers/
    @ cp sdk/c/include/hedera.h sdk/swift/CHedera.xcframework/macos-arm64/Headers/
    @ cp sdk/c/include/hedera.h sdk/swift/CHedera.xcframework/ios-x86_64-simulator/Headers/
    @ cp sdk/c/include/hedera.h sdk/swift/CHedera.xcframework/ios-arm64/Headers/

    @ # Copy .modulemap into each platform folder for Swift SDK

    @ cp sdk/swift/CHedera.xcframework/module.modulemap sdk/swift/CHedera.xcframework/macos-x86_64/Headers/
    @ cp sdk/swift/CHedera.xcframework/module.modulemap sdk/swift/CHedera.xcframework/macos-arm64/Headers/
    @ cp sdk/swift/CHedera.xcframework/module.modulemap sdk/swift/CHedera.xcframework/ios-x86_64-simulator/Headers/
    @ cp sdk/swift/CHedera.xcframework/module.modulemap sdk/swift/CHedera.xcframework/ios-arm64/Headers/

clean:
    @ rm -rf sdk/c/lib/*
    @ rm -rf sdk/swift/CHedera.xcframework/ios-*/
    @ rm -rf sdk/swift/CHedera.xcframework/macos-*/
