use eframe::egui::Pos2;

use crate::math::shapes::{KittyCircle, KittyDisc, KittyLineSegment};
use crate::math::pga::{KittyDotPGA, KittyPointPGA, KittyRegressivePGA, KittyWedgePGA};

pub trait KittyCollide<B> {
    fn collides(&self, other: B) -> bool;
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

impl KittyCollide<KittyLineSegment> for KittyCircle {
    fn collides(&self, other: KittyLineSegment) -> bool {
        if (self.center.distance(other.start) <= self.radius)
            && (self.center.distance(other.end) <= self.radius)
        {
            return false;
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