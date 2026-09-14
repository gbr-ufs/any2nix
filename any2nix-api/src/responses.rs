// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Serialize;
#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Response structure for a successful conversion. Includes only one "nix"
/// key.
#[derive(Serialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct ConvertResponse {
    /// Houses the converted data.
    pub nix: String,
}

/// Response structure for an unsuccessful conversion. Includes only one "error" key.
#[derive(Serialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct ErrorResponse {
    /// Houses the error message of the error.
    pub error: String,
}

/// Response structure for the root endpoint. Inclues crate information.
#[derive(Serialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct RootResponse {
    pub description: String,
    #[cfg(feature = "utoipa-swagger-ui")]
    pub docs: String,
    pub name: String,
    #[cfg(feature = "utoipa")]
    pub openapi: String,
    pub version: String,
}
