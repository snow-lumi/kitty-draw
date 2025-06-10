use eframe::egui::{Color32, Pos2, Shape, Stroke};

use crate::canvas_shapes;
use crate::extensions::ChangeColorExt;
use crate::math::{square_around_pos, StrokelessTransformExt};
use crate::structs::commands::CommandState;
use crate::structs::kitty::*;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum LinePoint {
    Start,
    End,
}

impl TryFrom<usize> for LinePoint {
    type Error = ();
    
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(LinePoint::Start),
            1 => Ok(LinePoint::End),
            _ => Err(())
        }
    }
}

impl From<LinePoint> for usize {
    fn from(value: LinePoint) -> Self {
        match value {
            LinePoint::Start => 0,
            LinePoint::End   => 1,
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum ShapePoint {
    Line(LinePoint),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SelectSingleState {
    Selected(usize),
    Dragging(usize,ShapePoint)
}

impl SelectSingleState {
    pub fn drag(self, kitty: &mut Kitty, drag_start: Pos2, pos: Pos2) -> CommandState {
        let drag_start_canvas = (*kitty).screen_to_canvas().transform_pos(drag_start);
        let pos_canvas = (*kitty).screen_to_canvas().transform_pos(pos);
        match self {
            Self::Selected(index) => {
                match kitty.canvas_contents.get_mut(index) {
                    None => panic!(),
                    Some(Shape::LineSegment { points, stroke: _ }) => {
                        let mut arr = [0_usize,1].iter()
                            .filter_map(|index| -> Option<(usize, &Pos2)> {
                                points.get(*index).map(|ptr| -> (usize, &Pos2) {
                                    (*index, ptr)
                                })
                            })
                            .map(|(i, point)| -> (usize, bool, f32) {
                                (
                                    i,
                                    square_around_pos(*point, 4.0).contains(drag_start_canvas),
                                    drag_start_canvas.distance(*point),
                                )
                            })
                            .filter_map(|(i,selected,distance)| -> Option<(usize,f32)> {
                                match selected {
                                    true => Some((i,distance)),
                                    false => None,
                                }
                            })
                            .collect::<Vec<_>>();
                        arr.sort_by(|(_,x),(_,y)| -> _ {
                            (*x).total_cmp(y)
                        });
                        println!("mew {:?}", arr);
                        if let Some((point_index, _)) = arr.first() {
                            if let Some(point) = points.get_mut(*point_index) {
                                *point = pos_canvas;
                                CommandState::SelectSingle(Self::Dragging(index, ShapePoint::Line((*point_index).try_into().unwrap())))
                            } else {
                                panic!()
                            }
                        } else {
                            panic!()
                        }
                    },
                    _ => todo!(),
                }
            }
            Self::Dragging(index,point_type ) => {
                match kitty.canvas_contents.get_mut(index) {
                    None => panic!(),
                    Some(Shape::LineSegment { points, stroke: _ }) => {
                        match point_type {
                            ShapePoint::Line(point_index) => {
                                match points.get_mut::<usize>(point_index.into()) {
                                    None => panic!(),
                                    Some(point) => {
                                        *point = pos_canvas;
                                        CommandState::SelectSingle(self)
                                    }
                                }
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
        
        let index = match *self {
            Self::Selected(index) => index,
            Self::Dragging(index, _) => index,
        };
        let selection = kitty.canvas_contents.get(index);
        match selection {
            None => (),
            Some(shape) => {
                result.push(shape.with_color(Color32::ORANGE).transform_kitty(kitty.canvas_to_screen));
                #[expect(clippy::single_match)]
                match shape {
                    Shape::LineSegment { points, stroke: _ } => {
                        result.push(canvas_shapes::square_shape(
                            kitty.canvas_to_screen.transform_pos(points[0]),
                            4.0,
                            Stroke {
                                width: 1.0,
                                color: Color32::WHITE,
                            },
                            Color32::RED,
                        ));
                        result.push(canvas_shapes::square_shape(
                            kitty.canvas_to_screen.transform_pos(points[1]),
                            4.0,
                            Stroke {
                                width: 1.0,
                                color: Color32::WHITE,
                            },
                            Color32::RED,
                        ));
                    }
                    _ => (),
                }
            },
        }
        result
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