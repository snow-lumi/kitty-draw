use eframe::egui::{self, Color32, Context, Pos2, Rect, Stroke, StrokeKind, Ui};
use eframe::emath::RectTransform;

use crate::ui::painter_shapes;
use crate::core::Kitty;
use crate::util::draw_shapes::KittyDrawShape;
use crate::core::frame_state::FrameState;
use crate::core::Preview;
use crate::util::convert::kittyrect_to_rect_t;
use crate::util::math::KittyVec2;

fn canvas_panel_fn(ctx: &Context, ui: &mut Ui, kitty: &mut Kitty, frame_state: &FrameState ) {
    // make canvas painter
    let available_space = ui.available_size();
    let (response, painter) = ui.allocate_painter(available_space, egui::Sense::hover());
    let screen_rect = response.rect;

    // initialize canvas
    kitty.canvas.update(screen_rect);

    kitty.do_kitty_commands(screen_rect);

    // handle mouse input
    match frame_state.pointer_in(screen_rect) {
        None => (),
        Some(pos) => {

            // calculate where the user wants the position of the pointer
            let des_pointer = match kitty.pointer_absolute {
                true  => kitty.pos_to_canvas(Pos2::ZERO)+kitty.pointer_offset(),
                false => kitty.pos_to_canvas(pos)+kitty.pointer_offset(),
            };

            kitty.handle_mouse_input_canvas(frame_state, pos, des_pointer);

            // hide mouse
            ctx.output_mut(|output| {
                output.cursor_icon = egui::CursorIcon::None
            });

        }
    }

    // draw origin
    painter.add(painter_shapes::simple_crosshair(screen_rect, kitty.canvas_origin(), egui::Stroke::new(1.0, egui::Color32::from_gray(100))));

    // draw the image
    painter.extend(kitty.canvas_draw());

    // draw the selection
    if kitty.command.selecting() {
        painter.extend(kitty.selection_draw());
    }

    // preview drag zoom
    if let Some(zoom_rect) = kitty.zoom_rect.clone() {
        painter.rect(
            kittyrect_to_rect_t(zoom_rect,kitty.canvas.to_screen),
            0.0,
            Color32::TRANSPARENT,
            Stroke {
                width: 1.0,
                color: Color32::WHITE,
            },
            StrokeKind::Middle,
        );
    }

    // draw mouse thingies
    match frame_state.pointer_in(screen_rect) {
        None => (),
        Some(pos) => {

            // calculate where the user wants the position of the pointer
            let des_pointer = match kitty.pointer_absolute {
                true  => kitty.pos_to_canvas(Pos2::ZERO)+kitty.pointer_offset(),
                false => kitty.pos_to_canvas(pos)+kitty.pointer_offset(),
            };

            // draw mouse crosshair
            painter.add(painter_shapes::cursor_crosshair(screen_rect, pos, !kitty.pointer_absolute));

            // draw the position of the thingy ([mouse + offset] or absolute position)
            if kitty.pointer_offset() != KittyVec2::ZERO
            {
                let stroke_cursor = egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 100, 210));
                painter.add(painter_shapes::x_shape(kitty.pos_to_screen(des_pointer), 4.0, stroke_cursor));
            }

            painter.add(kitty.preview((), pos));
        }
    }
}

pub fn canvas_panel(ctx: &Context, kitty: &mut Kitty, frame_data: &FrameState) -> impl FnMut(&mut Ui) {
    |ui| {canvas_panel_fn(ctx, ui, kitty, frame_data);}
}

#[derive(Debug)]
pub struct Canvas {
    initialized: bool,
    last_screen_rect: Option<Rect>,
    pub contents: Vec<KittyDrawShape>,
    pub to_screen: RectTransform,
}

impl Canvas {
    pub fn new() -> Self {
        Self {
            initialized: false,
            last_screen_rect: None,
            contents: vec![],
            to_screen: RectTransform::from_to(
                Rect {
                    min: (0.0,0.0).into(),
                    max: (1.0,1.0).into(), 
                },
                Rect {
                    min: (0.0,0.0).into(),
                    max: (1.0,-1.0).into(),
                },
            ),
        }
    }

    pub fn update(&mut self, screen_rect: Rect) {
        if !self.initialized {
            self.initialize(screen_rect);
        }
        if let Some(last_screen_rect) = self.last_screen_rect {
            if screen_rect != last_screen_rect {
                let new_canvas_rect = (RectTransform::from_to(last_screen_rect,screen_rect)).transform_rect(*self.to_screen.from());
                self.to_screen = RectTransform::from_to(new_canvas_rect,screen_rect)
            }
        }
        self.last_screen_rect = Some(screen_rect);
    }

    pub fn initialize(&mut self, screen_rect: Rect) {
        let center = screen_rect.center();
        let canvas_rect = screen_rect.translate(center.to_vec2() * -1.0);
        let screen_flipped = Rect {
            min: Pos2 {
                x: screen_rect.min.x,
                y: screen_rect.max.y,
            },
            max: Pos2 {
                x: screen_rect.max.x,
                y: screen_rect.min.y,
            },
        };
        self.to_screen = RectTransform::from_to(
            canvas_rect,
            screen_flipped,
        );
        self.initialized = true;
    }

    pub fn from_screen(&self) -> RectTransform {
        self.to_screen.inverse()
    }
}