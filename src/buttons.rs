use eframe::egui::Ui;

use crate::math::BoolToggleExt;
use crate::structs::kitty::Kitty;
use crate::structs::commands::{CommandState, Commands};

pub fn command_button(ui: &mut Ui, mrrp: Commands, kitty: &mut Kitty) {
    let button1 = ui.button(format!("{:?}",mrrp));
    if button1.clicked() {
        kitty.command = if kitty.command.into_command() == mrrp {
            CommandState::Noop
        } else {
            mrrp.starting_state()
        }
    }
    if kitty.command.into_command() == mrrp {
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