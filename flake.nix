{
  description = "Application to track time spent doing things";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:

    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      with pkgs;
      {
        devShells.default = mkShell {

          buildInputs = [
            rustup
            typos
            valgrind
            lldb
            mold
            openssl
            pkg-config
          ];

          shellHook = ''
            echo Entered Rust dev shell for project Stattrak
          '';

        };
      }
    );
}
