use eframe::egui::{Pos2, Shape, Stroke};

use super::CommandResult;
use super::super::*;
use crate::util::draw_shapes::KittyDrawShape;
use crate::util::math::shapes::{KittyLineSegment, KittyPoint};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum LineState {
    Nothing,
    StartPoint(KittyPoint),
}

impl NextCommandInput<(LineOptions,Stroke)> for LineState {
    fn next_input(&mut self, (options,stroke): (LineOptions,Stroke), pos_in: KittyPoint) -> CommandResult{
        match self {
            Self::Nothing => {
                *self = Self::StartPoint(pos_in);
                CommandResult::Nothing
            },
            Self::StartPoint(pos_1) => {
                let line = KittyDrawShape::line_segment( KittyLineSegment {
                    start: *pos_1,
                    end: pos_in,
                }, stroke);
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
                        kitty.pos_to_screen(pos_1),
                        pos
                    ],
                    stroke: kitty.stroke,
                }
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LineOptions {
    Separate,
    Connected,
}

impl Default for LineOptions {
    fn default() -> Self {
        Self::Separate
    }
}