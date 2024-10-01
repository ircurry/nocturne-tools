{
  description = "Tools to help me manage my Hyprland desktop";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
    };

    devenv = {
      url = "github:cachix/devenv";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    inputs@{ flake-parts, nixpkgs, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [ inputs.devenv.flakeModule ];
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
          devenv.shells.default = {
            # packages = with pkgs; [ ];
            languages.nix.enable = true;
            languages.rust.enable = true;
          };

          packages = rec {
            hyprdock = pkgs.callPackage ./nix/packages/hyprdock.nix { };
            default = hyprdock;
          };
        };
    };
}
