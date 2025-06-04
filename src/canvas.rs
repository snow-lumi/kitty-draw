use eframe::egui::{self, Context, Pos2, Ui};

use crate::math::StrokelessTransformExt;
use crate::shapes;
use crate::structs::kitty::Kitty;
use crate::structs::frame_data::FrameData;
use crate::structs::NextInput;
use crate::structs::commands::{CommandResult, CommandState};

fn canvas_fn(ctx: &Context, ui: &mut Ui, state: &mut Kitty, frame_data: &FrameData ) {
    // make canvas
    let available_space = ui.available_size();
    let (response, painter) = ui.allocate_painter(available_space, egui::Sense::hover());
    let rect = response.rect;

    // generic grey color
    let mlem = egui::Stroke::new(1.0, egui::Color32::from_gray(100));

    // fake calculate origin
    if state.initializing {
        state.canvas_to_screen.translation = rect.center().to_vec2();
        state.initializing = false;
    }

    // draw origin
    if state.show_origin {
        let origin = Pos2::ZERO.transform_kitty_flip(state.canvas_to_screen);
        painter.hline(rect.x_range(), origin.y, mlem);
        painter.vline(origin.x, rect.y_range(), mlem);
    }

    // draw the image
    painter.extend(state.canvas_contents.clone().iter().map(|shape| -> egui::Shape {
        shape.clone().transform_kitty(state.canvas_to_screen)
    }));

    // handle mouse thingies
    let pointer_pos = frame_data.raw_pointer.latest_pos().filter(|&raw_pos| rect.contains(raw_pos));
    match pointer_pos {
        None => (),
        Some(pos) => {

            // scroll zoom
            let scroll_event = frame_data.events
                .iter()
                .find(|e| -> bool {
                    matches!(e, egui::Event::MouseWheel {..})
                });

            if let Some(egui::Event::MouseWheel { unit: _, delta, modifiers }) = scroll_event {
                let factor = match *modifiers {
                    egui::Modifiers::NONE => (1.055_f32).powf(delta.y),
                    egui::Modifiers::ALT => (1.022_f32).powf(delta.y),
                    _ => 1.0,
                };
                state.canvas_to_screen.scaling *= factor;
                state.canvas_to_screen.translation += (state.canvas_to_screen.translation - pos.to_vec2())*(factor-1.0);
            }
            
            // middle drag
            if frame_data.raw_pointer.middle_down() {
                state.canvas_to_screen.translation += frame_data.raw_pointer.delta();
            }

            // pointer in canvas coords
            let pos_canvas = pos.transform_kitty_flip(state.canvas_to_screen.inverse());
            state.canvas_to_screen.inverse().mul_pos(pos);

            // hide mouse
            ctx.output_mut(|output| {
                output.cursor_icon = egui::CursorIcon::None
            });

            // draw mouse crosshair: color if relative, grey if absolute (cuz it doesnt do anything)
            painter.add(shapes::crosshair(rect, pos, !state.pointer_absolute));

            // calculate where the user wants the position of the pointer
            let pointer_offset: egui::Vec2 = (state.x_string.parse().unwrap_or(0.0),- state.y_string.parse().unwrap_or(0.0)).into();
            let des_pointer = match state.pointer_absolute {
                true  => Pos2::ZERO.transform_kitty_flip(state.canvas_to_screen.inverse())+pointer_offset,
                false => pos+pointer_offset,
            };

            // draw the position of the thingy ([mouse + offset] or absolute position)
            let stroke_cursor = egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 100, 210));
            painter.add(shapes::x_shape(des_pointer, 5.0, stroke_cursor));

            // current command preview
            // let preview_current = if bleh {
            //     egui::Shape::LineSegment { points: [last_click_pos,des_pointer], stroke: usr_stroke }
            // } else {
            //     egui::Shape::Noop
            // };
            
            if frame_data.raw_pointer.primary_clicked() {
                match state.next_input((), pos_canvas) {
                    CommandResult::Nothing => (),
                    CommandResult::Shape(shape) => {
                        state.canvas_contents.push(shape);
                    }
                }
                println!("{:?}",state.command)
            }
            //painter.add(preview_current);
        }
    }

    // esc to abort command
    if ui.input(|i| -> bool {i.key_pressed(egui::Key::Escape)}) {
        state.command = CommandState::Noop;
    }
}

pub fn canvas(ctx: &Context, state: &mut Kitty, frame_data: &FrameData) -> impl FnMut(&mut Ui) {
    |ui| {canvas_fn(ctx, ui, state, frame_data);}
}