<!--
SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>

SPDX-License-Identifier: CC-BY-4.0
-->

# [any2nix-website](https://docs.rs/any2nix-website)

> Nix Converter Website

<p align="center">
  <img src="https://zipline.gs-101.dev/u/DZWGB0.svg" alt="Ferris the Rustacean, Rust's mascot, holding a file on its left claw and Nix's logo on its right claw" width="250">
</p>

<p align="center">
  <a href="https://github.com/gbr-ufs/any2nix/actions/workflows/ci.yaml"><img src="https://img.shields.io/github/actions/workflow/status/gbr-ufs/any2nix/ci.yaml?branch=main&label=CI" alt="CI Status"></a>
  <a href="https://crates.io/crates/any2nix-website"><img src="https://img.shields.io/crates/v/any2nix-website.svg?label=crates.io" alt="crates.io"></a>
  <a href="https://www.gnu.org/licenses/gpl-3.0.html"><img src="https://img.shields.io/badge/License-GPLv3+-822422?logo=GNU&logoColor=black&labelColor=white" alt="License"></a>
  <a href="https://api.reuse.software/info/github.com/gbr-ufs/any2nix"><img src="https://api.reuse.software/badge/github.com/gbr-ufs/any2nix" alt="REUSE status"></a>
  <img src="https://img.shields.io/badge/Rust_edition-2024-dea584?logo=rust" alt="Rust Edition">
</p>

## What is any2nix-website?

any2nix-website is a web application used to translate from various formats to [Nix](https://nixos.org/).

<p align="center">
  <img src="https://zipline.gs-101.dev/u/1z48PH.png" alt="Screenshot showcasing the Material Design inspired interface.">
</p>

## Supported formats

any2nix-website supports converting from the following formats into Nix expressions:

- [INI](https://en.wikipedia.org/wiki/INI_file)
- [JSON](https://www.json.org)
- [TOML](https://toml.io)
- [YAML](https://yaml.org)

## Installation

> [!NOTE]
> Environment variables `ANY2NIX_WEBSITE_HOST` and `ANY2NIX_WEBSITE_PORT` are available for setting the host address and port, respectively.

### Install with Cargo

```bash
cargo install any2nix-website
```

### Install on macOS

macOS builds of any2nix-website are available at the [releases tab](https://github.com/gbr-ufs/any2nix/releases/latest).

### Install on Linux

Look for your favorite package manager in the list of supported repositories:

[![Repository status](https://repology.org/badge/repository-big/any2nix-website.svg)](https://repology.org/repository/any2nix-website)

Additionally, Linux builds are available at the [releases tab](https://github.com/gbr-ufs/any2nix/releases).

### Install on Windows

Windows builds of any2nix-website are available at the [releases tab](https://github.com/gbr-ufs/any2nix/releases/latest).

### Run using Docker

A docker image is available for any2nix-website.

```bash
docker run --rm -it -p 3000:3000 ghcr.io/gbr-ufs/any2nix-website:latest
```

After running this, the website will be available on <http://localhost:3000>.

## Building

This crate bundles frontend assets into the `dist/` directory before embedding them into the Rust application binary.

### 1. Install frontend dependencies

Run one of the following depending on your preferred package manager:

```bash
# Bun
bun install --frozen-lockfile

# Deno
deno install

# npm
npm install

# pnpm
pnpm install

# Yarn
yarn install
```

### 2. Bundle frontend assets

```bash
# Using Bun (built-in bundler):
bun build assets/js/index.js --outdir dist --minify
bun build assets/css/index.css --outdir dist --minify

# Using npx / pnpm dlx (via esbuild):
npx esbuild assets/js/index.js assets/css/index.css --outdir=dist --bundle --minify --loader:.woff2=file
```

### 3. Build the Rust binary

```bash
cargo build --release
```

When building through Nix (`nix build .#any2nix-website`), dependencies and bundled assets are handled automatically during the build phase.

## Crate features

All format parsers are enabled by default. You can disable default features and enable only the formats or integrations you need:

| Feature             | Description                                                                                    | Default |
|---------------------|------------------------------------------------------------------------------------------------|---------|
| `ini`               | Support for converting INI strings                                                             | Yes     |
| `json`              | Support for converting JSON strings                                                            | Yes     |
| `toml`              | Support for converting TOML strings                                                            | Yes     |
| `yaml`              | Support for converting YAML strings                                                            | Yes     |
| `api`               | Enables the `/api` endpoint for headless access via [any2nix-api](https://docs.rs/any2nix-api) | No      |
| `trace`             | Support for [tracing](https://en.wikipedia.org/wiki/Tracing_(software))                        | No      |
| `utoipa`            | Support for [OpenAPI](https://www.openapis.org/)                                               | No      |
| `utoipa-swagger-ui` | User interface for the API at the `/docs` endpoint                                             | No      |

## License

All code is licensed [GPL 3.0 or later](https://www.gnu.org/licenses/gpl-3.0.html). The distribution workflow is based on [cargo-dist](https://axodotdev.github.io/cargo-dist) and is thus licensed [MIT](https://opensource.org/license/MIT) or [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt).

Anything that is not code is [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/legalcode.txt).

This software family is [REUSE](https://reuse.software/) compliant.
