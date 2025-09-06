use eframe::egui;
use crate::core::Kitty;

pub mod buttons;
pub mod menus;
pub mod painter_shapes;
pub mod canvas_panel;

pub fn draw_ui(ctx: &egui::Context, kitty: &mut Kitty) {
        // menu where you can choose commands
        egui::TopBottomPanel::top("woof")
            .show_separator_line(true)
            .show(ctx, menus::command_menu::command_menu(kitty));

        // menu that controls canvas and pointer behavior
        egui::TopBottomPanel::bottom("meow")
            .show_separator_line(true)
            .show(ctx, menus::bottom_menu::bottom_menu(kitty));

        // context menu that controls the current command
        egui::SidePanel::right("boioing").show(ctx, menus::settings_menu::settings_menu(kitty));
}

#[derive(Default, Debug)]
pub struct UiIds {
    pub bottom_interactable: Option<egui::Id>,
}

impl UiIds {
    pub fn focus_bottom(&self, ui: &mut egui::Ui) {
        if let Some(meow) = self.bottom_interactable {
            ui.memory_mut(|mem| -> _ {mem.request_focus(meow)});
        }
    }
}