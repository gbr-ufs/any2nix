// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use any2nix::Format;
use askama::Template;

use crate::i18n::I18n;

#[derive(Template)]
#[template(path = "components/snackbars/error.html")]
pub(crate) struct ErrorSnackbar {
    pub(crate) error: String,
}

#[derive(Template)]
#[template(path = "pages/index.html")]
pub(crate) struct IndexPage {
    pub(crate) i18n: I18n,
    pub(crate) formats: Vec<Format>,
}

#[derive(Template)]
#[template(path = "components/codes/nix.html")]
pub(crate) struct Nix {
    pub(crate) i18n: I18n,
    pub(crate) nix: String,
}
