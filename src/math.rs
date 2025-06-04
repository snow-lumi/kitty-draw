use eframe::egui::{self, Pos2, Shape, Vec2};
use eframe::emath::TSTransform;

pub mod shapes;
pub mod collide;
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
    fn transform_kitty(self, transform: TSTransform) -> Self;
    fn transform_kitty_flip(self, transform: TSTransform) -> Self;
}

impl StrokelessTransformExt for Shape {
    fn transform_kitty(self, transform: TSTransform) -> Self {
        let mut result = self;
        match &mut result {
            Shape::Noop => (),
            Shape::LineSegment { points,..} => {
                *points = points.map(|p| {
                    transform.mul_pos(p)
                });
            },
            Shape::Circle(eframe::epaint::CircleShape { center: c, radius: r,  .. }) => {
                *c = transform.mul_pos(*c);
                *r *= transform.scaling;
            },
            _ => (), // TODO
        }
        result
    }

    fn transform_kitty_flip(self, transform: TSTransform) -> Self {
        let mut result = self;
        match &mut result {
            Shape::Noop => (),
            Shape::LineSegment { points,..} => {
                *points = points.map(|p| {
                    transform.mul_pos(p).flip_y()
                });
            },
            Shape::Circle(eframe::epaint::CircleShape { center: c, radius: r,  .. }) => {
                *c = transform.mul_pos(*c).flip_y();
                *r *= transform.scaling;
            },
            _ => (), // TODO
        }
        result
    }
}

impl StrokelessTransformExt for Vec2 {
    fn transform_kitty(self, transform: TSTransform) -> Self {
        transform.mul_pos(self.to_pos2()).to_vec2()
    }

    fn transform_kitty_flip(self, transform: TSTransform) -> Self {
        let mut result = transform.mul_pos(self.to_pos2()).to_vec2();
        result.y += -1.0;
        result
    }
}

impl StrokelessTransformExt for Pos2 {
    fn transform_kitty(self, transform: TSTransform) -> Self {
        transform.mul_pos(self)
    }

    fn transform_kitty_flip(self, transform: TSTransform) -> Self {
        let mut result = transform.mul_pos(self);
        result.y += -1.0;
        result
    }
}

pub trait FlipYExt {
    fn flip_y(self) -> Self;
}

impl FlipYExt for egui::Vec2 {
    fn flip_y(self) -> Self{
        Vec2 {
            x: self.x,
            y: -self.y,
        }
    }
}

impl FlipYExt for egui::Pos2 {
    fn flip_y(self) -> Self{
        Pos2 {
            x: self.x,
            y: -self.y,
        }
    }
}

// impl FlipYExt for Shape {

impl FlipYExt for Shape {
    fn flip_y(self) -> Self{
        let mut result = self;
        match &mut result {
            Shape::Noop => (),
            Shape::LineSegment { points,..} => {
                points[0].flip_y();
                points[1].flip_y();
            },
            Shape::Circle(eframe::epaint::CircleShape { center, .. }) => {
                center.flip_y();
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