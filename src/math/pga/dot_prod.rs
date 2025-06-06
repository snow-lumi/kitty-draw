use super::*;

pub trait KittyDotPGA<B> {
    type Output;

    fn dot_prod(self, other: B) -> Self::Output;
}

impl KittyDotPGA<KittyLinePGA> for KittyLinePGA {
    type Output = f32;

    fn dot_prod(self, other: KittyLinePGA) -> Self::Output {
        self.e_x * other.e_x + self.e_y + other.e_y
    }
}

impl KittyDotPGA<KittyPointPGA> for KittyLinePGA {
    type Output = KittyLinePGA;

    fn dot_prod(self, other: KittyPointPGA) -> Self::Output {
        Self::Output {
            e_0: - self.e_x * other.e_0x - self.e_y * other.e_0y,
            e_x: - self.e_y * other.e_xy,
            e_y: self.e_x * other.e_xy,
        }
    }
}

impl KittyDotPGA<KittyLinePGA> for KittyPointPGA {
    type Output = KittyLinePGA;

    fn dot_prod(self, other: KittyLinePGA) -> Self::Output {
        Self::Output {
            e_0: (self.e_0x * other.e_x) + (self.e_0y * other.e_y),
            e_x: self.e_xy * other.e_y,
            e_y: - self.e_xy * other.e_x,
        }
    }
}

impl KittyDotPGA<KittyPointPGA> for KittyPointPGA {
    type Output = f32;

    fn dot_prod(self, other: KittyPointPGA) -> Self::Output {
        self.e_xy * other.e_xy
    }
}