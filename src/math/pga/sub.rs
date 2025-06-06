use std::ops::Sub;

use super::*;

impl Sub<KittyLinePGA> for KittyLinePGA {
    type Output = Self;

    fn sub(self, rhs: KittyLinePGA) -> Self::Output {
        Self::Output {
            e_0: self.e_0 - rhs.e_0,
            e_x: self.e_x - rhs.e_x,
            e_y: self.e_y - rhs.e_y,
        }
    }
}

impl Sub<KittyPointPGA> for KittyPointPGA {
    type Output = Self;

    fn sub(self, rhs: KittyPointPGA) -> Self::Output {
        Self::Output {
            e_xy: self.e_xy - rhs.e_xy,
            e_0y: self.e_0y - rhs.e_0y,
            e_0x: self.e_0x - rhs.e_0x,
        }
    }
}