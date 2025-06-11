use super::shapes::*;
use super::pga::dot_prod::KittyDotPGA;
use super::pga::dual::KittyDualPGA;
use super::pga::regressive_prod::KittyRegressivePGA;
use super::pga::wedge_prod::KittyWedgePGA;
use super::pga::*;

pub trait KittyDistance {
    fn distance(self, other: KittyPoint) -> f32;
}

impl KittyDistance for KittyPoint {
    fn distance(self, other: KittyPoint) -> f32 {
        ((self.x-other.x).powi(2)+(self.y-other.y).powi(2)).sqrt()
    }
}

impl KittyDistance for KittyShape {
    fn distance(self, other: KittyPoint) -> f32 {
        match self {
            KittyShape::Point(point)      => point.distance(other),
            KittyShape::LineSegment(line) => line.distance(other),
            KittyShape::Disc(disc)        => disc.distance(other),
            KittyShape::Circle(circle)    => circle.distance(other),
            _ => todo!(),
        }
    }
}

impl KittyDistance for KittyLineSegment {
    fn distance(self, other: KittyPoint) -> f32 {
        let start: KittyPointPGA = self.start.into();
        let end: KittyPointPGA = self.end.into();
        let point: KittyPointPGA = other.into();
        let line = start.regressive_prod(end);
        let perpendicular = point.dot_prod(line);
        let projection_n = line.wedge_prod(perpendicular).normalize();
        let projection: KittyPointPGA = projection_n.into();
        let diff_line = (end - start).dual();
        let diff_projection = (projection - start).dual();
        let len_line_sq = diff_line.dot_prod(diff_line);
        let prj_prod = diff_projection.dot_prod(diff_line);
        if prj_prod < 0.0 {
            self.start.distance(other)
        } else if prj_prod > len_line_sq {
            self.end.distance(other)
        } else {
            let projection_p: KittyPoint = projection_n.into();
            other.distance(projection_p)
        }
    }
}

impl KittyDistance for KittyCircle {
    fn distance(self, other: KittyPoint) -> f32 {
        (self.center.distance(other) - self.radius).abs()
    }
}

impl KittyDistance for KittyDisc {
    fn distance(self, other: KittyPoint) -> f32 {
        (self.center.distance(other) - self.radius).clamp(0.0, f32::INFINITY)
    }
}