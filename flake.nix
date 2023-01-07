{
  description = "A flake to support development on XBI via NixOS";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustVersion = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };
        # rootBuild = rustPlatform.buildRustPackage {
        #   pname =
        #     "rust_nix_blog"; # make this what ever your cargo.toml package.name is
        #   version = "0.1.0";
        #   src = ./.; # the folder with the cargo.toml
        #   cargoLock.lockFile = ./Cargo.lock;
        #   # Add packages here, like openssl, clang, llvm etc
        #   buildInputs = [];
        # };
      in {
        # defaultPackage = devShell;
        devShell = pkgs.mkShell {
          nativeBuildInputs = [ pkgs.bashInteractive ];
          buildInputs =
            [ (rustVersion.override { extensions = [ "rust-src" ]; }) ];
        };
  });
}
