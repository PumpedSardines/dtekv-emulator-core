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
      frameworks.WebKit
      frameworks.Foundation
      frameworks.AppKit
      pkgs.rustup
      pkgs.cmake
      pkgs.openssl
      pkgs.pkg-config
      pkgs.libiconv
      pkgs.libwebview
      pkgs.clang
    ];
    shellHook = ''
      export PS1="[$name] \[$txtgrn\]\u@\h\[$txtwht\]:\[$bldpur\]\w \[$txtcyn\]\$git_branch\[$txtred\]\$git_dirty \[$bldylw\]\$aws_env\[$txtrst\]\$ "
      export NIX_LDFLAGS="-F${frameworks.CoreFoundation}/Library/Frameworks -framework CoreFoundation $NIX_LDFLAGS";
    '';
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;
  }
