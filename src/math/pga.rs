pub trait KittyDualPGA<O> {
    fn dual(&self) -> O;
}

pub trait KittyWedgePGA<B,O> {
    fn wedge_prod(&self, other: B) -> O;
}
pub trait KittyDotPGA<B,O> {
    fn dot_prod(&self, other: B) -> O;
}
pub trait KittyRegressivePGA<B,O> {
    fn regressive_prod(&self, other: B) -> O;
}

#[derive(Clone, Copy)]
pub struct KittyPointNormalPGA {
    pub e_0y: f32,
    pub e_0x: f32,
}

impl KittyDualPGA<KittyLinePGA> for KittyPointNormalPGA {
    fn dual(&self) -> KittyLinePGA {
        KittyLinePGA {
            e_0: 1.0,
            e_x: -self.e_0y,
            e_y: self.e_0x,
        }
    }
}

impl KittyDotPGA<KittyLinePGA, KittyLinePGA> for KittyPointPGA {
    fn dot_prod(&self, other: KittyLinePGA) -> KittyLinePGA {
        KittyLinePGA {
            e_0: self.e_0x * other.e_x + self.e_0y * other.e_y,
            e_x: self.e_xy * other.e_y - self.e_0x * other.e_0,
            e_y: - self.e_xy * other.e_x - self.e_0y * other.e_0,
        }
    }
}

impl KittyRegressivePGA<KittyPointPGA, KittyLinePGA> for KittyPointPGA {
    fn regressive_prod(&self, other: KittyPointPGA) -> KittyLinePGA {
        self.dual().wedge_prod(other.dual()).dual()
    }
}

#[derive(Clone, Copy)]
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

impl KittyDualPGA<KittyLinePGA> for KittyPointPGA {
    fn dual(&self) -> KittyLinePGA {
        KittyLinePGA {
            e_0: self.e_xy,
            e_x: -self.e_0y,
            e_y: self.e_0x,
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

#[derive(Clone, Copy)]
pub struct KittyLinePGA {
    pub e_0: f32,
    pub e_x: f32,
    pub e_y: f32,
}

impl KittyDualPGA<KittyPointPGA> for KittyLinePGA {
    fn dual(&self) -> KittyPointPGA {
        KittyPointPGA {
            e_xy: self.e_0,
            e_0y: -self.e_x,
            e_0x: self.e_y,
        }
    }
}

impl KittyDotPGA<KittyPointPGA, KittyLinePGA> for KittyLinePGA {
    fn dot_prod(&self, other: KittyPointPGA) -> KittyLinePGA {
        KittyLinePGA {
            e_0: - self.e_x * other.e_0x - self.e_y * other.e_0y,
            e_x: self.e_0 * other.e_0x - self.e_y * other.e_xy,
            e_y: self.e_0 * other.e_0y + self.e_x * other.e_xy,
        }
    }
}

impl KittyWedgePGA<KittyLinePGA,KittyPointPGA> for KittyLinePGA {
    fn wedge_prod(&self, other: KittyLinePGA) -> KittyPointPGA {
        KittyPointPGA {
            e_xy: self.e_x * other.e_y - self.e_y * other.e_x,
            e_0y: self.e_0 * other.e_y - self.e_y * other.e_0,
            e_0x: self.e_0 * other.e_x - self.e_x * other.e_0,
        }
    }
}