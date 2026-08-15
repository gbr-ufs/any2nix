# SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
#
# SPDX-License-Identifier: GPL-3.0-or-later

{
  description = "Serialization-powered Nix Converter.";
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:nixos/nixpkgs/master";
  };
  outputs = { rust-overlay, nixpkgs, self, ... }:
  let
    inherit (nixpkgs) lib;
    forAllSystems = lib.genAttrs lib.systems.flakeExposed;
  in
  {
    devShells = forAllSystems (
      system:
      let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {inherit overlays system;};
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
      in
      {
        default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # openssl
          ];
          packages = with pkgs; [
            actionlint
            bun
            cargo-audit
            cargo-llvm-cov
            cocogitto
            htmx-lsp
            mdbook
            nixd
            # pkg-config
            release-plz
            reuse
            rustToolchain
            taplo
            vscode-langservers-extracted
            zizmor
          ];
        };
      }
    );
    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt);
  };
}
