use eframe::egui::Pos2;
use crate::math::kitty_shapes::*;
use crate::math::pga::dot_prod::KittyDotPGA;
use crate::math::pga::dual::KittyDualPGA;
use crate::math::pga::regressive_prod::KittyRegressivePGA;
use crate::math::pga::wedge_prod::KittyWedgePGA;
use crate::math::pga::*;

pub trait KittyDistance<B> {
    fn distance_kitty(&self, other: B) -> f32;
}

impl KittyDistance<KittyShape> for Pos2 {
    fn distance_kitty(&self, other: KittyShape) -> f32 {
        match other {
            KittyShape::Point(point) => self.distance(point),
            KittyShape::LineSegment(line) => self.distance_kitty(line),
            KittyShape::Disc(disc) => self.distance_kitty(disc),
            KittyShape::Circle(circle) => self.distance_kitty(circle),
            _ => todo!(),
        }
    }
}

impl KittyDistance<KittyLineSegment> for Pos2 {
    fn distance_kitty(&self, other: KittyLineSegment) -> f32 {
        let start: KittyPointPGA = other.start.into();
        let end: KittyPointPGA = other.end.into();
        let point: KittyPointPGA = (*self).into();
        let line = start.regressive_prod(end);
        let perpendicular = point.dot_prod(line);
        let projection_n = line.wedge_prod(perpendicular).normalize();
        let projection: KittyPointPGA = projection_n.into();
        let diff_line = (end - start).dual();
        let diff_projection = (projection - start).dual();
        let len_line_sq = diff_line.dot_prod(diff_line);
        let prj_prod = diff_projection.dot_prod(diff_line);
        if prj_prod < 0.0 {
            other.start.distance(*self)
        } else if prj_prod > len_line_sq {
            other.end.distance(*self)
        } else {
            let projection_p: Pos2 = projection_n.into();
            self.distance(projection_p)
        }
    }
}

impl KittyDistance<KittyDisc> for Pos2 {
    fn distance_kitty(&self, other: KittyDisc) -> f32 {
        (other.center.distance(*self) - other.radius).clamp(0.0, f32::INFINITY)
    }
}

impl KittyDistance<KittyCircle> for Pos2 {
    fn distance_kitty(&self, other: KittyCircle) -> f32 {
        (other.center.distance(*self) - other.radius).abs()
    }
}

impl KittyDistance<KittyDisc> for KittyDisc {
    fn distance_kitty(&self, other: KittyDisc) -> f32 {
        (self.center.distance(other.center) - self.radius - other.radius).clamp(0.0, f32::INFINITY)
    }
}

impl KittyDistance<Pos2> for KittyShape {
    fn distance_kitty(&self, other: Pos2) -> f32 {
        other.distance_kitty(self.clone())
    }
}

impl KittyDistance<Pos2> for KittyLineSegment {
    fn distance_kitty(&self, other: Pos2) -> f32 {
        other.distance_kitty(self.clone())
    }
}

impl KittyDistance<Pos2> for KittyDisc {
    fn distance_kitty(&self, other: Pos2) -> f32 {
        other.distance_kitty(self.clone())
    }
}

impl KittyDistance<Pos2> for KittyCircle {
    fn distance_kitty(&self, other: Pos2) -> f32 {
        other.distance_kitty(self.clone())
    }
}