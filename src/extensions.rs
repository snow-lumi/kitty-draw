use eframe::egui::{Color32, Shape, Stroke};

pub trait ChangeColorExt {
    fn with_color(&self, color: Color32) -> Self;
}

impl ChangeColorExt for Shape {
    fn with_color(&self, color: Color32) -> Self {
        match self {
            Self::Noop => Self::Noop,
            Self::LineSegment { points, stroke } => {
                Self::LineSegment {
                    points: *points,
                    stroke: Stroke {
                        width: stroke.width,
                        color,
                    }
                }
            }
            _ => todo!(),
        }
    }
}