{
  description = "Tools to help me manage my Hyprland desktop";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };
  };

  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ ];
      systems = nixpkgs.lib.systems.flakeExposed;

      perSystem =
        {
          config,
          self',
          inputs',
          pkgs,
          system,
          ...
        }:
        {
          devShells.default = import ./nix/shell.nix { inherit pkgs; };
          packages = rec {
            default = pkgs.callPackage ./nix/package.nix { };
            hyprdock = default.overrideAttrs {
              cargoBuildFlags = [
                "--bin"
                "noct"
              ];
            };
          };
        };
    };
}
