use eframe::egui::{self, Context, Pos2, Ui};

use crate::math::StrokelessTransformExt;
use crate::canvas_shapes;
use crate::structs::kitty::Kitty;
use crate::structs::frame_state::FrameState;
use crate::structs::Preview;

fn canvas_fn(ctx: &Context, ui: &mut Ui, kitty: &mut Kitty, frame_state: &FrameState ) {
    // make canvas painter
    let available_space = ui.available_size();
    let (response, painter) = ui.allocate_painter(available_space, egui::Sense::hover());
    let screen_rect = response.rect;

    // initialize canvas
    if !kitty.canvas_initialized {
        kitty.initialize_canvas(screen_rect);
    }

    kitty.do_kitty_commands(screen_rect);

    // handle mouse input
    match frame_state.pointer_in(screen_rect) {
        None => (),
        Some(pos) => {

            // calculate where the user wants the position of the pointer
            let pointer_offset: egui::Vec2 = (kitty.x_string.parse().unwrap_or(0.0),- kitty.y_string.parse().unwrap_or(0.0)).into();
            let des_pointer = match kitty.pointer_absolute {
                true  => kitty.canvas_to_screen.inverse().transform_pos(Pos2::ZERO)+pointer_offset,
                false => pos+pointer_offset,
            };

            kitty.handle_mouse_input_canvas(frame_state, pos, des_pointer);

            // hide mouse
            ctx.output_mut(|output| {
                output.cursor_icon = egui::CursorIcon::None
            });
        }
    }

    // draw origin
    painter.add(canvas_shapes::simple_crosshair(screen_rect, kitty.canvas_origin(), egui::Stroke::new(1.0, egui::Color32::from_gray(100))));

    // draw the image
    painter.extend(kitty.canvas_contents.clone().iter().map(|shape| -> egui::Shape {
        shape.clone().transform_kitty(kitty.canvas_to_screen)
    }));

    // draw mouse thingies
    match frame_state.pointer_in(screen_rect) {
        None => (),
        Some(pos) => {

            // calculate where the user wants the position of the pointer
            let pointer_offset: egui::Vec2 = (kitty.x_string.parse().unwrap_or(0.0),- kitty.y_string.parse().unwrap_or(0.0)).into();
            let des_pointer = match kitty.pointer_absolute {
                true  => kitty.canvas_to_screen.inverse().transform_pos(Pos2::ZERO)+pointer_offset,
                false => pos+pointer_offset,
            };

            // draw mouse crosshair
            painter.add(canvas_shapes::cursor_crosshair(screen_rect, pos, !kitty.pointer_absolute));

            // draw the position of the thingy ([mouse + offset] or absolute position)
            let stroke_cursor = egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 100, 210));
            painter.add(canvas_shapes::x_shape(des_pointer, 5.0, stroke_cursor));

            painter.add(kitty.preview((), pos));
        }
    }
}

pub fn canvas(ctx: &Context, kitty: &mut Kitty, frame_data: &FrameState) -> impl FnMut(&mut Ui) {
    |ui| {canvas_fn(ctx, ui, kitty, frame_data);}
}