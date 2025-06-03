use std::result;

use eframe::egui::{self, Pos2, Resize, Shape, Ui, Vec2};
use eframe::emath::{self, TSTransform};

trait BoolToggleExt {
    fn toggle(&mut self);
}

impl BoolToggleExt for bool {
    fn toggle(&mut self) {
        *self = !*self;
    }
}

trait StrokelessTransformExt {
    fn transform_kitty(self, transform: TSTransform) -> Self;
    fn transform_kitty_flip(self, transform: TSTransform) -> Self;
}

impl StrokelessTransformExt for Shape {
    fn transform_kitty(self, transform: TSTransform) -> Self {
        let mut result = self;
        match &mut result {
            Shape::Noop => (),
            Shape::LineSegment { points,..} => {
                *points = points.map(|p| {
                    transform.mul_pos(p)
                });
            },
            Shape::Circle(eframe::epaint::CircleShape { center: c, radius: r,  .. }) => {
                *c = transform.mul_pos(*c);
                *r *= transform.scaling;
            },
            _ => (), // TODO
        }
        result
    }

    fn transform_kitty_flip(self, transform: TSTransform) -> Self {
        let mut result = self;
        match &mut result {
            Shape::Noop => (),
            Shape::LineSegment { points,..} => {
                *points = points.map(|p| {
                    transform.mul_pos(p).flip_y()
                });
            },
            Shape::Circle(eframe::epaint::CircleShape { center: c, radius: r,  .. }) => {
                *c = transform.mul_pos(*c).flip_y();
                *r *= transform.scaling;
            },
            _ => (), // TODO
        }
        result
    }
}

impl StrokelessTransformExt for egui::Vec2 {
    fn transform_kitty(self, transform: TSTransform) -> Self {
        transform.mul_pos(self.to_pos2()).to_vec2()
    }

    fn transform_kitty_flip(self, transform: TSTransform) -> Self {
        let mut result = transform.mul_pos(self.to_pos2()).to_vec2();
        result.y += -1.0;
        result
    }
}

impl StrokelessTransformExt for egui::Pos2 {
    fn transform_kitty(self, transform: TSTransform) -> Self {
        transform.mul_pos(self)
    }

    fn transform_kitty_flip(self, transform: TSTransform) -> Self {
        let mut result = transform.mul_pos(self);
        result.y += -1.0;
        result
    }
}


trait FlipYExt {
    fn flip_y(self) -> Self;
}

impl FlipYExt for egui::Vec2 {
    fn flip_y(self) -> Self{
        Vec2 {
            x: self.x,
            y: -self.y,
        }
    }
}

impl FlipYExt for egui::Pos2 {
    fn flip_y(self) -> Self{
        Pos2 {
            x: self.x,
            y: -self.y,
        }
    }
}

// impl FlipYExt for Shape {

impl FlipYExt for Shape {
    fn flip_y(self) -> Self{
        let mut result = self;
        match &mut result {
            Shape::Noop => (),
            Shape::LineSegment { points,..} => {
                points[0].flip_y();
                points[1].flip_y();
            },
            Shape::Circle(eframe::epaint::CircleShape { center, .. }) => {
                center.flip_y();
            },
            _ => (), // TODO
        }
        result
    }
}

trait NextInput<O> {
    fn next_input(&mut self,options: O, pos: Pos2) -> CommandResult;
}

struct ProgramStufs {
    command: CommandState,
    line_options: LineOptions,
    show_origin: bool,
    pointer_absolute: bool,
    stroke: egui::Stroke,
}

impl ProgramStufs {
    pub fn new() -> Self {
        Self {
            command: CommandState::Noop,
            line_options: LineOptions::Separate,
            show_origin: true,
            pointer_absolute: false,
            stroke: egui::Stroke::new(1.0, egui::Color32::WHITE),
        }
    }
}

impl NextInput<()> for ProgramStufs {
    fn next_input(&mut self, _: (), pos: Pos2) -> CommandResult {
        match &mut self.command {
            CommandState::Noop => CommandResult::Nothing,
            CommandState::Line(state) => state.next_input((self.line_options, self.stroke), pos),
            CommandState::Circle(_state) => CommandResult::Nothing,
        }
    }
}

trait Previewable {
    fn toggle(&self) -> bool;
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum CommandState {
    Noop,
    Line(LineState),
    Circle(CircleState),
}

impl From<CommandState> for Commands {
    fn from(value: CommandState) -> Self {
        match value {
            CommandState::Noop       => Commands::Noop,
            CommandState::Circle(..) => Commands::Circle,
            CommandState::Line(..)   => Commands::Line,
        }
    }
}

impl CommandState {
    fn into_command(self) -> Commands {
        let commands: Commands = (self).into();
        commands
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Commands {
    Noop,
    Line,
    Circle,
}

impl Commands {
    fn starting_state(&self) -> CommandState {
        match self {
            Commands::Noop   => CommandState::Noop,
            Commands::Circle => CommandState::Circle(CircleState::Begin),
            Commands::Line   => CommandState::Line(LineState::FirstPoint),
        }
    }
}

#[derive(PartialEq, Clone, Debug)]
enum CommandResult {
    Nothing,
    Shape(egui::Shape),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum LineState {
    FirstPoint,
    SecondPoint(Pos2),
}

impl NextInput<(LineOptions,egui::Stroke)> for LineState {
    fn next_input(&mut self, (options,stroke): (LineOptions,egui::Stroke), pos_in: Pos2) -> CommandResult{
        match self {
            Self::FirstPoint => {
                *self = Self::SecondPoint(pos_in);
                        println!("mew :3");

                CommandResult::Nothing
            },
            Self::SecondPoint(pos_1) => {
                let line = egui::Shape::LineSegment { points: [*pos_1,pos_in], stroke };
                *self = match options {
                    LineOptions::Separate => Self::FirstPoint,
                    LineOptions::Connected => Self::SecondPoint(pos_in),
                };
                CommandResult::Shape(line)
            }
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum CircleState {
    Begin,
    Finish,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum LineOptions {
    Separate,
    Connected,
}

fn command_button(ui: &mut Ui, mrrp: Commands, state: &mut ProgramStufs) {
    let button1 = ui.button(format!("{:?}",mrrp));
    if button1.clicked() {
        state.command = if state.command.into_command() == mrrp {
            CommandState::Noop
        } else {
            mrrp.starting_state()
        }
    }
    if state.command.into_command() == mrrp {
        button1.highlight();
    }
}

fn bool_button(ui: &mut Ui, name: String, state: &mut bool) {
    let button = ui.button(name);
    if button.clicked() {
        state.toggle();
    }
    if *state {
        button.highlight();
    }
}

fn func_button<F: FnOnce()>(ui: &mut Ui, name: String, func: F) {
    let button = ui.button(name);
    if button.clicked() {
        func();
    }
}

fn crosshair(rect: egui::Rect, pos: Pos2, focus: bool) -> egui::Shape{
    let c_defocus = egui::Color32::from_gray(100);
    let c_x = egui::Color32::from_rgb(189, 18, 11);
    let c_y = egui::Color32::from_rgb(17, 120, 5);
    let c_z = egui::Color32::from_rgb(18, 85, 220);
    let mut r_vec: Vec<egui::Shape> = vec![];
    if focus {
        r_vec.push(egui::Shape::hline(rect.x_range(), pos.y, (1.0,c_x)));
        r_vec.push(egui::Shape::vline(pos.x, rect.y_range(), (1.0,c_y)));
        r_vec.push(egui::Shape::circle_stroke(pos, 10.0, (1.0,c_z)));
    } else {
        r_vec.push(egui::Shape::hline(rect.x_range(), pos.y, (1.0,c_defocus)));
        r_vec.push(egui::Shape::vline(pos.x, rect.y_range(), (1.0,c_defocus)));
        r_vec.push(egui::Shape::circle_stroke(pos, 10.0, (1.0,c_defocus)));
    }
    egui::Shape::Vec(r_vec)
}

fn x_shape(pos: Pos2, size: f32, stroke: egui::Stroke) -> egui::Shape{
    let mut r_vec: Vec<egui::Shape> = vec![];
    let bottom_right = pos + (size,size).into();
    let bottom_left = pos + (-size,size).into();
    let top_right = pos + (size,-size).into();
    let top_left = pos + (-size,-size).into();
    r_vec.push(egui::Shape::line_segment([bottom_right,top_left], stroke));
    r_vec.push(egui::Shape::line_segment([bottom_left,top_right], stroke));
    egui::Shape::Vec(r_vec)
}

// (1.0,egui::Color32::from_rgb(18, 150, 200)

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([640.0, 480.0]),
        ..Default::default()
    };

    // Our application state:
    let mut x_string = String::default();
    let mut y_string = String::default();
    let mut canvas: Vec<egui::Shape> = vec![];
    let mut last_click_pos = Pos2::default();
    let mut state = ProgramStufs::new();
    let mut canvas_to_screen = emath::TSTransform::IDENTITY;
    let mut initializing = true;


    eframe::run_simple_native("kitty draw", options, move |ctx, _frame| {

        let events = ctx.input(|i| -> Vec<egui::Event> {i.events.clone()});
        let raw_pointer = ctx.input(|i| -> egui::PointerState {i.pointer.clone()});

        // menu where you can choose commands
        egui::TopBottomPanel::top("woof")
            .show_separator_line(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {

                    command_button(ui, Commands::Line, &mut state);
                    command_button(ui, Commands::Circle, &mut state);

                });
            });

        // menu that controls canvas and pointer behavior
        egui::TopBottomPanel::bottom("meow")
            .show_separator_line(true)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // pointer coordinates
                    ui.label("x: ");
                    ui.add_sized((70.0,20.0), egui::TextEdit::singleline(&mut x_string));
                    ui.label("y: ");
                    ui.add_sized((70.0,20.0), egui::TextEdit::singleline(&mut y_string));

                    // pointer behavior
                    bool_button(ui, "Absolute".to_string(), &mut state.pointer_absolute);

                    // origin behavior
                    bool_button(ui, "Show Origin".to_string(), &mut state.show_origin);
                });
            });

        // context menu that controls the current command
        egui::SidePanel::right("boioing").show(ctx, |ui| {
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
            ui.add(egui::Slider::new(&mut canvas_to_screen.scaling, 0.1..=10.0).text("zoom"));
            func_button(ui, "Home".to_owned(), || {
                canvas_to_screen.translation = (0.0,0.0).into();
            });
            ui.label(format!("mrrrp {:?}",state.command));
        });

        egui::CentralPanel::default().show(ctx, |ui| {

            // make canvas
            let available_space = ui.available_size();
            let (response, painter) = ui.allocate_painter(available_space, egui::Sense::hover());
            let rect = response.rect;

            // generic grey color
            let mlem = egui::Stroke::new(1.0, egui::Color32::from_gray(100));

            // fake calculate origin
            if initializing {
                canvas_to_screen.translation = rect.center().to_vec2();
                initializing = false;
            }

            // draw origin
            if state.show_origin {
                let origin = Pos2::ZERO.transform_kitty_flip(canvas_to_screen);
                painter.hline(rect.x_range(), origin.y, mlem);
                painter.vline(origin.x, rect.y_range(), mlem);
            }

            // draw the image
            painter.extend(canvas.clone().iter().map(|shape| -> egui::Shape {
                shape.clone().transform_kitty(canvas_to_screen)
            }));

            // handle mouse thingies
            let pointer_pos = raw_pointer.latest_pos().filter(|&raw_pos| rect.contains(raw_pos));
            match pointer_pos {
                None => (),
                Some(pos) => {

                    // scroll zoom
                    let scroll_event = events
                        .iter()
                        .find(|e| -> bool {
                            matches!(e, egui::Event::MouseWheel {..})
                        });

                    if let Some(egui::Event::MouseWheel { unit: _, delta, modifiers }) = scroll_event {
                        let factor = match *modifiers {
                            egui::Modifiers::NONE => (1.055_f32).powf(delta.y),
                            egui::Modifiers::ALT => (1.022_f32).powf(delta.y),
                            _ => 1.0,
                        };
                        canvas_to_screen.scaling *= factor;
                        canvas_to_screen.translation += (canvas_to_screen.translation - pos.to_vec2())*(factor-1.0);
                    }
                    
                    // middle drag
                    if raw_pointer.middle_down() {
                        canvas_to_screen.translation += raw_pointer.delta();
                    }

                    // pointer in canvas coords
                    let pos_canvas = pos.transform_kitty_flip(canvas_to_screen.inverse());
                    canvas_to_screen.inverse().mul_pos(pos);

                    // hide mouse
                    ctx.output_mut(|output| {
                        output.cursor_icon = egui::CursorIcon::None
                    });

                    // draw mouse crosshair: color if relative, grey if absolute (cuz it doesnt do anything)
                    painter.add(crosshair(rect, pos, !state.pointer_absolute));

                    // calculate where the user wants the position of the pointer
                    let pointer_offset: egui::Vec2 = (x_string.parse().unwrap_or(0.0),- y_string.parse().unwrap_or(0.0)).into();
                    let des_pointer = match state.pointer_absolute {
                        true  => Pos2::ZERO.transform_kitty_flip(canvas_to_screen.inverse())+pointer_offset,
                        false => pos+pointer_offset,
                    };

                    // draw the position of the thingy ([mouse + offset] or absolute position)
                    let stroke_cursor = egui::Stroke::new(1.0, egui::Color32::from_rgb(18, 100, 210));
                    painter.add(x_shape(des_pointer, 5.0, stroke_cursor));

                    // current command preview
                    // let preview_current = if bleh {
                    //     egui::Shape::LineSegment { points: [last_click_pos,des_pointer], stroke: usr_stroke }
                    // } else {
                    //     egui::Shape::Noop
                    // };
                    
                    if raw_pointer.primary_clicked() {
                        last_click_pos = des_pointer;
                        match state.next_input((), pos_canvas) {
                            CommandResult::Nothing => (),
                            CommandResult::Shape(shape) => {
                                canvas.push(shape);
                            }
                        }
                        println!("{:?}",state.command)
                    }
                    //painter.add(preview_current);
                }
            }

            // esc to abort command
            if ui.input(|i| -> bool {i.key_pressed(egui::Key::Escape)}) {
                state.command = CommandState::Noop;
            }
        });

    })
}