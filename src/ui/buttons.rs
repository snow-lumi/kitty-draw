use eframe::egui::Ui;



use crate::util::extensions::BoolToggleExt;
use crate::core::Kitty;
use crate::core::commands::Commands;

pub fn command_button(ui: &mut Ui, command: Commands, kitty: &mut Kitty ) {
    let button1 = ui.button(format!("{:?}", command));
    if button1.clicked() {
        if kitty.command.into_command() == command {
            kitty.command.noop();
        } else {
            kitty.command.start(command);
            kitty.ui_ids.focus_bottom(ui);
        }
    }
    if kitty.command.into_command() == command {
        button1.highlight();
    }
}

pub fn bool_button(ui: &mut Ui, name: String, state: &mut bool) {
    let button = ui.button(name);
    if button.clicked() {
        state.toggle();
    }
    if *state {
        button.highlight();
    }
}

pub fn enum_toggle_button<E: PartialEq + Clone>(ui: &mut Ui, name: String, state: &mut E, on: E, off: E) {
    let button = ui.button(name);
    if button.clicked() {
        if *state == on.clone() {
            *state = off.clone();
        } else {
            *state = on.clone();
        }
    }
    if *state == on.clone() {
        button.highlight();
    }
}

pub fn func_button<F: FnOnce()>(ui: &mut Ui, name: String, func: F) {
    let button = ui.button(name);
    if button.clicked() {
        func();
    }
}

#[expect(dead_code)]
pub fn func_button_highlight<F: FnOnce()>(ui: &mut Ui, name: String, func: F, highlight: bool) {
    let button = ui.button(name);
    if button.clicked() {
        func();
    }
    if highlight {
        button.highlight();
    }
}