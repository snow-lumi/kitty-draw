use eframe::egui::{Pos2, Shape, Stroke};

use crate::{math::StrokelessTransformExt, structs::{commands::CommandResult, kitty::Kitty, NextCommandInput, Preview}};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LineState {
    Nothing,
    StartPoint(Pos2),
}

impl NextCommandInput<(LineOptions,Stroke)> for LineState {
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

impl Preview<&Kitty> for LineState {
    fn preview(&self, kitty: &Kitty, pos: Pos2) -> Shape {
        match *self {
            Self::Nothing => Shape::Noop,
            Self::StartPoint(pos_1) => {
                Shape::LineSegment {
                    points: [
                        pos_1.transform_kitty_flip(kitty.canvas_to_screen),
                        pos
                    ],
                    stroke: kitty.stroke,
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum LineOptions {
    Separate,
    Connected,
}

impl Default for LineOptions {
    fn default() -> Self {
        Self::Separate
    }
}