use eframe::egui::{self, Ui};

use crate::buttons;
use crate::structs::commands::KittyCommands;
use crate::structs::kitty::Kitty;
use crate::structs::commands::{CommandState, line::LineOptions};

fn settings_menu_fn(ui: &mut Ui, kitty: &mut Kitty ) {
    match kitty.command {
        CommandState::Line(_) => {
            buttons::enum_toggle_button(ui, "Connected".to_owned(), &mut kitty.command_options.line, LineOptions::Connected, LineOptions::Separate);
        }
        _ => {ui.label("emty :>");}
    }

    ui.separator();
    ui.label("mew :3");
    ui.add(egui::Slider::new(&mut kitty.stroke.width, 0.1..=10.0).text("girth"));
    ui.color_edit_button_srgba(&mut kitty.stroke.color);

    ui.separator();
    buttons::func_button(ui, "Home".to_owned(), || {
        kitty.kitty_command_stack.push(KittyCommands::CanvasHome);
    });

    ui.separator();
}

pub fn settings_menu_menu(state: &mut Kitty ) -> impl FnMut(&mut Ui) {
    |ui| {settings_menu_fn(ui, state);}
}