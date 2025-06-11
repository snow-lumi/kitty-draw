use super::*;

pub trait KittyWedgePGA<B> {
    type Output;

    fn wedge_prod(self, other: B) -> Self::Output;
}

impl KittyWedgePGA<KittyLinePGA> for KittyLinePGA {
    type Output = KittyPointPGA;

    fn wedge_prod(self, other: KittyLinePGA) -> Self::Output {
        Self::Output {
            e_xy: self.e_x * other.e_y - self.e_y * other.e_x,
            e_0y: self.e_0 * other.e_y - self.e_y * other.e_0,
            e_0x: self.e_0 * other.e_x - self.e_x * other.e_0,
        }
    }
}