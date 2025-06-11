use eframe::egui::{Color32, Stroke};

use crate::util::draw_shapes::KittyDrawShape;

pub trait BoolToggleExt {
    fn toggle(&mut self);
}

impl BoolToggleExt for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}

pub trait ChangeColorExt {
    fn with_color(&self, color: Color32) -> Self;
}

impl ChangeColorExt for KittyDrawShape {
    fn with_color(&self, color: Color32) -> Self {
        match self {
            Self::Nothing => Self::Nothing,
            Self::LineSegment(line) => {
                Self::line_segment(
                    line.shape.clone(),
                    Stroke {
                        width: line.stroke.width,
                        color,
                    },
                )
            }
        }
    }
}