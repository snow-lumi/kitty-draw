use std::collections::HashSet;

use eframe::egui::{self, Key, Pos2, Rect};

pub struct FrameState {
    pub events: Vec<egui::Event>,
    pub raw_pointer: egui::PointerState,
    pub keys_down: HashSet<Key>,
    pub keys_pressed: HashSet<Key>,
}

impl FrameState {
    pub fn new(i: &egui::InputState) -> Self {
        let keys_down = i.keys_down.clone();
        let keys_pressed = keys_down
            .iter()
            .filter_map(|key| -> Option<Key> {
                if i.key_pressed(*key) {
                    Some(*key)
                } else {
                    None
                }
            }
        ).collect::<HashSet<Key>>();
        Self {
            events: i.events.clone(),
            raw_pointer: i.pointer.clone(),
            keys_down,
            keys_pressed,
        }
    }

    pub fn pointer_in(&self, rect: Rect) -> Option<Pos2> {
        self.raw_pointer.latest_pos().filter(|&pos| rect.contains(pos))
    }
}