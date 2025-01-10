{
  description = "Rust shell";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            cacert
            openssl
            pkg-config
            rust-bin.stable.latest.default
            protobuf
            rust-analyzer
          ];
      
        shellHook = ''
          export CARGO_HOME="$PWD/.cargo"
          export PATH="$PWD/.cargo/bin:$PATH"
        '';
      };
    }
  );
}
