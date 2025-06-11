use super::*;
use super::dual::KittyDualPGA;
use super::wedge_prod::KittyWedgePGA;


pub trait KittyRegressivePGA<B> {
    type Output;

    fn regressive_prod(self, other: B) -> Self::Output;
}

impl KittyRegressivePGA<KittyPointPGA> for KittyPointPGA {
    type Output = KittyLinePGA;

    fn regressive_prod(self, other: KittyPointPGA) -> Self::Output {
        self.dual().wedge_prod(other.dual()).dual()
    }
}