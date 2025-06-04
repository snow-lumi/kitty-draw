use eframe::egui::Pos2;

use crate::structs::commands::CommandResult;

pub mod frame_data;
pub mod kitty;
pub mod commands;

pub trait NextInput<O> {
    fn next_input(&mut self,options: O, pos: Pos2) -> CommandResult;
}

pub trait Previewable {
    fn toggle(&self) -> bool;
}