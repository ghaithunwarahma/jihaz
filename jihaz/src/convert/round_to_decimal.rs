use std::collections::HashMap;
use vello::kurbo::{
    Affine, Arc, BezPath, Circle, Insets, Line, PathEl, Point, 
    Rect, RoundedRect, RoundedRectRadii, Size, TranslateScale, Vec2
};

/// Round the specific f64 values to the specified decimal point
pub trait RoundToDecimal {
    fn round_to_decimal(&mut self, decimal_number: u32);
}

impl RoundToDecimal for f64 {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        assert!(decimal_number > 0);
        let multiplier = 10_f64.powi((decimal_number - 1) as i32);
        *self = (*self * multiplier).round() / multiplier;
    }
}

impl RoundToDecimal for Point {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.x.round_to_decimal(decimal_number);
        self.y.round_to_decimal(decimal_number);
    }
}

impl RoundToDecimal for Vec2 {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.x.round_to_decimal(decimal_number);
        self.y.round_to_decimal(decimal_number);
    }
}

impl RoundToDecimal for Affine {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        let mut coeffs = self.as_coeffs();
        coeffs.round_to_decimal(decimal_number);
        *self = Self::new(coeffs);
    }
}

impl RoundToDecimal for TranslateScale {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.translation.round_to_decimal(decimal_number);
        self.scale.round_to_decimal(decimal_number);
    }
}

impl RoundToDecimal for Size {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.width.round_to_decimal(decimal_number);
        self.height.round_to_decimal(decimal_number);
    }
}

impl RoundToDecimal for Rect {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.x0.round_to_decimal(decimal_number);
        self.y0.round_to_decimal(decimal_number);
        self.x1.round_to_decimal(decimal_number);
        self.y1.round_to_decimal(decimal_number);
    }
}

impl RoundToDecimal for RoundedRectRadii {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.top_right.round_to_decimal(decimal_number);
        self.top_left.round_to_decimal(decimal_number);
        self.bottom_right.round_to_decimal(decimal_number);
        self.bottom_left.round_to_decimal(decimal_number);
    }
}

impl RoundToDecimal for RoundedRect {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        let mut radii = self.radii();
        radii.round_to_decimal(decimal_number);
        *self = Self::from_rect(self.rect(), radii);
    }
}

impl RoundToDecimal for Insets {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.x0.round_to_decimal(decimal_number);
        self.y0.round_to_decimal(decimal_number);
        self.x1.round_to_decimal(decimal_number);
        self.y1.round_to_decimal(decimal_number);
    }
}

impl RoundToDecimal for PathEl {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        match self {
            PathEl::MoveTo(p) => p.round_to_decimal(decimal_number),
            PathEl::LineTo(p) => p.round_to_decimal(decimal_number),
            PathEl::QuadTo(p0, p1) => {
                p0.round_to_decimal(decimal_number);
                p1.round_to_decimal(decimal_number);
            }
            PathEl::CurveTo(p0, p1, p2) => {
                p0.round_to_decimal(decimal_number);
                p1.round_to_decimal(decimal_number);
                p2.round_to_decimal(decimal_number);
            }
            PathEl::ClosePath => ()
        }
    }
}

impl RoundToDecimal for BezPath {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.elements_mut()
            .iter_mut()
            .for_each(|el| el.round_to_decimal(decimal_number));
    }
}

impl RoundToDecimal for Arc {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.center.round_to_decimal(decimal_number);
        self.radii.round_to_decimal(decimal_number);
        self.start_angle.round_to_decimal(decimal_number);
        self.sweep_angle.round_to_decimal(decimal_number);
        self.sweep_angle.round_to_decimal(decimal_number);
        self.x_rotation.round_to_decimal(decimal_number);
    }
}

impl RoundToDecimal for Circle {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.center.round_to_decimal(decimal_number);
        self.radius.round_to_decimal(decimal_number);
    }
}

impl RoundToDecimal for Line {
    fn round_to_decimal(&mut self, decimal_number: u32) {
        self.p0.round_to_decimal(decimal_number);
        self.p1.round_to_decimal(decimal_number);
    }
}

impl<T> RoundToDecimal for [T] 
where
    T: RoundToDecimal
{
    fn round_to_decimal(&mut self, decimal_number: u32) {
        for item in self.iter_mut() {
            item.round_to_decimal(decimal_number);
        }
    }
}

impl<T> RoundToDecimal for Vec<T> 
where
    T: RoundToDecimal
{
    fn round_to_decimal(&mut self, decimal_number: u32) {
        for item in self.iter_mut() {
            item.round_to_decimal(decimal_number);
        }
    }
}

impl<K, T> RoundToDecimal for HashMap<K, T> 
where
    T: RoundToDecimal
{
    fn round_to_decimal(&mut self, decimal_number: u32) {
        for item in self.iter_mut() {
            item.1.round_to_decimal(decimal_number);
        }
    }
}

// macro_rules! round_to_decimal_tuple {
//     ($($size:tt)*) => {
//         impl<T> RoundToDecimal for [T; $n] 
//         where
//             T: RoundToDecimal
//         {
//             fn round_to_decimal(&mut self, decimal_number: u32) {
//                 (
//                     ($size)*
//                 )
//                 for item in self.iter_mut() {
//                     item.round_to_decimal(decimal_number);
//                 }
//             }
//         }
//     }
// }