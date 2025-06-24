use eframe::egui::Stroke;

use super::math::shapes::*;

pub trait KittyGetShape {
    fn get_shape(self) -> KittyShape;
}

#[derive(PartialEq, Clone, Debug)]
pub enum KittyDrawShape {
    Nothing,
    LineSegment(KittyDrawLineSegment),
    Circle(KittyDrawCircle),
}

impl KittyGetShape for KittyDrawShape {
    fn get_shape(self) -> KittyShape {
        match self {
            Self::Nothing => KittyShape::Nothing,
            Self::LineSegment(line) => line.get_shape(),
            Self::Circle(circle) => circle.get_shape(),
        }
    }
}

impl KittyDrawShape {
    pub fn line_segment(shape: KittyLineSegment, stroke: Stroke) -> Self {
        Self::LineSegment(KittyDrawLineSegment { shape, stroke })
    }

    pub fn circle(shape: KittyCircle, stroke: Stroke) -> Self {
        Self::Circle(KittyDrawCircle { shape, stroke })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct KittyDrawLineSegment {
    pub shape: KittyLineSegment,
    pub stroke: Stroke,
}

impl KittyGetShape for KittyDrawLineSegment {
    fn get_shape(self) -> KittyShape {
        KittyShape::LineSegment(self.shape)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct KittyDrawCircle {
    pub shape: KittyCircle,
    pub stroke: Stroke,
}

impl KittyGetShape for KittyDrawCircle {
    fn get_shape(self) -> KittyShape {
        KittyShape::Circle(self.shape)
    }
}