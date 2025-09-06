use eframe::egui;

mod ui;
mod util;
mod core;

use core::frame_state::FrameState;
use core::Kitty;

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

        ui::draw_ui(ctx, &mut kitty);

        egui::CentralPanel::default().show(ctx, ui::canvas_panel::canvas_panel(ctx, &mut kitty, &frame_state));

    })
}