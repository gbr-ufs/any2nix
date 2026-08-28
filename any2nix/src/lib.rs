// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

//! This crate provides simple functions for translating different formats
//! to [Nix](https://nixos.org).
//!
//! They serve as higher-level options to
//! [serialization functions](https://serde.rs/).
//!
//! Due to being based on serialization, the translation process has
//! a chance to fail, primarily on invalid formatting.
//!
//! # Usage
//!
//! This crate is on [crates.io](https://crates.io/crates/any2nix) and can
//! be added as a dependency to your project's `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! any2nix = "0.1"
//! ```
//!
//! # Supported Formats
//!
//! - [INI](https://en.wikipedia.org/wiki/INI_file)
//! - [JSON](https://www.json.org)
//! - [TOML](https://toml.io)
//! - [YAML](https://yaml.org)
//!
//! # Examples: TOML
//!
//! ```rust
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
//! ```

use serde::Serialize;

/// Aggregator of all errors from all used serializers (translators)
/// for simpler error handling.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[cfg(feature = "ini")]
    #[error("Invalid INI: {0}")]
    Ini(#[from] serde_ini::de::Error),
    #[cfg(feature = "json")]
    #[error("Invalid JSON: {0}")]
    Json(#[from] serde_json::Error),
    /// This error message is different because Nix is a target, not a source.
    #[error("Failed to serialize to Nix: {0}")]
    Nix(#[from] ser_nix::Error),
    #[cfg(feature = "toml")]
    #[error("Invalid TOML: {0}")]
    Toml(#[from] toml::de::Error),
    #[cfg(feature = "yaml")]
    #[error("Invalid YAML: {0}")]
    Yaml(#[from] yaml_serde::Error),
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
    let nix = ser_nix::to_string(&format)?;

    Ok(nix)
}

/// Translates text in [INI](https://en.wikipedia.org/wiki/INI_file) to [Nix](https://nixos.org).
///
/// # Errors
///
/// Returns [`Error::Ini`] in case the input cannot be parsed as valid INI.
///
/// Returns [`Error::Nix`] if the parsed data structure cannot be serialized to Nix.
///
/// # Examples
///
/// ```rust
/// let some_ini = "[Actor]
/// bUseNavMeshForMovement=1
/// fNotVisibleNavmeshMoveDist=2048.0000";
/// // We use "unwrap" because this returns a "Result" type. Never use `unwrap` in
/// // production!
/// let nix = any2nix::ini_to_nix(&some_ini).unwrap();
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
/// # assert_eq!(nix, "{\n  Actor = {\n    bUseNavMeshForMovement = \"1\";\n    fNotVisibleNavmeshMoveDist = \"2048.0000\";\n  };\n}")
/// ```
#[cfg(feature = "ini")]
pub fn ini_to_nix(input: &str) -> Result<String, Error> {
    format_to_nix(serde_ini::from_str::<serde_json::Value>, input)
}

/// Translates text in [JSON](https://www.json.org) to [Nix](https://nixos.org).
///
/// # Errors
///
/// Returns [`Error::Json`] in case the input cannot be parsed as valid JSON.
///
/// Returns [`Error::Nix`] if the parsed data structure cannot be serialized to Nix.
///
/// # Examples
///
/// ```rust
/// let some_json = r#"
/// {
///     "name": "any2nix",
///     "version": "0.1.0"
/// }"#;
/// // We use "unwrap" because this returns a "Result" type. Never use `unwrap` in
/// // production!
/// let nix = any2nix::json_to_nix(&some_json).unwrap();
///
///
/// println!("{}", nix);
/// // Output:
/// //
/// // {
/// //   name = "any2nix";
/// //   version = "0.1.0";
/// // }
/// # assert_eq!(nix, "{\n  name = \"any2nix\";\n  version = \"0.1.0\";\n}")
/// ```
#[cfg(feature = "json")]
pub fn json_to_nix(input: &str) -> Result<String, Error> {
    format_to_nix(|s| serde_json::from_str::<serde_json::Value>(s), input)
}

/// Translates text in [TOML](https://toml.io) to [Nix](https://nixos.org).
///
/// # Errors
///
/// Returns [`Error::Toml`] in case the input cannot be parsed as valid TOML.
///
/// Returns [`Error::Nix`] if the parsed data structure cannot be serialized to Nix.
///
/// # Examples
///
/// ```rust
/// let some_toml = r#"
/// [package]
/// name = "any2nix"
/// "#;
/// // We use "unwrap" because this returns a "Result" type. Never use `unwrap` in
/// // production!
/// let nix = any2nix::toml_to_nix(&some_toml).unwrap();
///
/// println!("{}", nix);
/// // Output:
/// //
/// // {
/// //   package = {
/// //     name = "any2nix";
/// //   };
/// // }
/// # assert_eq!(nix, "{\n  package = {\n    name = \"any2nix\";\n  };\n}")
/// ```
#[cfg(feature = "toml")]
pub fn toml_to_nix(input: &str) -> Result<String, Error> {
    format_to_nix(|s| toml::from_str::<toml::Value>(s), input)
}

/// Translates text in [YAML](https://yaml.org) to [Nix](https://nixos.org).
///
/// # Errors
///
/// Returns [`Error::Yaml`] in case the input cannot be parsed as valid YAML.
///
/// Returns [`Error::Nix`] if the parsed data structure cannot be serialized to Nix.
///
/// # Examples
///
/// ```rust
/// let some_yaml = r#"
/// services:
///   db:
///     image: postgres:16-alpine
///     restart: always
///     ports:
///       - "5432:5432"
/// "#;
/// // We use "unwrap" because this returns a "Result" type. Never use `unwrap` in
/// // production!
/// let nix = any2nix::yaml_to_nix(&some_yaml).unwrap();
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
/// # assert_eq!(nix, "{\n  services = {\n    db = {\n      image = \"postgres:16-alpine\";\n      ports = [\n        \"5432:5432\"\n      ];\n      restart = \"always\";\n    };\n  };\n}")
/// ```
#[cfg(feature = "yaml")]
pub fn yaml_to_nix(input: &str) -> Result<String, Error> {
    format_to_nix(|s| yaml_serde::from_str::<serde_json::Value>(s), input)
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

        assert!(nix.is_ok());

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

        assert!(nix.is_ok());

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

        assert!(nix.is_ok());

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

        assert!(nix.is_ok());

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
