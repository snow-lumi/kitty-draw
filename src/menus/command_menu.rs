use eframe::egui::Ui;

use crate::buttons;
use crate::structs::{commands::Commands, kitty::Kitty};

fn command_menu_fn(ui: &mut Ui, kitty: &mut Kitty ) {
    ui.horizontal( |ui| {
        buttons::command_button(ui, Commands::Line, kitty);
        buttons::command_button(ui, Commands::Circle, kitty);
    });
}

pub fn command_menu(state: &mut Kitty ) -> impl FnMut(&mut Ui) {
    |ui| {command_menu_fn(ui, state);}
}