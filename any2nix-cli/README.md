<!--
SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>

SPDX-License-Identifier: CC-BY-4.0
-->

# [any2nix-cli](https://docs.rs/any2nix-cli)

> Nix Converter CLI

<p align="center">
  <img src="https://zipline.gs-101.dev/u/DZWGB0.svg" alt="Ferris the Rustacean, Rust's mascot, holding a file on its left claw and Nix's logo on its right claw" width="250">
</p>

<p align="center">
  <a href="https://github.com/gbr-ufs/any2nix/actions/workflows/ci.yaml"><img src="https://img.shields.io/github/actions/workflow/status/gbr-ufs/any2nix/ci.yaml?branch=main&label=CI" alt="CI Status"></a>
  <a href="https://crates.io/crates/any2nix-cli"><img src="https://img.shields.io/crates/v/any2nix-cli.svg?label=crates.io" alt="crates.io"></a>
  <a href="https://docs.rs/any2nix-cli"><img src="https://img.shields.io/docsrs/any2nix-cli.svg?label=Docs.rs" alt="Docs.rs"></a>
  <a href="https://www.gnu.org/licenses/gpl-3.0.html"><img src="https://img.shields.io/badge/License-GPLv3+-822422?logo=GNU&logoColor=black&labelColor=white" alt="License"></a>
  <a href="https://api.reuse.software/info/github.com/gbr-ufs/any2nix"><img src="https://api.reuse.software/badge/github.com/gbr-ufs/any2nix" alt="REUSE status"></a>
  <img src="https://img.shields.io/badge/Rust_edition-2024-dea584?logo=rust" alt="Rust Edition">
</p>

## What is any2nix-cli?

any2nix-cli is a command-line interface (CLI) for translating text and files from various configuration formats to [Nix](https://nixos.org/).

![GIF showcasing the program being used on the project's "rust-toolchain.toml"](https://vhs.charm.sh/vhs-13JxibIvMVVKmWS323SntU.gif)

<p align="center">
  <a href="https://vhs.charm.sh">
    <img alt="VHS" src="https://stuff.charm.sh/vhs/badge.svg">
  </a>
</p>

## Supported formats

any2nix-cli supports converting from the following formats into Nix expressions:

- [INI](https://en.wikipedia.org/wiki/INI_file)
- [JSON](https://www.json.org)
- [TOML](https://toml.io)
- [YAML](https://yaml.org)

## Installation

### Install with Cargo

```bash
cargo install any2nix-cli
```

### Install on macOS

macOS builds of any2nix-cli are available at the [releases tab](https://github.com/gbr-ufs/any2nix/releases/latest).

### Install on Linux

Look for your favorite package manager in the list of supported repositories:

[![Repository status](https://repology.org/badge/repository-big/any2nix-cli.svg)](https://repology.org/repository/any2nix-cli)

Additionally, Linux builds are available at the [releases tab](https://github.com/gbr-ufs/any2nix/releases).

### Install on Windows

Windows builds of any2nix-cli are available at the [releases tab](https://github.com/gbr-ufs/any2nix/releases/latest).

### Run using Docker

A docker image is available for any2nix-cli.

```bash
docker run --rm -it -v $PWD:/app ghcr.io/gbr-ufs/any2nix-cli:latest -f toml /app/rust-toolchain.toml
```

### Agent Skill

You can install the `any2nix` skill into agent harnesses:

```sh
claude plugin marketplace add gbr-ufs/any2nix
claude plugin install any2nix@gbr-ufs-any2nix
```

## Crate features

All format parsers are enabled by default. You can disable default features and enable only the formats or integrations you need:

| Feature | Description                         | Default |
|---------|-------------------------------------|---------|
| `ini`   | Support for converting INI strings  | Yes     |
| `json`  | Support for converting JSON strings | Yes     |
| `toml`  | Support for converting TOML strings | Yes     |
| `yaml`  | Support for converting YAML strings | Yes     |

## License

All code is licensed [GPL 3.0 or later](https://www.gnu.org/licenses/gpl-3.0.html). The distribution workflow is based on [cargo-dist](https://axodotdev.github.io/cargo-dist) and is thus licensed [MIT](https://opensource.org/license/MIT) or [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt).

Anything that is not code is [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/legalcode.txt).

This software family is [REUSE](https://reuse.software/) compliant.
