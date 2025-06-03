use eframe::egui::{self, Ui};

use crate::buttons;
use crate::structs::{CommandState, LineOptions, program_state::ProgramState};

fn settings_menu_fn(ui: &mut Ui, state: &mut ProgramState ) {
    match state.command {
        CommandState::Line(_) => {
            let button = ui.button("Connected");
            if button.clicked() {
                if state.line_options == LineOptions::Connected {
                    state.line_options = LineOptions::Separate;
                } else {
                    state.line_options = LineOptions::Connected;
                }
            }
            if state.line_options == LineOptions::Connected {
                button.highlight();
            }
        }
        _ => {ui.label("emty :>");}
    }
    ui.separator();
    ui.label("mew :3");
    ui.add(egui::Slider::new(&mut state.stroke.width, 0.1..=10.0).text("girth"));
    ui.color_edit_button_srgba(&mut state.stroke.color);
    ui.separator();
    ui.add(egui::Slider::new(&mut state.canvas_to_screen.scaling, 0.1..=10.0).text("zoom"));
    buttons::func_button(ui, "Home".to_owned(), || {
        state.canvas_to_screen.translation = (0.0,0.0).into();
    });
    ui.label(format!("mrrrp {:?}",state.command));
}

pub fn settings_menu_menu(state: &mut ProgramState ) -> impl FnMut(&mut Ui) {
    |ui| {settings_menu_fn(ui, state);}
}