{pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
  frameworks = pkgs.darwin.apple_sdk.frameworks;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    buildInputs = [
      frameworks.Security
      frameworks.Cocoa
      frameworks.WebKit
      frameworks.Foundation
      frameworks.CoreFoundation
      frameworks.CoreServices
    ];
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
  }
