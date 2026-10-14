<!--
SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>

SPDX-License-Identifier: CC-BY-4.0
-->

# [any2nix-gui](https://docs.rs/any2nix-gui)

> Nix Converter GUI

<p align="center">
  <img src="https://zipline.gs-101.dev/u/DZWGB0.svg" alt="Ferris the Rustacean, Rust's mascot, holding a file on its left claw and Nix's logo on its right claw" width="250">
</p>

<p align="center">
  <a href="https://github.com/gbr-ufs/any2nix/actions/workflows/ci.yaml"><img src="https://img.shields.io/github/actions/workflow/status/gbr-ufs/any2nix/ci.yaml?branch=main&label=CI" alt="CI Status"></a>
  <a href="https://crates.io/crates/any2nix-gui"><img src="https://img.shields.io/crates/v/any2nix-gui.svg?label=crates.io" alt="crates.io"></a>
  <a href="https://www.gnu.org/licenses/gpl-3.0.html"><img src="https://img.shields.io/badge/License-GPLv3+-822422?logo=GNU&logoColor=black&labelColor=white" alt="License"></a>
  <a href="https://api.reuse.software/info/github.com/gbr-ufs/any2nix"><img src="https://api.reuse.software/badge/github.com/gbr-ufs/any2nix" alt="REUSE status"></a>
  <img src="https://img.shields.io/badge/Rust_edition-2024-dea584?logo=rust" alt="Rust Edition">
</p>

## What is any2nix-gui?

any2nix-gui is a graphical user interface (GUI) used to convert data in other formats to [Nix](https://nixos.org/).

![Screenshot showcasing the GTK-based UI, with sample syntax-highlighted INI text ready for conversion](https://zipline.gs-101.dev/u/BuZP2y.png)

## Supported formats

any2nix-gui supports converting from the following formats into Nix expressions:

- [INI](https://en.wikipedia.org/wiki/INI_file)
- [JSON](https://www.json.org)
- [TOML](https://toml.io)
- [YAML](https://yaml.org)


## Installation

> [!NOTE]
> Requires GTK 4, libadwaita and GtkSourceView 5 development libraries installed on the system.

### Install with Cargo

```bash
cargo install any2nix-gui
```

### Install on Linux

Look for your favorite package manager in the list of supported repositories:

[![Repository status](https://repology.org/badge/repository-big/any2nix-gui.svg)](https://repology.org/repository/any2nix-gui)

Additionally, Linux builds are available at the [releases tab](https://github.com/gbr-ufs/any2nix/releases).

### Run using Docker

A docker image is available for any2nix-gui.

```bash
docker run --rm -it \
  -e WAYLAND_DISPLAY=$WAYLAND_DISPLAY \
  -e XDG_RUNTIME_DIR=/tmp \
  -v $XDG_RUNTIME_DIR/$WAYLAND_DISPLAY:/tmp/$WAYLAND_DISPLAY:ro \
  --device /dev/dri \
  -v $PWD:/app \
  ghcr.io/gbr-ufs/any2nix-gui:latest
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
