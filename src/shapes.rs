use eframe::egui;

pub fn crosshair(rect: egui::Rect, pos: egui::Pos2, focus: bool) -> egui::Shape{
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

pub fn x_shape(pos: egui::Pos2, size: f32, stroke: egui::Stroke) -> egui::Shape{
    let mut r_vec: Vec<egui::Shape> = vec![];
    let bottom_right = pos + (size,size).into();
    let bottom_left = pos + (-size,size).into();
    let top_right = pos + (size,-size).into();
    let top_left = pos + (-size,-size).into();
    r_vec.push(egui::Shape::line_segment([bottom_right,top_left], stroke));
    r_vec.push(egui::Shape::line_segment([bottom_left,top_right], stroke));
    egui::Shape::Vec(r_vec)
}