use crate::util::math::shapes::KittyPoint;

pub mod size;
pub mod dual;
pub mod add;
pub mod sub;
pub mod mul;
pub mod dot_prod;
pub mod wedge_prod;
pub mod regressive_prod;
pub mod util;

#[expect(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum KittyMultivectorPGA {
    Zero,
    Scalar(f32), 
    Line(KittyLinePGA),
    Point(KittyPointPGA),
    PointNormal(KittyPointNormalPGA),
    PseudoVec(KittyPseudoVecPGA),
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KittyZeroPGA {
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KittyLinePGA {
    pub e_0: f32,
    pub e_x: f32,
    pub e_y: f32,
}

#[expect(dead_code)]
impl KittyLinePGA {
    pub fn resize(&self) -> Self {
        let factor: f32 = 1.0 / (self.e_x.powi(2) + self.e_y.powi(2)).sqrt();
        *self * factor
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KittyPointPGA {
    pub e_xy: f32,
    pub e_0y: f32,
    pub e_0x: f32,
}

impl From<KittyPointNormalPGA> for KittyPointPGA {
    fn from(value: KittyPointNormalPGA) -> Self {
        Self {
            e_xy: 1.0,
            e_0y: value.e_0y,
            e_0x: value.e_0x,
        }
    }
}

impl KittyPointPGA {
    pub fn normalize(&self) -> KittyPointNormalPGA{
        KittyPointNormalPGA {
            e_0y: self.e_0y/self.e_xy,
            e_0x: self.e_0x/self.e_xy,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KittyPointNormalPGA {
    pub e_0y: f32,
    pub e_0x: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct KittyPseudoVecPGA {
    pub e_0xy: f32,
}

impl From<KittyPoint> for KittyPointNormalPGA {
    fn from(value: KittyPoint) -> Self {
        Self {
            e_0y: value.x,
            e_0x: value.y,
        }
    }
}

impl From<KittyPoint> for KittyPointPGA {
    fn from(value: KittyPoint) -> Self {
        Self {
            e_xy: 1.0,
            e_0y: value.x,
            e_0x: value.y,
        }
    }
}

impl From<KittyPointNormalPGA> for KittyPoint {
    fn from(value: KittyPointNormalPGA) -> Self {
        Self {
            x: value.e_0y,
            y: value.e_0x,
        }
    }
}