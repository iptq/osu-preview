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
        myPkgs = rec { osu-preview = pkgs.callPackage ./. { }; };
      in rec {
        packages = flake-utils.lib.flattenTree myPkgs;
        defaultPackage = packages.osu-preview;
        devShell = pkgs.mkShell {
          CARGO_UNSTABLE_SPARSE_REGISTRY = "true";
          inputsFrom = with packages; [ osu-preview ];
          packages = with pkgs;
            with pkgs.fenix.minimal; [
              cargo
              rustc
              rustfmt
            ];
        };
      });
}
