use eframe::egui;
use eframe::egui::Pos2;

#[expect(dead_code)]
#[derive(Clone, Debug)]
pub enum KittyShape {
    Nothing,
    Point(Pos2),
    LineSegment(KittyLineSegment),
    Circle(KittyCircle),
    Disc(KittyDisc),
}

#[derive(Clone, Debug)]
pub struct KittyLineSegment {
    pub start: Pos2,
    pub end: Pos2,
}

#[derive(Clone, Debug)]
pub struct KittyCircle {
    pub center: Pos2,
    pub radius: f32,
}

#[derive(Clone, Debug)]
pub struct KittyDisc {
    pub center: Pos2,
    pub radius: f32,
}

impl KittyDisc {
    pub fn new(center: Pos2, radius: f32) -> Self{
        Self {
            center,
            radius,
        }
    }
}

impl From<egui::Shape> for KittyShape {
    fn from(value: egui::Shape) -> Self {
        match value {
            egui::Shape::LineSegment { points, stroke: _ } => {
                KittyShape::LineSegment(KittyLineSegment {
                    start: points[0],
                    end: points[1],
                })
            }
            _ => KittyShape::Nothing,
        }
    }
}