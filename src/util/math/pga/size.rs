use super::*;

pub trait KittySizePGA {
    #[expect(dead_code)]
    fn size(self) -> f32;
    fn size_sq(self) -> f32;
}

impl KittySizePGA for KittyLinePGA {
    fn size(self) -> f32 {
        (self.e_x.powi(2) + self.e_y.powi(2)).sqrt()
    }

    fn size_sq(self) -> f32 {
        self.e_x.powi(2) + self.e_y.powi(2)
    }
}

impl KittySizePGA for KittyPointPGA {
    fn size(self) -> f32 {
        self.e_xy
    }

    fn size_sq(self) -> f32 {
        self.e_xy.powi(2)
    }
}