// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets"]
pub(crate) struct Assets;

#[derive(Embed)]
#[folder = "node_modules"]
pub(crate) struct NodeModules;
