use eframe::egui::{Pos2, Shape};
use eframe::emath::{RectTransform};

pub mod kitty_shapes;
pub mod collide;
pub mod distance;
pub mod pga;

pub trait BoolToggleExt {
    fn toggle(&mut self);
}

impl BoolToggleExt for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}

pub trait StrokelessTransformExt {
    fn transform_kitty(self, transform: RectTransform) -> Self;
}

impl StrokelessTransformExt for Shape {
    fn transform_kitty(self, transform: RectTransform) -> Self {
        let mut result = self;
        match &mut result {
            Shape::Noop => (),
            Shape::LineSegment { points,..} => {
                *points = points.map(|p| {
                    transform.transform_pos(p)
                });
            },
            Shape::Circle(eframe::epaint::CircleShape { center: c, radius: r,  .. }) => {
                *c = transform.transform_pos(*c);
                *r *= transform.scale().x;
            },
            _ => (), // TODO
        }
        result
    }
}

impl From<Pos2> for pga::KittyPointNormalPGA {
    fn from(value: Pos2) -> Self {
        Self {
            e_0y: value.x,
            e_0x: value.y,
        }
    }
}

impl From<Pos2> for pga::KittyPointPGA {
    fn from(value: Pos2) -> Self {
        Self {
            e_xy: 1.0,
            e_0y: value.x,
            e_0x: value.y,
        }
    }
}

impl From<pga::KittyPointNormalPGA> for Pos2 {
    fn from(value: pga::KittyPointNormalPGA) -> Self {
        Self {
            x: value.e_0y,
            y: value.e_0x,
        }
    }
}