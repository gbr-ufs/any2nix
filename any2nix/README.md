<!--
SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>

SPDX-License-Identifier: CC-BY-4.0
-->

# [any2nix](https://docs.rs/any2nix)

> [serde](https://serde.rs/)-based Nix Converter Library

<p align="center">
  <img src="https://zipline.gs-101.dev/u/DZWGB0.svg" alt="Ferris the Rustacean, Rust's mascot, holding a file on its left claw and Nix's logo on its right claw" width="250">
</p>

<p align="center">
  <a href="https://github.com/gbr-ufs/any2nix/actions/workflows/ci.yaml"><img src="https://img.shields.io/github/actions/workflow/status/gbr-ufs/any2nix/ci.yaml?branch=main&label=CI" alt="CI Status"></a>
  <a href="https://crates.io/crates/any2nix"><img src="https://img.shields.io/crates/v/any2nix.svg?label=crates.io" alt="crates.io"></a>
  <a href="https://docs.rs/any2nix"><img src="https://img.shields.io/docsrs/any2nix.svg?label=Docs.rs" alt="Docs.rs"></a>
  <a href="https://www.gnu.org/licenses/gpl-3.0.html"><img src="https://img.shields.io/badge/License-GPLv3+-822422?logo=GNU&logoColor=black&labelColor=white" alt="License"></a>
  <a href="https://api.reuse.software/info/github.com/gbr-ufs/any2nix"><img src="https://api.reuse.software/badge/github.com/gbr-ufs/any2nix" alt="REUSE status"></a>
  <img src="https://img.shields.io/badge/Rust_edition-2024-dea584?logo=rust" alt="Rust Edition">
</p>

## What is any2nix?

any2nix is a [serde](https://serde.rs/)-powered Rust library that offers high-level functions to translate strings in various file formats to [Nix](https://nixos.org/).

## Supported formats

any2nix supports converting from the following formats into Nix expressions:

- [INI](https://en.wikipedia.org/wiki/INI_file)
- [JSON](https://www.json.org)
- [TOML](https://toml.io)
- [YAML](https://yaml.org)

## Getting started

This crate is on [crates.io](https://crates.io/crates/any2nix) and can be added as a dependency of your project:

```bash
cargo add any2nix
```

## Quick look

```rust
use any2nix::Format;

let some_toml = r#"
[package]
name = "any2nix"
"#;

// Format-specific functions.
let nix = any2nix::toml_to_nix(some_toml)?;
// Using the Format enum.
let nix_from_format = Format::Toml.to_nix(some_toml)?;

println!("{nix}");
// Output:
// {
//   package = {
//     name = "any2nix";
//   };
// }
```

## Crate features

All format parsers are enabled by default. You can disable default features and enable only the formats or integrations you need:

| Feature  | Description                                             | Default |
|----------|---------------------------------------------------------|---------|
| `ini`    | Support for converting INI strings                      | Yes     |
| `json`   | Support for converting JSON strings                     | Yes     |
| `toml`   | Support for converting TOML strings                     | Yes     |
| `yaml`   | Support for converting YAML strings                     | Yes     |
| `clap`   | Derive `ValueEnum` on `Format` for CLI argument parsing | No      |
| `utoipa` | Derive `ToSchema` on `Format` for OpenAPI documentation | No      |

## License

All code is licensed [GPL 3.0 or later](https://www.gnu.org/licenses/gpl-3.0.html). The distribution workflow is based on [cargo-dist](https://axodotdev.github.io/cargo-dist) and is thus licensed [MIT](https://opensource.org/license/MIT) or [Apache 2.0](https://www.apache.org/licenses/LICENSE-2.0.txt).

Anything that is not code is [CC-BY-4.0](https://creativecommons.org/licenses/by/4.0/legalcode.txt).

This software family is [REUSE](https://reuse.software/) compliant.
