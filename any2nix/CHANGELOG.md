# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.1](https://github.com/gbr-ufs/any2nix/compare/any2nix-v0.1.0...any2nix-v0.1.1) - 2026-09-20

### Other

- release v0.1.0

## [0.1.0](https://github.com/gbr-ufs/any2nix/releases/tag/any2nix-v0.1.0) - 2026-09-20

### Added

- strip prefix from error messages
- add clap feature
- *(lib.rs)* use yaml_sede::Value instead of serde_json::Value in yaml function
- implement format conversion library

### Other

- *(lib.rs)* increase test coverage
- add crate readmes
- pivot to VariantArray instead of EnumIter
- update address to project icons
- document features interaction for Format
- document clap feature
- streamline supported formats section
- modularize any2nix-api
- *(lib.rs)* use my standard spacing scheme
- *(lib.rs)* encourage use of cargo add instead of manually editing Cargo.toml
- *(lib.rs)* standardize error messages
- *(lib.rs)* assorted documentation fixes
- *(lib.rs)* remove nix error
- fix authors key
- *(examples)* remove unwrap calls
- remove unidiomatic "is_ok()" asserts
- remove unecessary backticks
- *(package)* add extra metadata
- add branding
- make Nix error docstring private
- polish docstrings
- fix error path
- initial commit
