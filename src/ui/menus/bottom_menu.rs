use eframe::egui::{self, Ui};

use crate::ui::buttons;
use crate::core::kitty::Kitty;

fn bottom_menu_fn(ui: &mut Ui, kitty: &mut Kitty ) {
    ui.horizontal(|ui| {
        // pointer coordinates
        ui.label("x: ");
        ui.add_sized((70.0,20.0), egui::TextEdit::singleline(&mut kitty.x_string));
        ui.label("y: ");
        ui.add_sized((70.0,20.0), egui::TextEdit::singleline(&mut kitty.y_string));

        // pointer behavior
        buttons::bool_button(ui, "Absolute".to_string(), &mut kitty.pointer_absolute);

        // origin behavior
        buttons::bool_button(ui, "Show Origin".to_string(), &mut kitty.show_origin);
    });
}

pub fn bottom_menu(state: &mut Kitty ) -> impl FnMut(&mut Ui) {
    |ui| {bottom_menu_fn(ui, state);}
}