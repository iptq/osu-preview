{
  description = "A very basic flake";

  inputs = { napalm.url = "github:nix-community/napalm"; };

  outputs = { self, nixpkgs, flake-utils, napalm, fenix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ napalm.overlay fenix.overlay ];
        };
        toolchain = with fenix.packages.${system};
          combine [
            minimal.cargo
            minimal.rustc
            targets.wasm32-unknown-unknown.latest.rust-std
          ];
        myPkgs = rec {
          backend = pkgs.callPackage ./backend { inherit toolchain; };
          osu-preview = pkgs.callPackage ./. { };
        };
      in rec {
        packages = flake-utils.lib.flattenTree myPkgs;
        defaultPackage = packages.osu-preview;
        devShell = pkgs.mkShell {
          CARGO_UNSTABLE_SPARSE_REGISTRY = "true";
          inputsFrom = with packages; [ backend osu-preview ];
          packages = (with pkgs; [ wasm-pack toolchain cargo-edit ]);
        };
      });
}
