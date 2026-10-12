<!--
SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>

SPDX-License-Identifier: CC-BY-4.0
-->

# [any2nix](https://docs.rs/any2nix)

> Serialization-powered Nix Converter

<p align="center">
  <img src="https://zipline.gs-101.dev/u/DZWGB0.svg" alt="Ferris the Rustacean, Rust's mascot, holding a file on its left claw and Nix's logo on its right claw" width="250">
</p>

<p align="center">
  <a href="https://github.com/gbr-ufs/any2nix/actions/workflows/ci.yaml"><img src="https://img.shields.io/github/actions/workflow/status/gbr-ufs/any2nix/ci.yaml?branch=main&label=CI" alt="CI Status"></a>
  <a href="https://www.gnu.org/licenses/gpl-3.0.html"><img src="https://img.shields.io/badge/License-GPLv3+-822422?logo=GNU&logoColor=black&labelColor=white" alt="License"></a>
  <a href="https://api.reuse.software/info/github.com/gbr-ufs/any2nix"><img src="https://api.reuse.software/badge/github.com/gbr-ufs/any2nix" alt="REUSE status"></a>
  <img src="https://img.shields.io/badge/Rust_edition-2024-dea584?logo=rust" alt="Rust Edition">
</p>

## What is any2nix?

any2nix is a Rust-based suite of conversion tools from multiple formats to [Nix](https://nixos.org/) based on [data serialization](https://docs.rs/serde).

## Why any2nix?

### New Nix users

any2nix is helpful for new Nix users who want to migrate their dotfiles, which are scattered across multiple configuration languages, to Nix. It provides a simple and efficient way to convert existing configurations into Nix expressions.

### Existing Nix users

Some programs like [Noctalia Shell](https://noctalia.dev/) have graphical configuration interfaces that support non-Nix formats. any2nix can be used to convert these configurations into Nix expressions, allowing users to integrate them into their Nix configuration.

## Multiple interfaces

any2nix provides multiple interfaces to fit different workflows:

- **Rust Library** ([any2nix](./any2nix/README.md)): serde-based Nix converter library.
- **JSON-based API** ([any2nix-api](./any2nix-api/README.md)): JSON-based Nix converter API.
  - **CLI** ([any2nix-cli](./any2nix-cli/README.md)): Nix converter CLI ([terminal recording](https://vhs.charm.sh/vhs-13JxibIvMVVKmWS323SntU.gif)).
  - **GUI** ([any2nix-gui](./any2nix-gui/README.md)): Nix converter GUI.
  - **Web App** ([any2nix-website](./any2nix-website/README.md)): Nix converter website.
- **Agent Skill** ([any2nix](./skills/any2nix/SKILL.md)): Nix converter skill for agent harnesses (Claude Code, Antigravity, etc.).

<p align="center">
  <a href="https://vhs.charm.sh/vhs-13JxibIvMVVKmWS323SntU.gif">
    <img src="https://vhs.charm.sh/vhs-13JxibIvMVVKmWS323SntU.gif" alt="GIF showcasing the CLI being used on rust-toolchain.toml">
  </a>
</p>

## Supported Formats

any2nix supports converting from the following formats into Nix expressions:

- [INI](https://en.wikipedia.org/wiki/INI_file)
- [JSON](https://www.json.org)
- [TOML](https://toml.io)
- [YAML](https://yaml.org)

## Crates

| Crate                                          | Description                                                   | `crates.io`                                                                                                   | Docs                                                                                             |
|------------------------------------------------|---------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------|--------------------------------------------------------------------------------------------------|
| [any2nix](./any2nix/README.md)                 | serde-based Nix converter library                             | [![crates.io](https://img.shields.io/crates/v/any2nix.svg)](https://crates.io/crates/any2nix)                 | [![docs.rs](https://img.shields.io/docsrs/any2nix.svg)](https://docs.rs/any2nix)                 |
| [any2nix-api](./any2nix-api/README.md)         | JSON-based Nix converter API                                  | [![crates.io](https://img.shields.io/crates/v/any2nix-api.svg)](https://crates.io/crates/any2nix-api)         | [![docs.rs](https://img.shields.io/docsrs/any2nix-api.svg)](https://docs.rs/any2nix-api)         |
| [any2nix-cli](./any2nix-cli/README.md)         | Nix converter CLI                                             | [![crates.io](https://img.shields.io/crates/v/any2nix-cli.svg)](https://crates.io/crates/any2nix-cli)         |                                                                                                  |
| [any2nix-gui](./any2nix-gui/README.md)         | Nix converter GUI                                             | [![crates.io](https://img.shields.io/crates/v/any2nix-gui.svg)](https://crates.io/crates/any2nix-gui)         |                                                                                                  |
| [any2nix-i18n](./any2nix-i18n/README.md)       | Internationalization (i18n) catalogs and utilities for any2nix |                                                                                                               |                                                                                                  |
| [any2nix-website](./any2nix-website/README.md) | Nix converter website                                         | [![crates.io](https://img.shields.io/crates/v/any2nix-website.svg)](https://crates.io/crates/any2nix-website) | [![docs.rs](https://img.shields.io/docsrs/any2nix-website.svg)](https://docs.rs/any2nix-website) |

## License

All code is licensed [GPL 3.0 or later](https://www.gnu.org/licenses/gpl-3.0.html). The distribution workflow is based on [cargo-dist](https://axodotdev.github.io/cargo-dist) and is thus licensed [MIT](https://opensource.org/license/MIT) or [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt).

This software family is [REUSE](https://reuse.software/) compliant.
