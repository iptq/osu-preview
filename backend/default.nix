{ toolchain, makeRustPlatform, pkg-config, openssl }:

let
  rustPlatform = makeRustPlatform {
    cargo = toolchain;
    rustc = toolchain;
  };

in rustPlatform.buildRustPackage {
  name = "osu-preview-backend";
  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [ pkg-config ];
  buildInputs = [ openssl ];
}
