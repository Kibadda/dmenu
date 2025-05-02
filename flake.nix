{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    let
      overlay = final: prev: {
        kibadda = (prev.kibadda or { }) // {
          dmenu = final.pkgs.rustPlatform.buildRustPackage {
            name = "dmenu";
            cargoHash = "sha256-CTd5cqxbVBtMnB9UXIVrUcYwllRsGA8eiidj7DMS7X0=";
            src = self;
          };
        };
      };

      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
    in
    flake-utils.lib.eachSystem supportedSystems (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [
            overlay
          ];
        };
      in
      {
        packages = rec {
          default = dmenu;
          inherit (pkgs.kibadda) dmenu;
        };

        devShells = {
          default = pkgs.mkShell {
            name = "dmenu-development-shell";
            buildInputs = with pkgs; [
              cargo
              rustc
              rustfmt
              rustPackages.clippy
            ];
            RUST_SRC_PATH = pkgs.rustPlatform.rustLibSrc;
          };
        };
      }
    )
    // {
      overlays.default = overlay;
    };
}
