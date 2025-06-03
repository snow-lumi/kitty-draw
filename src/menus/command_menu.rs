use eframe::egui::Ui;

use crate::buttons;
use crate::structs::{Commands, program_state::ProgramState};

fn command_menu_fn(ui: &mut Ui, state: &mut ProgramState ) {
    ui.horizontal( |ui| {
        buttons::command_button(ui, Commands::Line, state);
        buttons::command_button(ui, Commands::Circle, state);
    });
}

pub fn command_menu(state: &mut ProgramState ) -> impl FnMut(&mut Ui) {
    |ui| {command_menu_fn(ui, state);}
}