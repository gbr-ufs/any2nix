// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use serde::Deserialize;
#[cfg(feature = "utoipa")]
use utoipa::ToSchema;

/// Request structure for a conversion. Includes only one "input" key.
#[derive(Deserialize)]
#[cfg_attr(feature = "utoipa", derive(ToSchema))]
pub struct ConvertRequest {
    /// Houses the data that will be converted.
    pub input: String,
}
