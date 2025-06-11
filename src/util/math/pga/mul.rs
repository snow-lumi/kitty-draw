use std::ops::Mul;

use super::*;

impl Mul<f32> for KittyLinePGA {
    type Output = KittyLinePGA;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::Output {
            e_x: self.e_x * rhs,
            e_y: self.e_y * rhs,
            e_0: self.e_0 * rhs,
        }
    }
}