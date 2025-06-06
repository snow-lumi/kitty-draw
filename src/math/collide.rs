use eframe::egui::Pos2;

use crate::math::pga::dot_prod::KittyDotPGA;
use crate::math::pga::regressive_prod::KittyRegressivePGA;
use crate::math::pga::wedge_prod::KittyWedgePGA;
use crate::math::kitty_shapes::*;
use crate::math::pga::KittyPointPGA;

pub trait KittyCollide<B> {
    fn collides(&self, other: B) -> bool;
}

impl KittyCollide<KittyLineSegment> for KittyShape {
    fn collides(&self, other: KittyLineSegment) -> bool {
        match self {
            // KittyShape::LineSegment(..) => todo!(),
            KittyShape::Disc(disc) => disc.collides(other),
            KittyShape::Circle(circle) => circle.collides(other),
            _ => false,
        }
    }
}

impl KittyCollide<KittyDisc> for KittyShape {
    fn collides(&self, other: KittyDisc) -> bool {
        match self {
            KittyShape::Point(point) => other.collides(*point),
            KittyShape::LineSegment(line) => other.collides(line.clone()),
            KittyShape::Disc(_disc) => todo!(),
            KittyShape::Circle(_circle) => todo!(),
            _ => false,
        }
    }
}

impl KittyCollide<Pos2> for KittyDisc {
    fn collides(&self, other: Pos2) -> bool {
        self.center.distance(other) <= self.radius
    }
}

impl KittyCollide<KittyLineSegment> for KittyDisc {
    fn collides(&self, other: KittyLineSegment) -> bool {
        if self.collides(other.start) {
            return true;
        }
        if self.collides(other.end) {
            return true;
        }
        let start: KittyPointPGA = other.start.into();
        let end: KittyPointPGA = other.end.into();
        let center: KittyPointPGA = self.center.into();
        let line = start.regressive_prod(end);
        let perpendicular = center.dot_prod(line);
        let projection = line.wedge_prod(perpendicular);
        let projection: Pos2 = projection.normalize().into();
        self.center.distance(projection) <= self.radius
    }
}

impl KittyCollide<KittyDisc> for KittyDisc {
    fn collides(&self, other: KittyDisc) -> bool {
        self.center.distance(other.center) <= (self.radius + other.radius)
    }
}

impl KittyCollide<KittyLineSegment> for KittyCircle {
    fn collides(&self, other: KittyLineSegment) -> bool {
        if (self.center.distance(other.start) <= self.radius)
            && (self.center.distance(other.end) <= self.radius)
        {
            return false;
        }
        if (self.center.distance(other.start) <= self.radius)
            ^ (self.center.distance(other.end) <= self.radius)
        {
            return true;
        }
        let start: KittyPointPGA = other.start.into();
        let end: KittyPointPGA = other.end.into();
        let center: KittyPointPGA = self.center.into();
        let line = start.regressive_prod(end);
        let perpendicular = center.dot_prod(line);
        let projection = line.wedge_prod(perpendicular);
        let projection: Pos2 = projection.normalize().into();
        self.center.distance(projection) <= self.radius
    }
}