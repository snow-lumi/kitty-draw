use eframe::egui;

mod shapes;
mod buttons;
mod math;
mod structs;
mod menus;
mod canvas;

use crate::canvas::canvas;
use crate::structs::frame_data::FrameData;
use crate::structs::kitty::Kitty;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    // application state:
    let mut state = Kitty::new();


    eframe::run_simple_native("kitty draw", options, move |ctx, _frame| {

        let frame_data = ctx.input(|i| -> FrameData {FrameData::new(i)});

        // menu where you can choose commands
        egui::TopBottomPanel::top("woof")
            .show_separator_line(true)
            .show(ctx, menus::command_menu::command_menu(&mut state));

        // menu that controls canvas and pointer behavior
        egui::TopBottomPanel::bottom("meow")
            .show_separator_line(true)
            .show(ctx, menus::bottom_menu::bottom_menu(&mut state));

        // context menu that controls the current command
        egui::SidePanel::right("boioing").show(ctx, menus::settings_menu::settings_menu_menu(&mut state));

        egui::CentralPanel::default().show(ctx, canvas(ctx, &mut state, &frame_data));

    })
}