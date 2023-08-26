
{
  description = "A very basic flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.flake-utils.follows = "flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, ... }:
    let
      supportedSystems = [ "aarch64-darwin" ];
      forEachSystem = nixpkgs.lib.genAttrs supportedSystems;
    in {
      devShells = forEachSystem (system:
        let
          pkgs = nixpkgs.legacyPackages."${system}".extend (import rust-overlay);
          ruby = pkgs.ruby_3_2.override { inherit bundler bundix; };
          bundler = pkgs.bundler.override { inherit ruby; };
          bundix = pkgs.bundix.override { inherit bundler; };
          bundlerEnv = pkgs.bundlerEnv {
            inherit ruby bundler;
            name = "build-indexes-env";
            gemfile = ./build-indexes/Gemfile;
            lockfile = ./build-indexes/Gemfile.lock;
            gemset = ./build-indexes/gemset.nix;
          };
        in
          {
            default = pkgs.mkShell {
              name = "baroplanner";
              packages = with pkgs; [
                esbuild
                bundlerEnv.wrappedRuby
                cargo
                (rust-bin.stable.latest.default.override {
                  targets = [
                    "wasm32-unknown-unknown"
                  ];
                })
                rustfmt
                rust-analyzer
                rustPackages.clippy
                rustfilt
                trunk
              ];
            };

            bundix = pkgs.mkShell {
              name = "bundix";
              packages = [
                ruby
                bundler
                bundix
              ];
            };
          }
      );
    };
}
