{
  description = "signal-spirit-judge — typed Spirit judge request and reply contract";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-build = {
      url = "github:LiGoldragon/rust-build";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, flake-utils, rust-build }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rust = rust-build.lib.${system}.fromToolchainFile pkgs {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-gh/xTkxKHL4eiRXzWv8KP7vfjSk61Iq48x47BEDFgfk=";
        };
        inherit (rust) craneLib toolchain;
        src = rust.cleanSource {
          root = ./.;
          extraFilters = [
            (path: type:
              type == "directory" && baseNameOf path == "schema"
              || type == "regular" && baseNameOf path == "signal.schema")
          ];
        };
        commonArgs = { inherit src; strictDeps = true; };
        cargoArtifacts = craneLib.buildDepsOnly commonArgs;
        checkArgs = commonArgs // {
          inherit cargoArtifacts;
          doInstallCargoArtifacts = false;
        };
      in
      {
        packages.default = craneLib.buildPackage (commonArgs // { inherit cargoArtifacts; });
        checks = {
          build = craneLib.cargoBuild checkArgs;
          test = craneLib.cargoTest (checkArgs // {
            cargoTestExtraArgs = "--all-features";
          });
          fmt = craneLib.cargoFmt (commonArgs // { doInstallCargoArtifacts = false; });
          clippy = craneLib.cargoClippy (checkArgs // {
            cargoClippyExtraArgs = "--all-targets --all-features -- -D warnings";
          });
          doc = craneLib.mkCargoDerivation (checkArgs // {
            pname = "signal-spirit-judge-doc";
            buildPhaseCargoCommand = "cargo doc --workspace --all-features --no-deps";
          });
        };
        devShells.default = pkgs.mkShell {
          name = "signal-spirit-judge";
          packages = [ pkgs.jujutsu pkgs.pkg-config toolchain ];
        };
      });
}
