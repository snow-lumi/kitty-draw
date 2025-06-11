use eframe::egui::{Pos2, Rect};

use crate::util::math::shapes::{KittyPoint, KittyRectangle};

pub mod shapes;
pub mod collide;
pub mod distance;
pub mod pga;

pub fn square_around_pos(pos: Pos2, size: f32) -> Rect {
    Rect {
        min: pos + (-size,-size).into(),
        max: pos + (size,size).into(),
    }
}

pub fn square_around_point(point: KittyPoint, size: f32) -> KittyRectangle {
    KittyRectangle {
        x_range: ((point.x-size)..=(point.x+size)),
        y_range: ((point.y-size)..=(point.y+size)),
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct KittyVec2 {
    pub x: f32,
    pub y: f32,
}

impl From<(f32,f32)> for KittyVec2 {
    fn from((x,y): (f32,f32)) -> Self {
        Self { x, y }
    }
}

impl KittyVec2 {
    pub const ZERO: Self = Self { x: 0.0 , y: 0.0};
}