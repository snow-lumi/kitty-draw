use eframe::egui;

pub struct FrameData {
    pub events: Vec<egui::Event>,
    pub raw_pointer: egui::PointerState,
}

impl FrameData {
    pub fn new(i: &egui::InputState) -> Self {
        Self {
            events: i.events.clone(),
            raw_pointer: i.pointer.clone(),
        }
    }
}