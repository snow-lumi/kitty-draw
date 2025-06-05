use super::*;

pub trait KittyDotPGA<B> {
    type Output;

    fn dot_prod(self, other: B) -> Self::Output;
}

impl KittyDotPGA<KittyLinePGA> for KittyPointPGA {
    type Output = KittyLinePGA;

    fn dot_prod(self, other: KittyLinePGA) -> Self::Output {
        Self::Output {
            e_0: (self.e_0x * other.e_x) + (self.e_0y * other.e_y),
            e_x: (self.e_xy * other.e_y) - (self.e_0x * other.e_0),
            e_y: - (self.e_xy * other.e_x) - (self.e_0y * other.e_0),
        }
    }
}

impl KittyDotPGA<KittyPointPGA> for KittyLinePGA {
    type Output = KittyLinePGA;

    fn dot_prod(self, other: KittyPointPGA) -> Self::Output {
        Self::Output {
            e_0: - (self.e_x * other.e_0x) - (self.e_y * other.e_0y),
            e_x: (self.e_0 * other.e_0x) - (self.e_y * other.e_xy),
            e_y: (self.e_0 * other.e_0y) + (self.e_x * other.e_xy),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const ZERO_VEC: KittyLinePGA = KittyLinePGA { e_0: 0.0, e_x: 0.0, e_y: 0.0 };
    const TEST_VEC: KittyLinePGA = KittyLinePGA { e_0: 2.0, e_x: 3.0, e_y: 4.0 };
    const UNIT_E_0: KittyLinePGA = KittyLinePGA { e_0: 1.0, e_x: 0.0, e_y: 0.0 };
    const UNIT_E_X: KittyLinePGA = KittyLinePGA { e_0: 0.0, e_x: 1.0, e_y: 0.0 };
    const UNIT_E_Y: KittyLinePGA = KittyLinePGA { e_0: 0.0, e_x: 0.0, e_y: 1.0 };
    const _ZERO_BIVEC: KittyPointPGA = KittyPointPGA { e_xy: 0.0, e_0y: 0.0, e_0x: 0.0 };
    const TEST_BIVEC: KittyPointPGA = KittyPointPGA { e_xy: 5.0, e_0y: 6.0, e_0x: 7.0 };
    const UNIT_E_XY: KittyPointPGA = KittyPointPGA { e_xy: 1.0, e_0y: 0.0, e_0x: 0.0 };
    const UNIT_E_0Y: KittyPointPGA = KittyPointPGA { e_xy: 0.0, e_0y: 1.0, e_0x: 0.0 };
    const UNIT_E_0X: KittyPointPGA = KittyPointPGA { e_xy: 0.0, e_0y: 0.0, e_0x: 1.0 };

    const RES_TEST_1X2: KittyLinePGA = KittyLinePGA { e_0: -45.0, e_x: -6.0, e_y: 27.0};
    const RES_TEST_2X1: KittyLinePGA = KittyLinePGA { e_0: 45.0, e_x: 6.0, e_y: -27.0};

    #[test]
    fn test_dot_product() {

        // vec dot bivec
        assert_eq!(UNIT_E_0.dot_prod(UNIT_E_XY), ZERO_VEC);
        assert_eq!(UNIT_E_0.dot_prod(UNIT_E_0X), UNIT_E_X);
        assert_eq!(UNIT_E_0.dot_prod(UNIT_E_0Y), UNIT_E_Y);
        assert_eq!(UNIT_E_X.dot_prod(UNIT_E_XY), UNIT_E_Y);
        assert_eq!(UNIT_E_X.dot_prod(UNIT_E_0X), (UNIT_E_0 * -1.0));
        assert_eq!(UNIT_E_X.dot_prod(UNIT_E_0Y), ZERO_VEC);
        assert_eq!(UNIT_E_Y.dot_prod(UNIT_E_XY), (UNIT_E_X * -1.0));
        assert_eq!(UNIT_E_Y.dot_prod(UNIT_E_0X), ZERO_VEC);
        assert_eq!(UNIT_E_Y.dot_prod(UNIT_E_0Y), (UNIT_E_0 * -1.0));
        assert_eq!(TEST_VEC.dot_prod(TEST_BIVEC), RES_TEST_1X2);


        // bivec dot vec
        assert_eq!(UNIT_E_XY.dot_prod(UNIT_E_0), ZERO_VEC);
        assert_eq!(UNIT_E_0X.dot_prod(UNIT_E_0), (UNIT_E_X * -1.0));
        assert_eq!(UNIT_E_0Y.dot_prod(UNIT_E_0), (UNIT_E_Y * -1.0));
        assert_eq!(UNIT_E_XY.dot_prod(UNIT_E_X), (UNIT_E_Y * -1.0));
        assert_eq!(UNIT_E_0X.dot_prod(UNIT_E_X), UNIT_E_0);
        assert_eq!(UNIT_E_0Y.dot_prod(UNIT_E_X), ZERO_VEC);
        assert_eq!(UNIT_E_XY.dot_prod(UNIT_E_Y), UNIT_E_X);
        assert_eq!(UNIT_E_0X.dot_prod(UNIT_E_Y), ZERO_VEC);
        assert_eq!(UNIT_E_0Y.dot_prod(UNIT_E_Y), UNIT_E_0);
        assert_eq!(TEST_BIVEC.dot_prod(TEST_VEC), RES_TEST_2X1);
    }
}