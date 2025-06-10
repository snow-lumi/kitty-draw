use eframe::egui::{Pos2, Rect, Shape};
use eframe::emath::{RectTransform};
use eframe::epaint::{CircleShape, RectShape};

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
    fn transform_kitty(&self, transform: RectTransform) -> Self;
}

impl StrokelessTransformExt for Shape {
    fn transform_kitty(&self, transform: RectTransform) -> Self {
        let mut result = self.clone();
        match &mut result {
            Shape::Noop => (),
            Shape::LineSegment { points,..} => {
                *points = points.map(|p| {
                    transform.transform_pos(p)
                });
            },
            Shape::Circle(CircleShape { center, radius,  .. }) => {
                *center = transform.transform_pos(*center);
                *radius *= transform.scale().x;
            },
            Shape::Rect(RectShape { rect, corner_radius, .. }) => {
                *rect = transform.transform_rect(*rect);
                *corner_radius *= transform.scale().x;
            },
            Shape::Vec(vec) => {
                *vec = vec.iter().map(|shape| -> Shape {
                    shape.transform_kitty(transform)
                }).collect();
            }
            _ => todo!(), // TODO
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

pub fn square_around_pos(pos: Pos2, size: f32) -> Rect {
    Rect {
        min: pos + (-size,-size).into(),
        max: pos + (size,size).into(),
    }
}