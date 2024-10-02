{
  lib,
  rustPlatform,
  makeWrapper,
  hyprland,
}: rustPlatform.buildRustPackage rec {
  pname = "hyprdock";
  version = "0.1.0";
  env.NIX_RELEASE_VERSION = version;

  src = ../.;

  cargoLock.lockFile = ../Cargo.lock;

  nativeBuildInputs = [
    makeWrapper
  ];

  postFixup = ''
    wrapProgram $out/bin/hyprdock --prefix PATH : ${lib.makeBinPath [hyprland]}
  '';

  meta = {
    description = "hyprdock sets hyprland monitor configuration according to a profile";
  };
}
