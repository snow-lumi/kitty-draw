use eframe::egui::{Pos2, Shape};

use crate::structs::commands::CommandResult;

pub mod frame_state;
pub mod kitty;
pub mod commands;

pub trait NextCommandInput<O> {
    fn next_input(&mut self, options: O, pos: Pos2) -> CommandResult;
}

pub trait Preview<O> {
    fn preview(&self, options: O, pos: Pos2) -> Shape;
}