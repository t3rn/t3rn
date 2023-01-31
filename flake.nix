{
  description = "A flake to support reproducible development of t3rn";

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
      in {
        devShell = pkgs.mkShell {
          LIBCLANG_PATH = pkgs.libclang.lib + "/lib/";
          LD_LIBRARY_PATH = "${pkgs.stdenv.cc.cc.lib}/lib/:$LD_LIBRARY_PATH";

          stdenv = pkgs.clangStdenv;
          
          nativeBuildInputs = with pkgs; [ 
            bashInteractive
            taplo
            clang
            cmake
            openssl
            pkg-config
            protobuf
            sccache
            nodejs-slim
          ];
          buildInputs = with pkgs; [ 
              (rustVersion.override { extensions = [ "rust-src" ]; }) 
          ];
          
        };
  });
}
