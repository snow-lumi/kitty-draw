use std::ops::{Add, Div, RangeInclusive, Sub};

use eframe::egui::{Pos2, Rect};

use crate::util::math::shapes::{KittyPoint, KittyRectangle};

pub mod shapes;
pub mod collide;
pub mod distance;
pub mod pga;

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

pub fn sort_pair<T: PartialOrd>((first,second): (T,T)) -> Option<(T,T)> {
    if first <= second {
        Some((first, second))
    } else if first > second {
        Some((second, first))
    } else {
        println!("guh!");
        None
    }
}

pub trait KittyLengthExt<T>
    where T: Copy + Sub
{
    fn length(self) -> <T as std::ops::Sub>::Output;
}

impl<T> KittyLengthExt<T> for RangeInclusive<T>
    where T: Copy + Sub
{
    fn length(self) -> <T as std::ops::Sub>::Output {
        *self.end() - *self.start()
    }
}

pub trait KittyMidExt<T>
    where T: Copy + Add,
    <T as std::ops::Add>::Output: Div<<Self as crate::util::math::KittyMidExt<T>>::Divisor>,
{
    type Divisor;

    fn mid(self) -> <<T as std::ops::Add>::Output as std::ops::Div<<Self as crate::util::math::KittyMidExt<T>>::Divisor>>::Output;
}

impl KittyMidExt<f32> for RangeInclusive<f32>
{
    type Divisor = f32;

    fn mid(self) -> f32 {
        (*self.end() + *self.start()) / 2.0
    }
}

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

pub fn weird_rect_func(inner: KittyRectangle, outer: KittyRectangle) -> KittyRectangle {
    let (x_range,y_range) = if inner.aspect_ratio_x_y() < outer.aspect_ratio_x_y() {
        let x_center = inner.x_range.mid();
        let x_radius = inner.y_range.clone().length() * outer.aspect_ratio_x_y() / 2.0;
        (
            (x_center-x_radius)..=(x_center+x_radius),
            inner.y_range,
        )
    } else {
        let y_center = inner.y_range.mid();
        let y_radius = inner.x_range.clone().length() / outer.aspect_ratio_x_y() / 2.0;
        (
            inner.x_range,
            (y_center-y_radius)..=(y_center+y_radius),
        )
    };
    KittyRectangle { x_range, y_range }
}