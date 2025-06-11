use eframe::egui::{Pos2, Shape};

use crate::{core::commands::CommandResult, util::math::shapes::KittyPoint};

pub mod frame_state;
pub mod kitty;
pub mod commands;

pub trait NextCommandInput<O> {
    fn next_input(&mut self, options: O, pos: KittyPoint) -> CommandResult;
}

pub trait Preview<O> {
    fn preview(&self, options: O, pos: Pos2) -> Shape;
}