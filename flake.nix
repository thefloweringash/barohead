{
  description = "A very basic flake";

  outputs = { self, nixpkgs }: 
    let 
      supportedSystems = [ "aarch64-darwin" ];
      forEachSystem = nixpkgs.lib.genAttrs supportedSystems;
    in {
      devShells = forEachSystem (system: 
        let 
          pkgs = nixpkgs.legacyPackages."${system}"; 
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
