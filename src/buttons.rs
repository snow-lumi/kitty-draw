use eframe::egui::Ui;

use crate::math::BoolToggleExt;
use crate::structs::{CommandState, Commands, program_state::ProgramState};

pub fn command_button(ui: &mut Ui, mrrp: Commands, state: &mut ProgramState) {
    let button1 = ui.button(format!("{:?}",mrrp));
    if button1.clicked() {
        state.command = if state.command.into_command() == mrrp {
            CommandState::Noop
        } else {
            mrrp.starting_state()
        }
    }
    if state.command.into_command() == mrrp {
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

pub fn func_button<F: FnOnce()>(ui: &mut Ui, name: String, func: F) {
    let button = ui.button(name);
    if button.clicked() {
        func();
    }
}