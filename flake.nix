{
  description = "rust development flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let pkgs = nixpkgs.legacyPackages.${system}; in
      {
        devShells = {
          default = pkgs.mkShell {
            buildInputs = with pkgs; [
              cargo
              clippy
              rustc
              rustfmt
              rust-analyzer
              taplo
            ];
            
            shellHook = ''
              export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
            '';
          };
        };
      }
    );
}
