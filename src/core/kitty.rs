use eframe::egui::{self, Key, Pos2, Rect, Shape, Stroke, Vec2};
use eframe::emath::{RectTransform, TSTransform};

use crate::util::draw_shapes::KittyDrawShape;
use crate::util::math::collide::KittyCollide;
use crate::util::math::distance::KittyDistance;
use crate::util::math::shapes::{KittyDisc, KittyPoint, KittyRectangle, KittyShape};
use crate::util::convert::{kittyds_to_shape, kittypt_to_pos2_t, kittyrect_to_rect, pos2_to_kittypt_t, rect_to_kittyrect};
use crate::core::commands::select_single::SelectSingleState;
use crate::core::frame_state::FrameState;
use crate::core::{NextCommandInput, Preview};
use crate::core::commands::{CommandOptions, CommandResult, CommandState, KittyCommands};
use crate::util::math::{weird_rect_func, KittyVec2};

#[derive(Debug)]
pub struct Kitty {
    pub command: CommandState,
    pub command_options: CommandOptions,
    pub show_origin: bool,
    pub pointer_absolute: bool,
    pub stroke: Stroke,
    pub x_string: String,
    pub y_string: String,
    pub canvas_to_screen: RectTransform,
    pub canvas_initialized: bool,
    pub canvas_contents: Vec<KittyDrawShape>,
    pub kitty_command_stack: Vec<KittyCommands>,
    pub zoom_rect: Option<KittyRectangle>,
}

impl Kitty {
    pub fn new() -> Self {
        Self {
            command: CommandState::Noop,
            command_options: CommandOptions::default(),
            show_origin: true,
            pointer_absolute: false,
            stroke: egui::Stroke::new(1.0, egui::Color32::WHITE),
            x_string: String::default(),
            y_string: String::default(),
            canvas_to_screen: RectTransform::from_to(
                Rect {
                    min: (0.0,0.0).into(),
                    max: (1.0,1.0).into(), 
                },
                Rect {
                    min: (0.0,0.0).into(),
                    max: (1.0,-1.0).into(),
                },
            ),
            canvas_initialized: false,
            canvas_contents: vec![],
            kitty_command_stack: vec![],
            zoom_rect: None,
        }
    }

    pub fn initialize_canvas(&mut self, screen_rect: Rect) {
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
        self.canvas_to_screen = RectTransform::from_to(
            canvas_rect,
            screen_flipped,
        );
        self.canvas_initialized = true;
    }

    pub fn canvas_origin(&self) -> Pos2 {
        self.pos_to_screen(KittyPoint::ZERO)
    }

    pub fn screen_to_canvas(&self) -> RectTransform {
        self.canvas_to_screen.inverse()
    }

    pub fn vec2_screen_to_canvas_scale(&self, vec: Vec2) -> Vec2 {
        let scale = self.screen_to_canvas().scale();
        Vec2 {
            x: vec.x * scale.x,
            y: vec.y * scale.y,
        }
    }

    pub fn canvas_zoom(&mut self, amount: f32, center: Pos2) {
        let center_in_canvas = self.screen_to_canvas().transform_pos(center);
        let scale_trans = TSTransform::from_translation(center_in_canvas.to_vec2()) * TSTransform::from_scaling(amount) * TSTransform::from_translation(center_in_canvas.to_vec2()*-1.0);
        self.canvas_to_screen = RectTransform::from_to(
            scale_trans.mul_rect(*self.canvas_to_screen.from()),
            *self.canvas_to_screen.to(),
        );
    }

    pub fn canvas_drag(&mut self, amount: Vec2) {
        self.canvas_to_screen = RectTransform::from_to(
            TSTransform::from_translation(- self.vec2_screen_to_canvas_scale(amount)).mul_rect(*self.canvas_to_screen.from()),
            *self.canvas_to_screen.to(),
        );
    }

    pub fn handle_keyboard_input(&mut self, state: &FrameState) {
        if state.keys_pressed.contains(&Key::Escape) {
            self.command = CommandState::Noop;
        }
    }

    pub fn click_select(&self, pos: Pos2) -> Option<usize> {
        let shapes = self.canvas_contents.clone();
        let pointer_in_canvas = self.pos_to_canvas(pos);
        let pointer_disc = KittyDisc::new(pointer_in_canvas, self.screen_to_canvas().scale().x * 10.0);

        shapes.iter()
            .enumerate()
            .filter(|&(_index,shape)| -> bool {
                let shape_k: KittyShape = shape.clone().get_shape();
                shape_k.collides(pointer_disc.clone())
            })
            .map(|(index,shape)| -> (usize,f32) {
                let shape_k: KittyShape = shape.clone().get_shape();
                (index,shape_k.distance(pointer_in_canvas))
            })
            .fold(None, |acc ,(index,dist)| -> Option<(usize,f32)> {
                match acc {
                    None => Some((index,dist)),
                    Some((_,acc_d)) => {
                        if dist < acc_d {
                            Some((index,dist))
                        } else {
                            acc
                        }
                    },
                }
            })
            .map(|(index,_)| -> usize {index})
    }

    pub fn pointer_offset(&self) -> KittyVec2 {
        (self.x_string.parse().unwrap_or(0.0),self.y_string.parse().unwrap_or(0.0)).into()
    }

    pub fn handle_mouse_input_canvas(&mut self, frame_state: &FrameState, pos: Pos2, des_pointer: KittyPoint) {
        // scroll zoom
        let scroll_event = frame_state.events
            .iter()
            .find(|e| -> bool {
                matches!(e, egui::Event::MouseWheel {..})
            });

        if let Some(egui::Event::MouseWheel { unit: _, delta, modifiers }) = scroll_event {
            let amount = match *modifiers {
                egui::Modifiers::NONE => (1.1_f32).powf(-delta.y),
                egui::Modifiers::ALT => (1.03_f32).powf(-delta.y),
                _ => 1.0,
            };
            self.canvas_zoom(amount, pos);
        }
        
        // middle drag
        if frame_state.raw_pointer.middle_down() {
            self.canvas_drag(frame_state.raw_pointer.delta());
        }

        // left click
        if frame_state.raw_pointer.primary_clicked() {
            match self.next_input((), des_pointer) {
                CommandResult::Nothing => (),
                CommandResult::Shape(shape) => {
                    self.canvas_contents.push(shape);
                }
            }

            if self.command.idling() {
                match self.click_select(pos) {
                    None => {
                        self.command = CommandState::Noop;
                    },
                    Some(index) => {
                        self.command = CommandState::select_single(index);
                    }
                }
            }
        }

        if frame_state.raw_pointer.is_decidedly_dragging() {
            if let Some(drag_start) = frame_state.raw_pointer.press_origin(){
                if frame_state.raw_pointer.primary_down() {
                    #[expect(clippy::single_match)]
                    match self.command {
                        CommandState::SelectSingle(state) => {
                            self.command = state.drag(self, drag_start, pos);
                        }
                        _ => (),
                    }
                }

                if frame_state.raw_pointer.secondary_down() {
                    self.zoom_rect = KittyRectangle::from_points(
                        self.pos_to_canvas(drag_start),
                        self.pos_to_canvas(pos),
                    )
                }
            }
        }

        if !(frame_state.raw_pointer.primary_down()) {
            #[expect(clippy::single_match,clippy::collapsible_match)]
            match self.command {
                CommandState::SelectSingle(state) => {
                    match state {
                        SelectSingleState::Dragging(index, _) => {
                            self.command = CommandState::select_single(index);
                        }
                        _ => (),
                    }
                },
                _ => (),
            }
        }

        #[expect(clippy::collapsible_if)]
        if !(frame_state.raw_pointer.secondary_down()) {
            if self.zoom_rect.is_some() {
                self.drag_zoom_apply();
            }
        }
    }

    pub fn do_kitty_commands(&mut self, screen_rect: Rect) {
        for cmd in self.kitty_command_stack.clone() {
            match cmd {
                KittyCommands::CanvasHome => self.initialize_canvas(screen_rect),
            }
        }
        self.kitty_command_stack = vec![];
    }

    pub fn canvas_draw(&self) -> Vec<Shape> {
        let mut result = self.canvas_contents.clone();
        if let CommandState::SelectSingle(SelectSingleState::Selected(index)) = self.command {
            let _ = result.remove(index);
        }
        result.iter().map(|shape| -> egui::Shape {
            self.shape_to_screen(shape.clone())
        }).collect()
    }

    pub fn selection_draw(&self) -> Vec<Shape> {
        match self.command {
            CommandState::SelectSingle(select_state) => {
                select_state.draw(self)
            },
            _ => vec![],
        }
    }

    pub fn pos_to_canvas(&self, pos: Pos2) -> KittyPoint {
        pos2_to_kittypt_t(pos, self.screen_to_canvas())
    }

    pub fn pos_to_screen(&self, pos: KittyPoint) -> Pos2 {
        kittypt_to_pos2_t(pos, self.canvas_to_screen)
    }

    pub fn shape_to_screen(&self, shape: KittyDrawShape) -> Shape {
        kittyds_to_shape(shape, self.canvas_to_screen)
    }

    pub fn drag_zoom_apply(&mut self) {
        self.canvas_to_screen = RectTransform::from_to(
            kittyrect_to_rect(weird_rect_func(self.zoom_rect.clone().unwrap(), rect_to_kittyrect(*self.canvas_to_screen.to()).unwrap())),
            *self.canvas_to_screen.to(),
        );
        self.zoom_rect = None;
    }
}

impl NextCommandInput<()> for Kitty {
    fn next_input(&mut self, _: (), pos: KittyPoint) -> CommandResult {
        match &mut self.command {
            CommandState::Noop => CommandResult::Nothing,
            CommandState::SelectSingle(_state) => CommandResult::Nothing,
            CommandState::Line(state) => state.next_input((self.command_options.line, self.stroke), pos),
            CommandState::Circle(_state) => CommandResult::Nothing,
        }
    }
}

impl Preview<()> for Kitty {
    fn preview(&self, _: (), pos: Pos2) -> Shape {
        match self.command {
            CommandState::Line(state) => state.preview(self, pos),
            _ => Shape::Noop,
        }
    }
}