{
  pkgs ? import <nixpkgs> { }
}:
pkgs.mkShell {
  packages = with pkgs; [
    hyprland
    cargo
    clippy
    rustc
    rustfmt
    rust-analyzer
    nixd
    nixfmt-rfc-style
  ];
}
