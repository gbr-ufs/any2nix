// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

#![doc(html_favicon_url = "https://zipline.gs-101.dev/u/IRXrE8.ico")]
#![doc(html_logo_url = "https://zipline.gs-101.dev/u/2QuAGa.svg")]
//! This crate provides simple functions for translating different formats
//! to [Nix](https://nixos.org).
//!
//! They serve as higher-level options to
//! [serialization functions][serde].
//!
//! Due to being based on serialization, the translation process has
//! a chance to fail, primarily on invalid formatting.
//!
//! # Usage
//!
//! This crate is on [crates.io](https://crates.io/crates/any2nix) and can
//! be added as a dependency of your project:
//!
//! ```bash
//! cargo add any2nix
//! ```
//!
//! # Supported Formats
//!
//! See [Format] for an enumeration of supported formats.
//!
//! Each format is gated behind its own [feature](https://doc.rust-lang.org/cargo/reference/features.html).
//!
//! The default feature enables all formats.
//!
//! # Crate Features
//! Besides the features for each format, this crate also exposes the following
//! features:
//!
//! - `clap`: Enables command-line argument parsing through [clap].
//! - `utoipa`: Enables OpenAPI schema generation through [utoipa].
//!
//! # Examples: TOML
//!
//! ```rust
//! # #[cfg(feature = "toml")]
//! # {
//! let some_toml = r#"
//! [package]
//! name = "any2nix"
//! "#;
//! // Functions follow a clear "{format}_to_nix" naming convention.
//! let nix = match any2nix::toml_to_nix(some_toml) {
//!     Ok(v) => v,
//!     Err(e) => panic!("invalid TOML") // Returns a "Result<String, any2nix::Error>".
//! };
//!
//! println!("{}", nix);
//! // Output:
//! //
//! // {
//! //   package = {
//! //     name = "any2nix";
//! //   };
//! // }
//! # assert_eq!(nix, "{\n  package = {\n    name = \"any2nix\";\n  };\n}")
//! # }
//! ```

use std::fmt::Display;

#[cfg(feature = "clap")]
use clap::ValueEnum;
use serde::{Deserialize, Serialize};
use strum::{EnumIter, IntoEnumIterator, VariantArray};
#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Aggregator of all errors from all used serializers (translators)
/// for simpler error handling.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[cfg(feature = "ini")]
    #[error("{}", match .0 {
        serde_ini::de::Error::Custom(msg) => msg.strip_prefix("INI syntax error: ").unwrap_or(msg),
        serde_ini::de::Error::UnexpectedEof => "unexpected end of file",
        serde_ini::de::Error::InvalidState => "invalid state",
    })]
    Ini(#[from] serde_ini::de::Error),
    #[cfg(feature = "json")]
    #[error("{0}")]
    Json(#[from] serde_json::Error),
    #[cfg(feature = "toml")]
    #[error("{}", .0.message())]
    Toml(#[from] toml::de::Error),
    #[cfg(feature = "yaml")]
    #[error("{0}")]
    Yaml(#[from] yaml_serde::Error),
}

/// Enumeration of the currently supported formats for conversion.
///
/// # Crate Features
///
/// With `clap` enabled, it can be used to limit possible values for a flag.
///
/// With `utoipa` enabled,generates an [OpenAPI](https://www.openapis.org/)-compatible
/// schema to describe a value.
#[derive(Clone, Copy, Debug, Deserialize, EnumIter, Eq, PartialEq, Serialize, VariantArray)]
#[serde(rename_all = "lowercase")]
#[cfg_attr(feature = "clap", derive(ValueEnum))]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub enum Format {
    #[cfg(feature = "ini")]
    Ini,
    #[cfg(feature = "json")]
    Json,
    #[cfg(feature = "toml")]
    Toml,
    #[cfg(feature = "yaml")]
    Yaml,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            #[cfg(feature = "ini")]
            Self::Ini => write!(f, "INI"),
            #[cfg(feature = "json")]
            Self::Json => write!(f, "JSON"),
            #[cfg(feature = "toml")]
            Self::Toml => write!(f, "TOML"),
            #[cfg(feature = "yaml")]
            Self::Yaml => write!(f, "YAML"),
        }
    }
}

impl Format {
    pub fn iter() -> impl Iterator<Item = Self> {
        <Self as IntoEnumIterator>::iter()
    }

    pub fn to_nix(&self, input: &str) -> Result<String, Error> {
        match self {
            #[cfg(feature = "ini")]
            Self::Ini => ini_to_nix(input),
            #[cfg(feature = "json")]
            Self::Json => json_to_nix(input),
            #[cfg(feature = "toml")]
            Self::Toml => toml_to_nix(input),
            #[cfg(feature = "yaml")]
            Self::Yaml => yaml_to_nix(input),
        }
    }
}

/// Wrapper around deserializer functions to have them serialize the formats to Nix.
fn format_to_nix<T, E>(
    from_str: impl FnOnce(&str) -> Result<T, E>,
    input: &str,
) -> Result<String, Error>
where
    T: Serialize,
    Error: From<E>,
{
    let format = from_str(input)?;
    let nix = ser_nix::to_string(&format).expect("AST to Nix is infallible");

    Ok(nix)
}

/// Translates text in [INI](https://en.wikipedia.org/wiki/INI_file) to [Nix](https://nixos.org).
///
/// # Errors
///
/// Returns [Error::Ini] in case the input cannot be parsed as valid INI.
///
/// # Examples
///
/// ```rust
/// let some_ini = "[Actor]
/// bUseNavMeshForMovement=1
/// fNotVisibleNavmeshMoveDist=2048.0000";
/// let nix = any2nix::ini_to_nix(&some_ini)?;
///
/// println!("{}", nix);
/// // Output:
/// //
/// // {
/// //   Actor = {
/// //     bUseNavMeshForMovement = "1";
/// //     fNotVisibleNavmeshMoveDist = "2048.0000";
/// //   };
/// // }
/// # assert_eq!(nix, "{\n  Actor = {\n    bUseNavMeshForMovement = \"1\";\n    fNotVisibleNavmeshMoveDist = \"2048.0000\";\n  };\n}");
/// # Ok::<(), any2nix::Error>(())
/// ```
#[cfg(feature = "ini")]
pub fn ini_to_nix(input: &str) -> Result<String, Error> {
    format_to_nix(serde_ini::from_str::<serde_json::Value>, input)
}

/// Translates text in [JSON](https://www.json.org) to [Nix](https://nixos.org).
///
/// # Errors
///
/// Returns [Error::Json] in case the input cannot be parsed as valid JSON.
///
/// # Examples
///
/// ```rust
/// let some_json = r#"
/// {
///     "name": "any2nix",
///     "version": "0.1.0"
/// }"#;
/// let nix = any2nix::json_to_nix(&some_json)?;
///
///
/// println!("{}", nix);
/// // Output:
/// //
/// // {
/// //   name = "any2nix";
/// //   version = "0.1.0";
/// // }
/// # assert_eq!(nix, "{\n  name = \"any2nix\";\n  version = \"0.1.0\";\n}");
/// # Ok::<(), any2nix::Error>(())
/// ```
#[cfg(feature = "json")]
pub fn json_to_nix(input: &str) -> Result<String, Error> {
    format_to_nix(|s| serde_json::from_str::<serde_json::Value>(s), input)
}

/// Translates text in [TOML](https://toml.io) to [Nix](https://nixos.org).
///
/// # Errors
///
/// Returns [Error::Toml] in case the input cannot be parsed as valid TOML.
///
/// # Examples
///
/// ```rust
/// let some_toml = r#"
/// [package]
/// name = "any2nix"
/// "#;
/// let nix = any2nix::toml_to_nix(&some_toml)?;
///
/// println!("{}", nix);
/// // Output:
/// //
/// // {
/// //   package = {
/// //     name = "any2nix";
/// //   };
/// // }
/// # assert_eq!(nix, "{\n  package = {\n    name = \"any2nix\";\n  };\n}");
/// # Ok::<(), any2nix::Error>(())
/// ```
#[cfg(feature = "toml")]
pub fn toml_to_nix(input: &str) -> Result<String, Error> {
    format_to_nix(|s| toml::from_str::<toml::Value>(s), input)
}

/// Translates text in [YAML](https://yaml.org) to [Nix](https://nixos.org).
///
/// # Errors
///
/// Returns [Error::Yaml] in case the input cannot be parsed as valid YAML.
///
/// # Examples
///
/// ```rust
/// let some_yaml = r#"
/// services:
///   db:
///     image: postgres:16-alpine
///     ports:
///       - "5432:5432"
///     restart: always
/// "#;
/// let nix = any2nix::yaml_to_nix(&some_yaml)?;
///
/// println!("{}", nix);
/// // Output:
/// //
/// // {
/// //   services = {
/// //     db = {
/// //       image = "postgres:16-alpine";
/// //       ports = [
/// //         "5432:5432"
/// //       ];
/// //       restart = "always";
/// //     };
/// //   };
/// // }
/// # assert_eq!(nix, "{\n  services = {\n    db = {\n      image = \"postgres:16-alpine\";\n      ports = [\n        \"5432:5432\"\n      ];\n      restart = \"always\";\n    };\n  };\n}");
/// # Ok::<(), any2nix::Error>(())
/// ```
#[cfg(feature = "yaml")]
pub fn yaml_to_nix(input: &str) -> Result<String, Error> {
    format_to_nix(|s| yaml_serde::from_str::<yaml_serde::Value>(s), input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "ini")]
    #[test]
    fn converts_valid_ini() {
        let ini = "
enable-mouse = no
[dmenu]
mode = index
";
        let nix = ini_to_nix(ini);
        let expected = r#"{
  dmenu = {
    mode = "index";
  };
  enable-mouse = "no";
}"#;

        assert_eq!(nix.unwrap(), expected)
    }

    #[cfg(feature = "ini")]
    #[test]
    fn errors_on_invalid_ini() {
        let ini = r#"[broken
nonsense = "
"#;
        let nix = ini_to_nix(ini);

        assert!(matches!(nix, Err(Error::Ini(_))))
    }

    #[cfg(feature = "json")]
    #[test]
    fn converts_valid_json() {
        let json = r#"{
    "name": "forgejo"
}"#;
        let nix = json_to_nix(json);
        let expected = r#"{
  name = "forgejo";
}"#;

        assert_eq!(nix.unwrap(), expected)
    }

    #[cfg(feature = "json")]
    #[test]
    fn errors_on_invalid_json() {
        let json = r#"{
    "unclosed": "
}"#;
        let nix = json_to_nix(json);

        assert!(matches!(nix, Err(Error::Json(_))))
    }

    #[cfg(feature = "toml")]
    #[test]
    fn converts_valid_toml() {
        let toml = r#"
[package]
name = "any2nix"
"#;
        let nix = toml_to_nix(toml);
        let expected = r#"{
  package = {
    name = "any2nix";
  };
}"#;

        assert_eq!(nix.unwrap(), expected)
    }

    #[cfg(feature = "toml")]
    #[test]
    fn errors_on_invalid_toml() {
        let toml = "[broken
doesnt_work = foo";
        let nix = toml_to_nix(toml);

        assert!(matches!(nix, Err(Error::Toml(_))))
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn converts_valid_yaml() {
        let yaml = "countries:
  - BR
  - NO";
        let nix = yaml_to_nix(yaml);
        let expected = r#"{
  countries = [
    "BR"
    "NO"
  ];
}"#;

        assert_eq!(nix.unwrap(), expected)
    }

    #[cfg(feature = "yaml")]
    #[test]
    fn errors_on_invalid_yaml() {
        let yaml = "[unclosed, sequence";
        let nix = yaml_to_nix(yaml);

        assert!(matches!(nix, Err(Error::Yaml(_))))
    }
}
