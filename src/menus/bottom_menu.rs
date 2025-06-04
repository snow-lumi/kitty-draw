use eframe::egui::{self, Ui};

use crate::buttons;
use crate::structs::kitty::Kitty;

fn bottom_menu_fn(ui: &mut Ui, state: &mut Kitty ) {
    ui.horizontal(|ui| {
        // pointer coordinates
        ui.label("x: ");
        ui.add_sized((70.0,20.0), egui::TextEdit::singleline(&mut state.x_string));
        ui.label("y: ");
        ui.add_sized((70.0,20.0), egui::TextEdit::singleline(&mut state.y_string));

        // pointer behavior
        buttons::bool_button(ui, "Absolute".to_string(), &mut state.pointer_absolute);

        // origin behavior
        buttons::bool_button(ui, "Show Origin".to_string(), &mut state.show_origin);
    });
}

pub fn bottom_menu(state: &mut Kitty ) -> impl FnMut(&mut Ui) {
    |ui| {bottom_menu_fn(ui, state);}
}