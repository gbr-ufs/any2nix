/*
 * SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
 *
 * SPDX-License-Identifier: GPL-3.0-or-later
 */

document.addEventListener("htmx:beforeSwap", function (evt) {
  if (
    evt.detail.xhr.status === 400 ||
    evt.detail.xhr.status === 401 ||
    evt.detail.xhr.status === 404
  ) {
    evt.detail.shouldSwap = true;
    evt.detail.isError = false;
    const snackbar = document.getElementById("snackbar");
    if (snackbar) {
      evt.detail.target = snackbar;
    }
  }
});

document.addEventListener("htmx:afterSwap", function (evt) {
  if (evt.detail.xhr.status === 200) {
    const activeSnackbar = document.querySelector(".snackbar.active");
    if (activeSnackbar) {
      activeSnackbar.classList.remove("active");
    }
    return;
  }

  const snackbar = document.getElementById("snackbar");
  if (snackbar && snackbar.classList.contains("snackbar")) {
    if (window.ui) {
      window.ui("#snackbar");
    }
  }
});
