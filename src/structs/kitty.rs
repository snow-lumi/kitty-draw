use eframe::egui;
use eframe::egui::Pos2;
use eframe::emath::TSTransform;

use crate::structs::CommandResult;
use crate::structs::NextInput;
use crate::structs::commands::{CommandState, line::LineOptions};

pub struct Kitty {
    pub command: CommandState,
    pub line_options: LineOptions,
    pub show_origin: bool,
    pub pointer_absolute: bool,
    pub stroke: egui::Stroke,
    pub x_string: String,
    pub y_string: String,
    pub canvas_to_screen: TSTransform,
    pub initializing: bool,
    pub canvas_contents: Vec<egui::Shape>,
}

impl Kitty {
    pub fn new() -> Self {
        Self {
            command: CommandState::Noop,
            line_options: LineOptions::Separate,
            show_origin: true,
            pointer_absolute: false,
            stroke: egui::Stroke::new(1.0, egui::Color32::WHITE),
            x_string: String::default(),
            y_string: String::default(),
            canvas_to_screen: TSTransform::IDENTITY,
            initializing: true,
            canvas_contents: vec![],
        }
    }
}

impl NextInput<()> for Kitty {
    fn next_input(&mut self, _: (), pos: Pos2) -> CommandResult {
        match &mut self.command {
            CommandState::Noop => CommandResult::Nothing,
            CommandState::Line(state) => state.next_input((self.line_options, self.stroke), pos),
            CommandState::Circle(_state) => CommandResult::Nothing,
        }
    }
}