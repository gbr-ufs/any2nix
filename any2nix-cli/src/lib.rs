// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![doc(html_favicon_url = "https://zipline.gs-101.dev/u/DF6up7.ico")]
#![doc(html_logo_url = "https://zipline.gs-101.dev/u/DZWGB0.svg")]
//! This crate provides a command-line interface for translating different
//! formats to [Nix](https://nixos.org).
//!
//! # Supported Formats
//!
//! See [any2nix::Format] for an enumeration of supported formats.
//!
//! Each format is gated behind its own [feature](https://doc.rust-lang.org/cargo/reference/features.html).
//!
//! The default feature enables all formats.

use any2nix::Format;
use clap::Parser;
use clap_complete::Shell;

/// Generator of the program's flags.
#[derive(Parser, Debug)]
#[command(about, name = "any2nix", version)]
pub struct Cli {
    /// Generate shell completions for the given shell.
    #[arg(exclusive = true, long, value_enum)]
    pub completions: Option<Shell>,

    /// The format of the input data.
    #[arg(long, required_unless_present_any = ["completions", "man"], short, value_enum)]
    pub format: Option<Format>,

    /// The input file to convert. If missing or "-", reads from stdin.
    pub input: Option<String>,

    /// Generate a manpage in roff format.
    #[arg(exclusive = true, long)]
    pub man: bool,
}
