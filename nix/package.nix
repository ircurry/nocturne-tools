{
  lib,
  rustPlatform,
  makeWrapper,
  hyprland,
  compositor ? "hyprland"
}:
assert lib.assertOneOf "compositor" compositor [
  "hyprland"
];
rustPlatform.buildRustPackage rec {
  pname = "noct";
  version = "0.1.0";
  env.NIX_RELEASE_VERSION = version;

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    makeWrapper
  ];

  postFixup =
    let
      packages = [] ++ lib.optionals (compositor == "hyprland") [hyprland];
    in
    ''
      wrapProgram $out/bin/noct --prefix PATH : ${lib.makeBinPath packages}
    '';

  meta = {
    description = "manage my nocture nixos configurations";
  };
}
