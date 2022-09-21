# NOTE: this is *ONLY* for developer environment configuration,
# and is used by *one* specific developer (`Skyler Ross <skyler@launchbadge.com>`).
# This file MUST NOT be relyed on existing or having any particular setup.
# This can and will break at random. 
# This MAY be completely removed at some arbitrary point in the future.
# This IS NOT required for building the SDK.
{
  description = "Hedera SDK (Rust)";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
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
