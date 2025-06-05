use eframe::egui;

mod canvas_shapes;
mod buttons;
mod math;
mod structs;
mod menus;
mod canvas;

use crate::canvas::canvas;
use crate::structs::frame_state::FrameState;
use crate::structs::kitty::Kitty;

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
            .show(ctx, menus::command_menu::command_menu(&mut kitty));

        // menu that controls canvas and pointer behavior
        egui::TopBottomPanel::bottom("meow")
            .show_separator_line(true)
            .show(ctx, menus::bottom_menu::bottom_menu(&mut kitty));

        // context menu that controls the current command
        egui::SidePanel::right("boioing").show(ctx, menus::settings_menu::settings_menu_menu(&mut kitty));

        egui::CentralPanel::default().show(ctx, canvas(ctx, &mut kitty, &frame_state));

    })
}