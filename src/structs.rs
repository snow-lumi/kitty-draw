use eframe::egui::{self, Pos2, Shape, Stroke};

pub mod frame_data;
pub mod program_state;

pub trait NextInput<O> {
    fn next_input(&mut self,options: O, pos: Pos2) -> CommandResult;
}

pub trait Previewable {
    fn toggle(&self) -> bool;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CommandState {
    Noop,
    Line(LineState),
    Circle(CircleState),
}

impl From<CommandState> for Commands {
    fn from(value: CommandState) -> Self {
        match value {
            CommandState::Noop       => Commands::Noop,
            CommandState::Circle(..) => Commands::Circle,
            CommandState::Line(..)   => Commands::Line,
        }
    }
}

impl CommandState {
    pub fn into_command(self) -> Commands {
        let commands: Commands = (self).into();
        commands
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Commands {
    Noop,
    Line,
    Circle,
}

impl Commands {
    pub fn starting_state(&self) -> CommandState {
        match self {
            Commands::Noop   => CommandState::Noop,
            Commands::Circle => CommandState::Circle(CircleState::Begin),
            Commands::Line   => CommandState::Line(LineState::Nothing),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum CommandResult {
    Nothing,
    Shape(egui::Shape),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LineState {
    Nothing,
    StartPoint(Pos2),
}

impl NextInput<(LineOptions,Stroke)> for LineState {
    fn next_input(&mut self, (options,stroke): (LineOptions,Stroke), pos_in: Pos2) -> CommandResult{
        match self {
            Self::Nothing => {
                *self = Self::StartPoint(pos_in);
                CommandResult::Nothing
            },
            Self::StartPoint(pos_1) => {
                let line = Shape::LineSegment { points: [*pos_1,pos_in], stroke };
                *self = match options {
                    LineOptions::Separate => Self::Nothing,
                    LineOptions::Connected => Self::StartPoint(pos_in),
                };
                CommandResult::Shape(line)
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CircleState {
    Begin,
    Finish,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LineOptions {
    Separate,
    Connected,
}