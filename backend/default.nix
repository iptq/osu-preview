{ lib, toolchain, makeRustPlatform, pkg-config, openssl }:

let
  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };

in rustPlatform.buildRustPackage {
  name = "osu-preview-backend";
  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
    outputHashes = {
      "libosu-0.0.28" = "sha256-idWA4c8y8+nu6yx7Pi2F/CliQlFzVikA83NXehD5Xrs=";
    };
  };

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];
}
