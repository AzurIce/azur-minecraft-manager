{
  description = "azur-minecraft-manager";

  nixConfig = {
    extra-substituters = [
      "https://mirrors.ustc.edu.cn/nix-channels/store"
    ];
    trusted-substituters = [
      "https://mirrors.ustc.edu.cn/nix-channels/store"
    ];
  };


  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
    nur.url = github:nix-community/NUR;
  };

  outputs = { self, nur, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) (nur.overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust-tools = pkgs.rust-bin.nightly."2022-10-09".default.override {
          extensions = [ "rust-src" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            clang
            llvmPackages_16.bintools
            # openssl
            pkg-config
          ] ++ [
            rust-tools
          ] ++ (with pkgs.darwin.apple_sdk.frameworks; pkgs.lib.optionals pkgs.stdenv.isDarwin [
            System
            IOKit
            Security
            CoreFoundation
            AppKit
            WebKit
          ]);
        };
      }
    );
}
