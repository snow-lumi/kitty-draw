use eframe::egui::{self, Ui};

use crate::ui::buttons;
use crate::core::commands::KittyCommands;
use crate::core::kitty::Kitty;
use crate::core::commands::{CommandState, line::LineOptions};

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
    buttons::func_button(ui, "Clear".to_owned(), || {
        kitty.canvas_contents.clear();
    });

    ui.separator();


    ui.label(format!("pointer: {:#?}",kitty.pointer_pos));
    ui.label(format!("drag: {:#?}",kitty.drag_pos));
    ui.label(format!("zoom: {:#?}",kitty.zoom_rect));
    ui.label(format!(
        "zoom: {:#?}",
        kitty.zoom_rect.clone().map( |x| -> _ {
            crate::util::convert::kittyrect_to_rect_t(x,kitty.canvas_to_screen)
        }),
    ));
}

pub fn settings_menu(state: &mut Kitty ) -> impl FnMut(&mut Ui) {
    |ui| {settings_menu_fn(ui, state);}
}