use super::distance::KittyDistance;
use super::shapes::*;
use super::pga::KittyPointPGA;
use super::pga::size::KittySizePGA;
use super::pga::dual::KittyDualPGA;
use super::pga::dot_prod::KittyDotPGA;
use super::pga::regressive_prod::KittyRegressivePGA;
use super::pga::wedge_prod::KittyWedgePGA;

pub trait KittyCollide<B> {
    fn collides(&self, other: B) -> bool;
}

impl KittyCollide<KittyLineSegment> for KittyShape {
    fn collides(&self, other: KittyLineSegment) -> bool {
        match self {
            Self::Nothing => false,
            Self::Point(..) => todo!(),
            Self::LineSegment(..) => todo!(),
            Self::Rectangle(..) => todo!(),
            Self::Disc(disc) => disc.collides(other),
            Self::Circle(circle) => circle.collides(other),
        }
    }
}

impl KittyCollide<KittyDisc> for KittyShape {
    fn collides(&self, other: KittyDisc) -> bool {
        match self {
            Self::Nothing => false,
            Self::Point(point) => other.collides(*point),
            Self::LineSegment(line) => other.collides(line.clone()),
            Self::Rectangle(..) => todo!(),
            Self::Disc(_disc) => todo!(),
            Self::Circle(_circle) => todo!(),
        }
    }
}

impl KittyCollide<KittyPoint> for KittyDisc {
    fn collides(&self, other: KittyPoint) -> bool {
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
        let projection_n = projection.normalize().into();
        let intersects_with_inf_line = self.center.distance(projection_n) <= self.radius;
        let bla = (KittyPointPGA::from(projection.normalize()) - start).dual();
        let mrrp = (end - start).dual();
        let meow = bla.dot_prod(mrrp);
        let bark = mrrp.size_sq();
        let woof = meow >= 0.0;
        let yelp = meow <= bark;
        println!("bla: {:#?}", bla);
        println!("mrrp: {:#?}", mrrp);
        println!("mew :3 {meow} {bark}");
        intersects_with_inf_line && woof && yelp
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
        let projection: KittyPoint = projection.normalize().into();
        self.center.distance(projection) <= self.radius
    }
}

impl KittyCollide<KittyPoint> for KittyRectangle {
    fn collides(&self, other: KittyPoint) -> bool {
        self.x_range.contains(&other.x) && self.y_range.contains(&other.y)
    }
}