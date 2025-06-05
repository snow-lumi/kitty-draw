use eframe::egui::{self, Shape, Vec2};
use eframe::egui::{Key, Pos2, Rect};
use eframe::emath::{RectTransform, TSTransform};

use crate::math::collide::KittyCollide;
use crate::math::kitty_shapes::{KittyDisc, KittyShape};
use crate::structs::frame_state::FrameState;
use crate::structs::{NextCommandInput, Preview};
use crate::structs::commands::{CommandOptions, CommandResult, CommandState, KittyCommands};

pub struct Kitty {
    pub command: CommandState,
    pub command_options: CommandOptions,
    pub show_origin: bool,
    pub pointer_absolute: bool,
    pub stroke: egui::Stroke,
    pub x_string: String,
    pub y_string: String,
    pub canvas_to_screen: RectTransform,
    pub canvas_initialized: bool,
    pub canvas_contents: Vec<egui::Shape>,
    pub kitty_command_stack: Vec<KittyCommands>,
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
    }

    pub fn canvas_origin(&self) -> Pos2 {
        self.canvas_to_screen.transform_pos(Pos2::ZERO)
    }

    pub fn screen_to_canvas(&self) -> RectTransform {
        self.canvas_to_screen.inverse()
    }

    pub fn canvas_zoom(&mut self, amount: f32, center: Pos2) {
        let center_in_canvas = self.screen_to_canvas().transform_pos(center);
        let scale_trans = TSTransform::from_translation(center_in_canvas.to_vec2()*-1.0) * TSTransform::from_scaling(amount) * TSTransform::from_translation(center_in_canvas.to_vec2());
        self.canvas_to_screen = RectTransform::from_to(
            scale_trans.mul_rect(*self.canvas_to_screen.from()),
            *self.canvas_to_screen.to(),
        );
    }

    pub fn canvas_drag(&mut self, amount: Vec2) {
        self.canvas_to_screen = RectTransform::from_to(
            TSTransform::from_translation(amount).mul_rect(*self.canvas_to_screen.from()),
            *self.canvas_to_screen.to(),
        );
    }

    pub fn handle_keyboard_input(&mut self, state: &FrameState) {
        if state.keys_pressed.contains(&Key::Escape) {
            self.command = CommandState::Noop;
        }
    }

    pub fn handle_mouse_input_canvas(&mut self, state: &FrameState, pos: Pos2) {

            // scroll zoom
            let scroll_event = state.events
                .iter()
                .find(|e| -> bool {
                    matches!(e, egui::Event::MouseWheel {..})
                });

            if let Some(egui::Event::MouseWheel { unit: _, delta, modifiers }) = scroll_event {
                let amount = match *modifiers {
                    egui::Modifiers::NONE => (1.1_f32).powf(delta.y),
                    egui::Modifiers::ALT => (1.03_f32).powf(delta.y),
                    _ => 1.0,
                };
                self.canvas_zoom(amount, pos);
            }
            
            // middle drag
            if state.raw_pointer.middle_down() {
                self.canvas_drag(state.raw_pointer.delta());
            }

            // left click
            if state.raw_pointer.primary_clicked() {
                match self.next_input((), pos) {
                    CommandResult::Nothing => (),
                    CommandResult::Shape(shape) => {
                        self.canvas_contents.push(shape);
                    }
                }

                println!("pos: {:?}", self.canvas_to_screen.inverse().transform_pos(pos));

                if self.command == CommandState::Noop {
                    let bleh = self.canvas_contents.clone();
                    let woof = self.canvas_to_screen.inverse().transform_pos(pos);
                    let bork = KittyDisc::new(woof, 10.0);
                    let meow: Vec<_> = bleh.iter().filter(|shape| -> bool {
                        let shape_k: KittyShape = KittyShape::from(shape.clone().clone());
                        shape_k.collides(bork.clone())
                    }).collect();
                    println!("{:?}",meow)
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
}

impl NextCommandInput<()> for Kitty {
    fn next_input(&mut self, _: (), pos: Pos2) -> CommandResult {
        let pos_canvas = self.screen_to_canvas().transform_pos(pos);
        match &mut self.command {
            CommandState::Noop => CommandResult::Nothing,
            CommandState::Line(state) => state.next_input((self.command_options.line, self.stroke), pos_canvas),
            CommandState::Circle(_state) => CommandResult::Nothing,
        }
    }
}

impl Preview<()> for Kitty{
    fn preview(&self, _: (), pos: Pos2) -> Shape {
        match self.command {
            CommandState::Line(state) => state.preview(self, pos),
            _ => Shape::Noop,
        }
    }
}