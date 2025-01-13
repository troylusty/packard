{
  description = "Packard is a simple RSS aggregator meant to allow you to take a quick glance at what's occurring in topics you care about.";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      packages = {
        packard = let
          manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
        in
          pkgs.rustPlatform.buildRustPackage {
            pname = manifest.name;
            version = manifest.version;

            cargoLock.lockFile = ./Cargo.lock;

            src = pkgs.lib.cleanSource ./.;

            nativeBuildInputs = [pkgs.pkg-config];
            buildInputs = [pkgs.openssl];
          };
        default = self.packages.${system}.packard;
      };
    })
    // {
      overlays.default = final: prev: {
        inherit (self.packages.${final.system}) packard;
      };
    };
}
