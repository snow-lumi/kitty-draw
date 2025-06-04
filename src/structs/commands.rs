use eframe::egui;

pub mod line;
pub mod circle;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum CommandState {
    Noop,
    Line(line::LineState),
    Circle(circle::CircleState),
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
            Commands::Circle => CommandState::Circle(circle::CircleState::Begin),
            Commands::Line   => CommandState::Line(line::LineState::Nothing),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum CommandResult {
    Nothing,
    Shape(egui::Shape),
}

#[derive(Default)]
pub struct CommandOptions {
    pub line: line::LineOptions
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum KittyCommands {
    CanvasHome
}