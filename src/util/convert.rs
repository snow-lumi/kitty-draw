use eframe::egui::Shape;
use eframe::emath::RectTransform;
use eframe::epaint::*;

use crate::util::draw_shapes::KittyDrawShape;
use crate::util::math::shapes::{KittyPoint, KittyRectangle};

pub fn pos2_to_kittypt(pos: Pos2) -> KittyPoint {
    KittyPoint {
        x: pos.x,
        y: pos.y,
    }
}

// pub fn kittypt_to_pos2(point: KittyPoint) -> Pos2 {
//     Pos2 {
//         x: point.x,
//         y: point.y,
//     }
// }

pub fn pos2_to_kittypt_t(pos: Pos2, transform: RectTransform) -> KittyPoint {
    let pos_t = transform.transform_pos(pos);
    KittyPoint {
        x: pos_t.x,
        y: pos_t.y,
    }
}

pub fn kittypt_to_pos2_t(point: KittyPoint, transform: RectTransform) -> Pos2 {
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
            let start= kittypt_to_pos2_t(line.shape.start, transform);
            let end= kittypt_to_pos2_t(line.shape.end, transform);
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

pub fn kittyrect_to_rect(rect: KittyRectangle) -> Rect {
    Rect {
        min: (*(rect.x_range.start()),*(rect.y_range.start())).into(),
        max: (*(rect.x_range.end()),*(rect.y_range.end())).into(),
    }
}

pub fn rect_to_kittyrect(rect: Rect) -> Option<KittyRectangle> {
    KittyRectangle::from_points(
        pos2_to_kittypt(rect.min),
        pos2_to_kittypt(rect.min),
    )
}

pub fn kittyrect_to_rect_t(rect: KittyRectangle, transform: RectTransform) -> Rect {
    let rect = Rect {
        min: (*(rect.x_range.start()),*(rect.y_range.start())).into(),
        max: (*(rect.x_range.end()),*(rect.y_range.end())).into(),
    };
    transform.transform_rect(rect)
}