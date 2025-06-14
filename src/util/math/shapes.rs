use std::ops::{Add, RangeInclusive};

use crate::util::math::KittyVec2;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LinePoint {
    Start,
    End,
}

impl LinePoint {
    pub const ALL: [Self;2] = [Self::Start,Self::End];
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ShapePoint {
    Line(LinePoint),
}

impl ShapePoint {
    pub fn line_start() -> Self {
        Self::Line(LinePoint::Start)
    }

    pub fn line_end() -> Self {
        Self::Line(LinePoint::End)
    }
}

#[expect(dead_code)]
#[derive(Clone, Debug)]
pub enum KittyShape {
    Nothing,
    Point(KittyPoint),
    LineSegment(KittyLineSegment),
    Rectangle(KittyRectangle),
    Circle(KittyCircle),
    Disc(KittyDisc),
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct KittyPoint {
    pub x: f32,
    pub y: f32,
}

impl Add<KittyVec2> for KittyPoint {
    type Output = KittyPoint;

    fn add(self, rhs: KittyVec2) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl KittyPoint {
    #[expect(dead_code)]
    pub const ZERO: Self = Self { x: 0.0 , y: 0.0};
}

#[derive(PartialEq, Clone, Debug)]
pub struct KittyLineSegment {
    pub start: KittyPoint,
    pub end: KittyPoint,
}

impl KittyLineSegment {
    pub fn get_point(self, id: LinePoint) -> KittyPoint {
        match id {
            LinePoint::Start => self.start,
            LinePoint::End => self.end,
        }
    }

    pub fn get_point_mut(&mut self, id: LinePoint) -> &mut KittyPoint {
        match id {
            LinePoint::Start => &mut self.start,
            LinePoint::End => &mut self.end,
        }
    }
}

#[derive(Clone, Debug)]
pub struct KittyRectangle {
    pub x_range: RangeInclusive<f32>,
    pub y_range: RangeInclusive<f32>,
}

#[derive(Clone, Debug)]
pub struct KittyCircle {
    pub center: KittyPoint,
    pub radius: f32,
}

#[derive(Clone, Debug)]
pub struct KittyDisc {
    pub center: KittyPoint,
    pub radius: f32,
}

impl KittyDisc {
    pub fn new(center: KittyPoint, radius: f32) -> Self{
        Self {
            center,
            radius,
        }
    }
}