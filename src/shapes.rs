use eframe::egui::{Color32, Pos2, Rect, Shape, Stroke};

pub fn cursor_crosshair(rect: Rect, pos: Pos2, focus: bool) -> Shape {
    let c_defocus = Color32::from_gray(100);
    let c_x = Color32::from_rgb(189, 18, 11);
    let c_y = Color32::from_rgb(17, 120, 5);
    let c_z = Color32::from_rgb(18, 85, 220);
    let mut r_vec: Vec<Shape> = vec![];
    if focus {
        r_vec.push(Shape::hline(rect.x_range(), pos.y, (1.0,c_x)));
        r_vec.push(Shape::vline(pos.x, rect.y_range(), (1.0,c_y)));
        r_vec.push(Shape::circle_stroke(pos, 10.0, (1.0,c_z)));
    } else {
        r_vec.push(Shape::hline(rect.x_range(), pos.y, (1.0,c_defocus)));
        r_vec.push(Shape::vline(pos.x, rect.y_range(), (1.0,c_defocus)));
        r_vec.push(Shape::circle_stroke(pos, 10.0, (1.0,c_defocus)));
    }
    Shape::Vec(r_vec)
}

pub fn x_shape(pos: Pos2, size: f32, stroke: Stroke) -> Shape {
    let bottom_right = pos + (size,size).into();
    let bottom_left = pos + (-size,size).into();
    let top_right = pos + (size,-size).into();
    let top_left = pos + (-size,-size).into();
    Shape::Vec(vec![
        Shape::line_segment([bottom_right,top_left], stroke),
        Shape::line_segment([bottom_left,top_right], stroke),
    ])
}

pub fn simple_crosshair(rect: Rect, pos: Pos2, stroke: Stroke) -> Shape {
    let mut r_vec: Vec<Shape> = vec![];
    let mut empty = true;
    if rect.x_range().contains(pos.x) {
        r_vec.push(Shape::vline(pos.x, rect.y_range(), stroke));
        empty = false;
    }
    if rect.y_range().contains(pos.y) {
        r_vec.push(Shape::hline(rect.x_range(), pos.y, stroke));
        empty = false;
    }
    if empty {
        Shape::Noop
    } else {
        Shape::Vec(r_vec)
    }
}