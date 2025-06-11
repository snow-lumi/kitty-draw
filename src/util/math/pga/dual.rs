use super::*;

pub trait KittyDualPGA {
    type Output;

    fn dual(self) -> Self::Output;
}

impl KittyDualPGA for KittyZeroPGA {
    type Output = KittyZeroPGA;
    
    fn dual(self) -> Self::Output {
        Self::Output {}
    }
}

impl KittyDualPGA for f32 {
    type Output = KittyPseudoVecPGA;
    
    fn dual(self) -> Self::Output {
        Self::Output {
            e_0xy: self
        }
    }
}

impl KittyDualPGA for KittyLinePGA {
    type Output = KittyPointPGA;

    fn dual(self) -> Self::Output {
        Self::Output {
            e_xy: self.e_0,
            e_0y: -self.e_x,
            e_0x: self.e_y,
        }
    }
}

impl KittyDualPGA for KittyPointPGA {
    type Output = KittyLinePGA;

    fn dual(self) -> Self::Output {
        Self::Output {
            e_0: self.e_xy,
            e_x: -self.e_0y,
            e_y: self.e_0x,
        }
    }
}

impl KittyDualPGA for KittyPointNormalPGA {
    type Output = KittyLinePGA;

    fn dual(self) -> Self::Output {
        Self::Output {
            e_0: 1.0,
            e_x: -self.e_0y,
            e_y: self.e_0x,
        }
    }
}