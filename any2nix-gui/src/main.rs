// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod ui;

use adw::prelude::*;
use any2nix::Format;
use any2nix_i18n::I18n;
use gtk::{Orientation, glib};

const APP_ID: &str = "dev.gs-101.any2nix";

fn main() -> glib::ExitCode {
    let app = adw::Application::builder().application_id(APP_ID).build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(application: &adw::Application) {
    let i18n = I18n::from_locales(glib::language_names().iter().map(|s| s.as_str()));
    let header_bar = adw::HeaderBar::new();
    let content = ui::build_content(i18n, Format::VARIANTS);
    let window_box = gtk::Box::new(Orientation::Vertical, 0);

    window_box.append(&header_bar);
    window_box.append(&content);

    let window = adw::ApplicationWindow::builder()
        .application(application)
        .title("any2nix")
        .content(&window_box)
        .default_width(850)
        .default_height(600)
        .build();

    window.present();
}
