// SPDX-FileCopyrightText: 2026 Gabriel Santos de Souza <gabriel.santosdesouza@dcomp.ufs.br>
//
// SPDX-License-Identifier: GPL-3.0-or-later

use any2nix::Format;
use any2nix_i18n::I18n;
use gtk::{
    Align, Button, Image, Label, Orientation, Revealer, RevealerTransitionType, ScrolledWindow,
    Stack, StackSwitcher, StackTransitionType,
    glib::{self, clone},
    prelude::*,
};
use sourceview5::prelude::{BufferExt, ViewExt};

fn set_style_scheme(buffer: &sourceview5::Buffer) {
    let style_manager = adw::StyleManager::default();
    let is_dark = style_manager.is_dark();
    let scheme_id = if is_dark { "Adwaita-dark" } else { "Adwaita" };

    if let Some(scheme) = sourceview5::StyleSchemeManager::default().scheme(scheme_id) {
        buffer.set_style_scheme(Some(&scheme));
    }

    style_manager.connect_dark_notify(clone!(
        #[weak]
        buffer,
        move |sm| {
            let scheme_id = if sm.is_dark() {
                "Adwaita-dark"
            } else {
                "Adwaita"
            };

            if let Some(scheme) = sourceview5::StyleSchemeManager::default().scheme(scheme_id) {
                buffer.set_style_scheme(Some(&scheme));
            }
        }
    ));
}

fn build_error_message() -> Revealer {
    let icon = Image::from_icon_name("dialog-error-symbolic");

    icon.add_css_class("error");

    let label = Label::new(None);

    label.add_css_class("error");

    label.set_wrap(true);
    label.set_max_width_chars(80);

    let root_box = gtk::Box::new(Orientation::Horizontal, 8);

    root_box.set_halign(Align::Center);

    root_box.append(&icon);
    root_box.append(&label);

    Revealer::builder()
        .child(&root_box)
        .transition_type(RevealerTransitionType::SlideDown)
        .reveal_child(false)
        .build()
}

fn build_editor(format: &Format, error_message: &Revealer) -> ScrolledWindow {
    let buffer = sourceview5::Buffer::new(None);

    buffer.set_highlight_syntax(true);
    set_style_scheme(&buffer);

    if let Some(ref language) =
        sourceview5::LanguageManager::new().language(&format.to_string().to_lowercase())
    {
        buffer.set_language(Some(language));
    }

    buffer.connect_changed(clone!(
        #[weak]
        error_message,
        move |_| {
            if error_message.reveals_child() {
                error_message.set_reveal_child(false);
            }
        }
    ));

    let view = sourceview5::View::with_buffer(&buffer);

    view.set_top_margin(12);
    view.set_bottom_margin(12);
    view.set_left_margin(12);
    view.set_right_margin(12);

    view.set_monospace(true);
    view.set_show_line_numbers(true);

    ScrolledWindow::builder()
        .child(&view)
        .has_frame(true)
        .min_content_width(680)
        .min_content_height(260)
        .vexpand(true)
        .build()
}

fn build_result_page() -> ScrolledWindow {
    let buffer = sourceview5::Buffer::new(None);

    buffer.set_highlight_syntax(true);
    set_style_scheme(&buffer);

    if let Some(ref nix_lang) = sourceview5::LanguageManager::new().language("nix") {
        buffer.set_language(Some(nix_lang));
    }

    let view = sourceview5::View::with_buffer(&buffer);

    view.set_editable(false);
    view.set_monospace(true);
    view.set_show_line_numbers(true);

    view.set_top_margin(12);
    view.set_bottom_margin(12);
    view.set_left_margin(12);
    view.set_right_margin(12);

    ScrolledWindow::builder()
        .child(&view)
        .has_frame(true)
        .min_content_width(680)
        .min_content_height(260)
        .vexpand(true)
        .build()
}

fn get_editor_buffer(scrolled_window: &ScrolledWindow) -> sourceview5::Buffer {
    scrolled_window
        .child()
        .and_downcast::<sourceview5::View>()
        .expect("scrolled window contains a view")
        .buffer()
        .downcast::<sourceview5::Buffer>()
        .expect("view contains a sourceview buffer")
}

fn get_error_message_label(error_message: &Revealer) -> Label {
    error_message
        .child()
        .and_downcast::<gtk::Box>()
        .expect("revealer contains a box")
        .last_child()
        .and_downcast::<Label>()
        .expect("box contains error label")
}

fn build_attach_button(i18n: I18n, editor: &ScrolledWindow, error_message: &Revealer) -> Button {
    let button = Button::builder()
        .label(i18n.attach_file)
        .icon_name("mail-attachment-symbolic")
        .build();

    let buffer = get_editor_buffer(editor);
    let error_message_label = get_error_message_label(error_message);

    button.connect_clicked(clone!(
        #[weak]
        error_message,
        move |btn| {
            let window = btn.root().and_downcast::<gtk::Window>();
            let dialog = gtk::FileDialog::builder().title(i18n.attach_file).build();

            dialog.open(
                window.as_ref(),
                gtk::gio::Cancellable::NONE,
                clone!(
                    #[weak]
                    buffer,
                    #[weak]
                    error_message,
                    #[weak]
                    error_message_label,
                    move |res| {
                        if let Ok(file) = res {
                            match file.load_contents(gtk::gio::Cancellable::NONE) {
                                Ok((bytes, _)) => match std::str::from_utf8(&bytes) {
                                    Ok(text) => {
                                        error_message.set_reveal_child(false);
                                        buffer.set_text(text);
                                    }
                                    // Invalid UTF-8.
                                    Err(err) => {
                                        error_message_label.set_text(&err.to_string());
                                        error_message.set_reveal_child(true);
                                    }
                                },
                                // I/O error.
                                Err(err) => {
                                    error_message_label.set_text(&err.to_string());
                                    error_message.set_reveal_child(true);
                                }
                            }
                        }
                    }
                ),
            );
        }
    ));

    button
}

fn build_submit_button(
    i18n: I18n,
    format: &Format,
    stack: &Stack,
    editor: &ScrolledWindow,
    result_page: &ScrolledWindow,
    error_message: &Revealer,
) -> Button {
    let button = Button::with_label(i18n.submit);

    button.add_css_class("suggested-action");

    let current_format = *format;
    let input_buffer = get_editor_buffer(editor);
    let result_buffer = get_editor_buffer(result_page);
    let error_message_label = get_error_message_label(error_message);

    button.connect_clicked(clone!(
        #[weak]
        stack,
        #[weak]
        error_message,
        move |_| {
            let text =
                input_buffer.text(&input_buffer.start_iter(), &input_buffer.end_iter(), false);

            match current_format.to_nix(&text) {
                Ok(nix_code) => {
                    error_message.set_reveal_child(false);
                    result_buffer.set_text(&nix_code);
                    stack.set_visible_child_name("result");
                }
                Err(err) => {
                    error_message_label.set_text(&err.to_string());
                    error_message.set_reveal_child(true);
                }
            }
        }
    ));

    button
}

fn build_result_button(i18n: I18n, stack: &Stack, editor: &ScrolledWindow) -> Button {
    let button = Button::with_label(i18n.another_one);
    let input_buffer = get_editor_buffer(editor);

    button.connect_clicked(clone!(
        #[weak]
        stack,
        move |_| {
            input_buffer.set_text("");
            stack.set_visible_child_name("input");
        }
    ));

    button
}

fn build_format_page(i18n: I18n, format: &Format) -> Stack {
    let stack = Stack::new();
    stack.set_transition_type(StackTransitionType::Crossfade);

    let error_message = build_error_message();
    let editor = build_editor(format, &error_message);
    let result_page = build_result_page();
    let attach_button = build_attach_button(i18n, &editor, &error_message);
    let submit_button =
        build_submit_button(i18n, format, &stack, &editor, &result_page, &error_message);

    let input_button_box = gtk::Box::new(Orientation::Horizontal, 12);

    input_button_box.set_halign(Align::Center);
    input_button_box.append(&attach_button);
    input_button_box.append(&submit_button);

    let result_button = build_result_button(i18n, &stack, &editor);

    let result_button_box = gtk::Box::new(Orientation::Horizontal, 12);

    result_button_box.set_halign(Align::Center);
    result_button_box.append(&result_button);

    let input_box = gtk::Box::new(Orientation::Vertical, 16);

    input_box.append(&editor);
    input_box.append(&error_message);
    input_box.append(&input_button_box);

    let result_box = gtk::Box::new(Orientation::Vertical, 16);

    result_box.append(&result_page);
    result_box.append(&result_button_box);

    stack.add_named(&input_box, Some("input"));
    stack.add_named(&result_box, Some("result"));
    stack.set_visible_child_name("input");

    stack
}

fn build_stack(i18n: I18n, formats: &'static [Format]) -> Stack {
    let stack = Stack::new();

    stack.set_transition_type(StackTransitionType::Crossfade);

    for format in formats {
        let page = build_format_page(i18n, format);

        stack.add_titled(&page, Some(&format.to_string()), &format.to_string());
    }

    stack
}

pub(crate) fn build_content(i18n: I18n, formats: &'static [Format]) -> gtk::Box {
    let stack = build_stack(i18n, formats);
    let stack_switcher = StackSwitcher::builder()
        .stack(&stack)
        .halign(Align::Center)
        .margin_bottom(24)
        .build();
    let root_box = gtk::Box::new(Orientation::Vertical, 0);

    root_box.set_halign(Align::Center);
    root_box.set_valign(Align::Center);
    root_box.set_margin_top(24);
    root_box.set_margin_bottom(24);
    root_box.set_margin_start(24);
    root_box.set_margin_end(24);

    root_box.append(&stack_switcher);
    root_box.append(&stack);

    root_box
}
