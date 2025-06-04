use eframe::egui::Pos2;

pub struct KittyLineSegment {
    pub start: Pos2,
    pub end: Pos2,
}

pub struct KittyCircle {
    pub center: Pos2,
    pub radius: f32,
}

pub struct KittyDisc {
    pub center: Pos2,
    pub radius: f32,
}