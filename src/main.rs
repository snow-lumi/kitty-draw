use eframe::egui;

mod ui;
mod util;
mod core;

use crate::core::frame_state::FrameState;
use crate::core::kitty::Kitty;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    // application state:
    let mut kitty = Kitty::new();

    eframe::run_simple_native("kitty draw", options, move |ctx, _frame| {

        let frame_state = ctx.input(|i| -> FrameState {FrameState::new(i)});

        kitty.handle_keyboard_input(&frame_state);

        // menu where you can choose commands
        egui::TopBottomPanel::top("woof")
            .show_separator_line(true)
            .show(ctx, ui::menus::command_menu::command_menu(&mut kitty));

        // menu that controls canvas and pointer behavior
        egui::TopBottomPanel::bottom("meow")
            .show_separator_line(true)
            .show(ctx, ui::menus::bottom_menu::bottom_menu(&mut kitty));

        // context menu that controls the current command
        egui::SidePanel::right("boioing").show(ctx, ui::menus::settings_menu::settings_menu(&mut kitty));

        egui::CentralPanel::default().show(ctx, ui::canvas_panel::canvas_panel(ctx, &mut kitty, &frame_state));

    })
}