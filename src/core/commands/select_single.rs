use eframe::egui::{Color32, Pos2, Rect, Shape, Stroke};

use crate::ui::painter_shapes;
use crate::util::draw_shapes::KittyDrawShape;
use crate::util::extensions::ChangeColorExt;
use crate::util::math::collide::KittyCollide;
use crate::util::math::distance::KittyDistance;
use crate::util::math::shapes::{KittyPoint, LinePoint, ShapePoint};
use crate::util::math::{square_around_point, square_around_pos};
use crate::core::commands::CommandState;
use crate::core::kitty::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SelectSingleState {
    Selected(usize),
    Dragging(usize,ShapePoint)
}

impl SelectSingleState {
    pub fn drag(self, kitty: &mut Kitty, drag_start: Pos2, pos: Pos2) -> CommandState {
        let drag_start_canvas = (*kitty).pos_to_canvas(drag_start);
        let pos_canvas = (*kitty).pos_to_canvas(pos);
        match self {
            Self::Selected(index) => {
                match kitty.canvas_contents.get_mut(index) {
                    None => panic!(),
                    Some(KittyDrawShape::LineSegment(line)) => {
                        let mut arr = LinePoint::ALL.iter()
                            .map(|id| -> (LinePoint, KittyPoint) {
                                (*id, line.shape.clone().get_point(*id))
                            })
                            .map(|(id, point)| -> (LinePoint, bool, f32) {
                                (
                                    id,
                                    square_around_point(point, 4.0).collides(drag_start_canvas),
                                    drag_start_canvas.distance(point),
                                )
                            })
                            .filter_map(|(id,selected,distance)| -> Option<(LinePoint,f32)> {
                                match selected {
                                    true => Some((id,distance)),
                                    false => None,
                                }
                            })
                            .collect::<Vec<_>>();
                        arr.sort_by(|(_,x),(_,y)| -> _ {
                            (*x).total_cmp(y)
                        });
                        if let Some((point_index, _)) = arr.first() {
                            let point = line.shape.get_point_mut(*point_index);
                            *point = pos_canvas;
                            CommandState::SelectSingle(Self::Dragging(index, ShapePoint::Line(*point_index)))
                        } else {
                            CommandState::SelectSingle(self)
                        }
                    },
                    _ => todo!(),
                }
            }
            Self::Dragging(index, point_id ) => {
                match kitty.canvas_contents.get_mut(index) {
                    None => panic!(),
                    Some(KittyDrawShape::LineSegment(line)) => {
                        match point_id {
                            ShapePoint::Line(point_id) => {
                                let point = line.shape.get_point_mut(point_id);
                                *point = pos_canvas;
                                CommandState::SelectSingle(self)
                            }
                        }
                    },
                    _ => todo!(),
                }
            }
        }
    }

    pub fn draw(&self, kitty: &Kitty) -> Vec<Shape> {
        let mut result = vec![];
        
        let (index, point_index): (usize, Option<ShapePoint>) = match *self {
            Self::Selected(index) => (index,None),
            Self::Dragging(index, point_index) => (index,Some(point_index)),
        };
        let selection = kitty.canvas_contents.get(index);
        match selection {
            None => (),
            Some(shape) => {
                result.push(kitty.shape_to_screen(shape.with_color(Color32::ORANGE)));
                #[expect(clippy::single_match)]
                match shape {
                    KittyDrawShape::LineSegment(line) => {
                        result.push(Self::drag_point_shape(kitty.pos_to_screen(line.shape.start), point_index == Some(ShapePoint::line_start())));
                        result.push(Self::drag_point_shape(kitty.pos_to_screen(line.shape.end), point_index == Some(ShapePoint::line_end())));
                    }
                    _ => (),
                }
            },
        }
        result
    }

    pub fn drag_point_rect(pos: Pos2) -> Rect {
        square_around_pos(pos, 4.0)
    }

    pub fn drag_point_shape(pos: Pos2, selected: bool) -> Shape {
        let fill_color = if selected {
            Color32::RED
        } else {
            Color32::BLUE
        };
        painter_shapes::simple_rect_shape(
            Self::drag_point_rect(pos),
            Stroke {
                width: 1.0,
                color: Color32::WHITE,
            },
            fill_color,
        )
    }
}

#[expect(dead_code)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LineExpandDirection {
    Start,
    End,
    Both,
}

#[expect(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct SelectLineOptions {
    pub expand: LineExpandDirection,
    pub expand_x: LineExpandDirection,
    pub expand_y: LineExpandDirection,
}

#[expect(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct SelectSingleOptions {
    pub line: SelectLineOptions,
}