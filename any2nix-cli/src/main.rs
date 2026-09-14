// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use any2nix_cli::Cli;
use clap::{CommandFactory, Parser};
use clap_mangen::Man;
use std::fs;
use std::io::{self, Read};
use std::process;

fn main() {
    let cli = Cli::parse();

    if let Some(shell) = cli.completions {
        let mut cmd = Cli::command();
        let name = cmd.get_name().to_string();

        clap_complete::generate(shell, &mut cmd, name, &mut io::stdout());

        return;
    }

    if cli.man {
        let cmd = Cli::command();

        if let Err(e) = Man::new(cmd).render(&mut io::stdout()) {
            eprintln!("{}: {}", env!("CARGO_CRATE_NAME"), e);

            process::exit(1);
        }

        return;
    }

    let mut input = String::new();
    let input_source = cli.input.as_deref().unwrap_or("-");

    match cli.input.as_deref() {
        Some(path) if path != "-" => {
            input = fs::read_to_string(path).unwrap_or_else(|e| {
                eprintln!("{}: {}: {}", env!("CARGO_CRATE_NAME"), path, e);

                process::exit(1);
            });
        }
        _ => {
            io::stdin().read_to_string(&mut input).unwrap_or_else(|e| {
                eprintln!("{}: {}", env!("CARGO_CRATE_NAME"), e);

                process::exit(1);
            });
        }
    }

    let format = cli
        .format
        .expect("format is required when not generating completions or man pages");

    match format.to_nix(&input) {
        Ok(nix) => println!("{}", nix),
        Err(e) => {
            if input_source == "-" {
                eprintln!("{}: {}", env!("CARGO_CRATE_NAME"), e);
            } else {
                eprintln!("{}: {}: {}", env!("CARGO_CRATE_NAME"), input_source, e);
            }

            process::exit(1);
        }
    }
}
