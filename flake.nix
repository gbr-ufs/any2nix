# SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
#
# SPDX-License-Identifier: GPL-3.0-or-later

{
  description = "Serialization-powered Nix Converter.";
  nixConfig = {
    extra-substituters = [ "https://any2nix.cachix.org" ];
    extra-trusted-public-keys = [
      "any2nix.cachix.org-1:NbpxEAtAuoVbp5CnyC5tjTi4WUsfvACee3Ffricnqgk="
    ];
  };
  inputs = {
    rust-overlay.url = "github:oxalica/rust-overlay";
    nixpkgs.url = "github:nixos/nixpkgs/master";
  };
  outputs =
    {
      rust-overlay,
      nixpkgs,
      self,
      ...
    }:
    let
      inherit (nixpkgs) lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      packages = forAllSystems (
        system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit overlays system; };
          mkDockerImage = {
            pname,
            package,
            binName ? pname,
            exposedPort ? null,
          }:
          pkgs.dockerTools.streamLayeredImage {
            config = {
              Entrypoint = [ "${package}/bin/${binName}" ];
              Env = [
                "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt"
              ];
            } // lib.optionalAttrs (exposedPort != null) {
              ExposedPorts = {
                "${toString exposedPort}/tcp" = { };
              };
            };
            contents = [
              package
              pkgs.cacert
            ];
            name = pname;
            tag = "dev";
          };
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          rustPlatform = pkgs.makeRustPlatform {
            cargo = rustToolchain;
            rustc = rustToolchain;
          };
        in
        {
          any2nix-api = rustPlatform.buildRustPackage {
            pname = "any2nix-api";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildAndTestSubdir = "any2nix-api";
          };
          any2nix-cli = rustPlatform.buildRustPackage {
            pname = "any2nix-cli";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildAndTestSubdir = "any2nix-cli";
            nativeBuildInputs = [ pkgs.installShellFiles ];
            postInstall = ''
              $out/bin/any2nix --man > any2nix.1
              installManPage any2nix.1
              installShellCompletion --cmd any2nix \
                --bash <($out/bin/any2nix --completions bash) \
                --zsh <($out/bin/any2nix --completions zsh) \
                --fish <($out/bin/any2nix --completions fish)
            '';
          };
          any2nix-gui = rustPlatform.buildRustPackage {
            pname = "any2nix-gui";
            version = "0.1.0";
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
            buildAndTestSubdir = "any2nix-gui";
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.wrapGAppsHook4
            ];
            buildInputs = [
              pkgs.gtk4
              pkgs.gtksourceview5
              pkgs.libadwaita
            ];
            postInstall = ''
              install -Dm644 any2nix-gui/data/dev.gs-101.any2nix.desktop $out/share/applications/dev.gs-101.any2nix.desktop
              install -Dm644 any2nix-gui/assets/img/any2nix.svg $out/share/icons/hicolor/scalable/apps/dev.gs-101.any2nix.svg
            '';
          };
          any2nix-website =
            let
              node_modules = pkgs.stdenvNoCC.mkDerivation {
                pname = "any2nix-website-node_modules";
                version = "0.1.0";
                src = lib.cleanSourceWith {
                  src = ./any2nix-website;
                  filter =
                    path: _type:
                    let
                      base = baseNameOf path;
                    in
                    base == "package.json" || base == "bun.lock";
                };
                nativeBuildInputs = [ pkgs.bun ];
                dontConfigure = true;
                dontFixup = true;
                buildPhase = ''
                  export HOME=$TMPDIR
                  bun install --frozen-lockfile --no-progress
                '';
                installPhase = ''
                  mkdir -p $out
                  cp -r node_modules $out/
                '';
                outputHash = "sha256-dd/BG9ggQwKXkpqv/zoRVnekzrEE33lFZAN2FuLDLdE=";
                outputHashMode = "recursive";
              };
            in
            rustPlatform.buildRustPackage {
              pname = "any2nix-website";
              version = "0.1.0";
              src = ./.;
              cargoLock.lockFile = ./Cargo.lock;
              buildAndTestSubdir = "any2nix-website";
              preBuild = ''
                cp -r ${node_modules}/node_modules any2nix-website/
              '';
            };
          docker-any2nix-api = mkDockerImage {
            pname = "any2nix-api";
            package = self.packages.${system}.any2nix-api;
            exposedPort = 3000;
          };
          docker-any2nix-cli = mkDockerImage {
            pname = "any2nix-cli";
            package = self.packages.${system}.any2nix-cli;
            binName = "any2nix";
          };
          docker-any2nix-gui = mkDockerImage {
            pname = "any2nix-gui";
            package = self.packages.${system}.any2nix-gui;
          };
          docker-any2nix-website = mkDockerImage {
            pname = "any2nix-website";
            package = self.packages.${system}.any2nix-website;
            exposedPort = 3000;
          };
          default = self.packages.${system}.any2nix-cli;
        }
      );
      devShells = forAllSystems (
        system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs { inherit overlays system; };
          emacsSettings = pkgs.writeText "dir-locals.el" ''
            ((rust-mode     . ((eglot-workspace-configuration
                                . (:rust-analyzer
                                   (:check
                                    (:command "clippy"
                                     :allTargets t))))))
              (rust-ts-mode . ((eglot-workspace-configuration
                                . (:rust-analyzer
                                   (:check
                                    (:command "clippy"
                                     :allTargets t)))))))
          '';
          rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        in
        {
          default = pkgs.mkShell {
            packages = with pkgs; [
              actionlint
              bun
              cargo-audit
              cargo-llvm-cov
              cocogitto
              dockerfile-language-server
              gtk4
              gtksourceview5
              htmx-lsp
              libadwaita
              mdbook
              nixd
              pkg-config
              release-plz
              reuse
              rustToolchain
              skopeo
              taplo
              vscode-langservers-extracted
              zizmor
            ];
            shellHook = ''
              ln -sf ${emacsSettings} .dir-locals.el
            '';
          };
        }
      );
      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt);
    };
}
