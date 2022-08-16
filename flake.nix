{
  description = "hedera-sdk";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, ... }:
    utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        rec {
          devShell = pkgs.mkShell
            {
              buildInputs = with pkgs;
                [
                  (rust-bin.fromRustupToolchainFile ./sdk/rust/rust-toolchain)
                  cargo-outdated
                  cargo-edit
                  cmake
                ];
            };
        }
      );
}
