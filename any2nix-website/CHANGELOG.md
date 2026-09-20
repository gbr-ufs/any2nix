# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.4](https://github.com/gbr-ufs/any2nix/compare/any2nix-website-v0.1.3...any2nix-website-v0.1.4) - 2026-09-20

### Fixed

- add .gitkeep file to ensure any2nix-website compiles even without bun
- allow dirty status for publishing any2nix-website

## [0.1.3](https://github.com/gbr-ufs/any2nix/releases/tag/any2nix-website-v0.1.3) - 2026-09-20

### Added

- implement internationalization crate
- *(highlighting)* add line numbers and rainbow delimiters to output
- add noscript tag
- implement website

### Fixed

- *(lib.rs)* remove unecessary braces from use statement
- *(lib.rs)* remove some unused imports

### Other

- add crate readmes
- pivot to VariantArray instead of EnumIter
- update address to project icons
- *(images)* regenerate website images
- *(editor.js)* remove unecessary options
- *(docker)* unify docker setup
- streamline supported formats section
- try to be idiomatic with some use statements
