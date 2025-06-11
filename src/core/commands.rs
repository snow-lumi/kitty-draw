use crate::{core::commands::select_single::SelectSingleState, util::draw_shapes::KittyDrawShape};

pub mod line;
pub mod circle;
pub mod select_single;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum CommandState {
    Noop,
    SelectSingle(select_single::SelectSingleState),
    Line(line::LineState),
    Circle(circle::CircleState),
}

impl From<CommandState> for Commands {
    fn from(value: CommandState) -> Self {
        match value {
            CommandState::Noop             => Commands::Noop,
            CommandState::SelectSingle(..) => Commands::SelectSingle,
            CommandState::Circle(..)       => Commands::Circle,
            CommandState::Line(..)         => Commands::Line,
        }
    }
}

impl CommandState {
    pub fn into_command(self) -> Commands {
        let commands: Commands = (self).into();
        commands
    }

    pub fn select_single(index: usize) -> Self {
        Self::SelectSingle(SelectSingleState::Selected(index))
    }

    pub fn idling(&self) -> bool {
        matches!(*self, Self::Noop | Self::SelectSingle(..))
    }

    pub fn selecting(&self) -> bool {
        matches!(*self, Self::SelectSingle(..))
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Commands {
    Noop,
    SelectSingle,
    Line,
    Circle,
}

impl Commands {
    pub fn starting_state(&self) -> CommandState {
        match self {
            Commands::Noop         => CommandState::Noop,
            Commands::SelectSingle => CommandState::Noop,
            Commands::Circle       => CommandState::Circle(circle::CircleState::Begin),
            Commands::Line         => CommandState::Line(line::LineState::Nothing),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
pub enum CommandResult {
    Nothing,
    Shape(KittyDrawShape),
}

#[derive(Default, Debug)]
pub struct CommandOptions {
    pub line: line::LineOptions
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum KittyCommands {
    CanvasHome
}