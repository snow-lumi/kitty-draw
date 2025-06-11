use eframe::egui::Shape;
use eframe::emath::RectTransform;
use eframe::epaint::*;

use crate::util::draw_shapes::KittyDrawShape;
use crate::util::math::shapes::KittyPoint;

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

pub fn pos2_to_kittypt(pos: Pos2, transform: RectTransform) -> KittyPoint {
    let pos_t = transform.transform_pos(pos);
    KittyPoint {
        x: pos_t.x,
        y: pos_t.y,
    }
}

pub fn kittypt_to_pos2(point: KittyPoint, transform: RectTransform) -> Pos2 {
    let pos = Pos2 {
        x: point.x,
        y: point.y,
    };
    transform.transform_pos(pos)
}

pub fn kittyds_to_shape(shape: KittyDrawShape, transform: RectTransform) -> Shape {
    match shape {
        KittyDrawShape::Nothing => Shape::Noop,
        KittyDrawShape::LineSegment(line) => {
            let start= kittypt_to_pos2(line.shape.start, transform);
            let end= kittypt_to_pos2(line.shape.end, transform);
            Shape::LineSegment {
                points: [
                    start,
                    end,
                ],
                stroke: line.stroke,
            }
        }
    }
}