use eframe::egui::{self, Shape};
use eframe::egui::{Key, Pos2, Rect};
use eframe::emath::TSTransform;

use crate::math::StrokelessTransformExt;
use crate::structs::frame_state::FrameState;
use crate::structs::{NextCommandInput, Preview};
use crate::structs::commands::{CommandOptions, CommandResult, CommandState, Commands, KittyCommands};

pub struct Kitty {
    pub command: CommandState,
    pub command_options: CommandOptions,
    pub show_origin: bool,
    pub pointer_absolute: bool,
    pub stroke: egui::Stroke,
    pub x_string: String,
    pub y_string: String,
    pub canvas_to_screen: TSTransform,
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
            canvas_to_screen: TSTransform::IDENTITY,
            canvas_initialized: false,
            canvas_contents: vec![],
            kitty_command_stack: vec![],
        }
    }

    pub fn initialize_canvas(&mut self, screen_rect: Rect) {
        self.canvas_to_screen.translation = screen_rect.center().to_vec2();
        self.canvas_to_screen.scaling = 1.0;
        self.canvas_initialized = true;
    }

    pub fn canvas_origin(&self) -> Pos2 {
        Pos2::ZERO.transform_kitty_flip(self.canvas_to_screen)
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
                let factor = match *modifiers {
                    egui::Modifiers::NONE => (1.1_f32).powf(delta.y),
                    egui::Modifiers::ALT => (1.03_f32).powf(delta.y),
                    _ => 1.0,
                };
                self.canvas_to_screen.scaling *= factor;
                self.canvas_to_screen.translation += (self.canvas_to_screen.translation - pos.to_vec2())*(factor-1.0);
            }
            
            // middle drag
            if state.raw_pointer.middle_down() {
                self.canvas_to_screen.translation += state.raw_pointer.delta();
            }

            // mouse input
            if state.raw_pointer.primary_clicked() {
                match self.next_input((), pos) {
                    CommandResult::Nothing => (),
                    CommandResult::Shape(shape) => {
                        self.canvas_contents.push(shape);
                    }
                }
            }
    }

    pub fn do_kitty_commands(&mut self, screen_rect: Rect) {
        for cmd in self.kitty_command_stack.clone() {
            match cmd {
                KittyCommands::CanvasHome => self.initialize_canvas(screen_rect)
            }
        }
        self.kitty_command_stack = vec![];
    }
}

impl NextCommandInput<()> for Kitty {
    fn next_input(&mut self, _: (), pos: Pos2) -> CommandResult {
        let pos_canvas = pos.transform_kitty_flip(self.canvas_to_screen.inverse());
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