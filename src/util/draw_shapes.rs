use eframe::egui::Stroke;

use super::math::shapes::*;

#[derive(PartialEq, Clone, Debug)]
pub enum KittyDrawShape {
    Nothing,
    LineSegment(KittyDrawLineSegment),
}

impl KittyDrawShape {
    pub fn get_shape(self) -> KittyShape {
        match self {
            Self::Nothing => KittyShape::Nothing,
            Self::LineSegment(line) => KittyShape::LineSegment(line.shape),
        }
    }

    pub fn line_segment(shape: KittyLineSegment, stroke: Stroke) -> Self {
        Self::LineSegment(KittyDrawLineSegment { shape, stroke })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct KittyDrawLineSegment {
    pub shape: KittyLineSegment,
    pub stroke: Stroke,
}