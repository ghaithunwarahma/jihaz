//! Conversions for some vello types
extern crate alloc;
use vello::kurbo::{Point, Vec2, Size, Insets};
use vello::peniko::Color;

pub trait PointExt {
    fn cp_with_x(self, x: f64) -> Point;
    fn cp_add_x(self, x: f64) -> Point;
    fn cp_with_y(self, y: f64) -> Point;
    fn cp_add_y(self, y: f64) -> Point;
    fn cp_add(self, vec: impl Into<Vec2>) -> Point;
    fn scale(&mut self, scale: f64);
    fn with_scale(self, scale: f64) -> Self;
}

impl PointExt for Point {
    fn cp_with_x(mut self, x: f64) -> Point {
        self.x = x;
        self
    }

    fn cp_add_x(mut self, x: f64) -> Point {
        self.x += x;
        self
    }

    fn cp_with_y(mut self, y: f64) -> Point {
        self.y = y;
        self
    }

    fn cp_add_y(mut self, y: f64) -> Point {
        self.y += y;
        self
    }

    fn cp_add(mut self, vec: impl Into<Vec2>) -> Point {
        let vec = vec.into();
        self += vec;
        self
    }
    
    fn scale(&mut self, scale: f64) {
        self.x *= scale;
        self.y *= scale;
    }
    
    fn with_scale(mut self, scale: f64) -> Self {
        self.scale(scale);
        self
    }

    
}

pub trait PolarPoint {
    /// Slide around an origin by a rotation delta of delta_angle
    fn slide(self, circle_origin: Point, delta_angle: f64) -> Point;
    /// Slide around an origin by a rotation delta of delta_angle, for a unit circle, r = 1
    fn slide_unit(self, circle_origin: Point, delta_angle: f64) -> Point;
}

impl PolarPoint for Point {
    fn slide(mut self, circle_origin: Point, mut delta_angle: f64) -> Point {

        let x = circle_origin.x - self.x;
        let y = circle_origin.y - self.y;
        // the angle from arctan ranges from - π/2 and π/2, so it will always lie in either
        // the first or the fourth quadrant, so we need to correct that.
        let mut angle = (y / x).atan();

        // for positive values => 0 <= angle <= π/2 (first quadrant)
        // for negative values => - π/2 <= angle <= 0 (fourth quadrant)
        // so for when x is negative -either second or third quadrant- we need to add π to the result to correct the angle.
        if x.is_sign_negative() {
            angle += std::f64::consts::PI;
        }
        
        // apply the delta to the angle
        if delta_angle >= std::f64::consts::TAU {
            delta_angle = delta_angle % std::f64::consts::TAU;
        }
        angle += delta_angle;
        
        let r = (x.powi(2) + y.powi(2)).sqrt();

        self.x = r * angle.cos() + circle_origin.x;
        self.y = r * angle.sin() + circle_origin.y;
        self
    }

    fn slide_unit(mut self, circle_origin: Point, mut delta_angle: f64) -> Point {

        let x = circle_origin.x - self.x;
        let y = circle_origin.y - self.y;
        // the angle from arctan ranges from - π/2 and π/2, so it will always lie in either
        // the first or the fourth quadrant, so we need to correct that.
        let mut angle = (y / x).atan();

        // for positive values => 0 <= angle <= π/2 (first quadrant)
        // for negative values => - π/2 <= angle <= 0 (fourth quadrant)
        // so for when x is negative -either second or third quadrant- we need to add π to the result to correct the angle.
        if x.is_sign_negative() {
            angle += std::f64::consts::PI;
        }
        
        // apply the delta to the angle
        if delta_angle >= std::f64::consts::TAU {
            delta_angle = delta_angle % std::f64::consts::TAU;
        }
        angle += delta_angle;
        
        self.x = angle.cos() + circle_origin.x;
        self.y = angle.sin() + circle_origin.y;
        self
    }
}

pub trait SizeArray {
    fn max_width(&self) -> f64;
    fn max_height(&self) -> f64;
    fn min_width(&self) -> f64;
    fn min_height(&self) -> f64;
    fn total_width(&self) -> f64;
    fn total_height(&self) -> f64;
}
impl SizeArray for [Size] {
    fn max_width(&self) -> f64 {
        self.iter()
            .map(|s| s.width)
            .reduce(|acc, width| acc.max(width))
            .unwrap()
    }

    fn max_height(&self) -> f64 {
        self.iter()
            .map(|s| s.height)
            .reduce(|acc, height| acc.max(height))
            .unwrap()
    }

    fn min_width(&self) -> f64 {
        self.iter()
            .map(|s| s.width)
            .reduce(|acc, width| acc.min(width))
            .unwrap()
    }

    fn min_height(&self) -> f64 {
        self.iter()
            .map(|s| s.height)
            .reduce(|acc, height| acc.min(height))
            .unwrap()
    }
    
    fn total_width(&self) -> f64 {
        self.iter()
            .map(|s| s.width)
            .reduce(|acc, width| acc + width)
            .unwrap()
    }
    
    fn total_height(&self) -> f64 {
        self.iter()
            .map(|s| s.height)
            .reduce(|acc, height| acc + height)
            .unwrap()
    }
}

/// Helper trait for the total width size calculation
pub trait TotalSizeExt {
    /// Adds to size assuming widgets are stacked horizontally.
    fn horizontal_add(&mut self, size: impl Into<Size>);
    /// Adds to size assuming widgets are stacked vertically.
    fn vertical_add(&mut self, size: impl Into<Size>);
    /// Adds to size assuming widgets are stacked horizontally.
    fn with_horizontal_add(self, size: impl Into<Size>) -> Self;
    /// Adds to size assuming widgets are stacked vertically.
    fn with_vertical_add(self, size: impl Into<Size>) -> Self;
    fn scale(&mut self, scale: f64);
    fn with_scale(self, scale: f64) -> Self;
    /// Substract from size making sure result is non-zero.
    fn sub_size(&mut self, size: impl Into<Size>);
    /// Substract from size width making sure result is non-zero.
    fn sub_width(&mut self, width: f64);
    /// Substract from size height making sure result is non-zero.
    fn sub_height(&mut self, height: f64);
    fn sub_inset(&mut self, inset: Insets);
    /// Substract from size making sure result is non-zero.
    fn with_sub_size(self, size: impl Into<Size>) -> Self;
    /// Substract from size width making sure result is non-zero.
    fn with_sub_width(self, width: f64) -> Self;
    /// Substract from size height making sure result is non-zero.
    fn with_sub_height(self, height: f64) -> Self;
    fn with_sub_inset(self, inset: Insets) -> Self;
    /// Multiply size width by multiple making sure result is non-zero.
    fn mul_width(&mut self, mul: f64);
    /// Multiply size height by multiple making sure result is non-zero.
    fn mul_height(&mut self, mul: f64);
    /// Multiply size width by multiple making sure result is non-zero.
    fn with_mul_width(self, mul: f64) -> Self;
    /// Multiply size height by multiple making sure result is non-zero.
    fn with_mul_height(self, mul: f64) -> Self;
    /// Ensure that self is at least equal to size.
    fn fit(&mut self, size: impl Into<Size>);
    /// Ensure that self is at most equal to size.
    fn shrink_to_fit(&mut self, size: impl Into<Size>);
    fn with_add_size(self, size: impl Into<Size>) -> Self;
    fn with_add_width(self, width: f64) -> Self;
    fn with_add_height(self, height: f64) -> Self;
    /// Add to size making sure result is non-zero.
    fn with_add_inset(self, inset: Insets) -> Self;
}

impl TotalSizeExt for Size {
    fn horizontal_add(&mut self, size: impl Into<Size>) {
        let size = size.into();
        self.width += size.width;
        self.height = self.height.max(size.height);
    }

    fn vertical_add(&mut self, size: impl Into<Size>) {
        let size = size.into();
        self.width = self.width.max(size.width);
        self.height += size.height;
    }

    fn with_horizontal_add(mut self, size: impl Into<Size>) -> Self {
        self.horizontal_add(size);
        self
    }

    fn with_vertical_add(mut self, size: impl Into<Size>) -> Self {
        self.vertical_add(size);
        self
    }

    fn scale(&mut self, scale: f64) {
        self.width *= scale;
        self.height *= scale;
    }

    fn with_scale(mut self, scale: f64) -> Self {
        self.scale(scale);
        self
    }

    fn sub_size(&mut self, size: impl Into<Size>) {
        let size = size.into();
        self.sub_width(size.width);
        self.sub_height(size.height);
    }

    fn sub_width(&mut self, width: f64) {
        self.width = (self.width - width).max(0f64);
    }

    fn sub_height(&mut self, height: f64) {
        self.height = (self.height - height).max(0f64);
    }

    fn sub_inset(&mut self, inset: Insets) {
        self.width = (self.width - inset.x_value()).max(0f64);
        self.height = (self.height - inset.y_value()).max(0f64);
    }

    fn with_sub_size(mut self, size: impl Into<Size>) -> Self {
        self.sub_size(size);
        self
    }

    fn with_sub_width(mut self, width: f64) -> Self {
        self.sub_width(width);
        self
    }

    fn with_sub_height(mut self, height: f64) -> Self {
        self.sub_height(height);
        self
    }

    fn with_sub_inset(mut self, inset: Insets) -> Self {
        self.sub_inset(inset);
        self
    }

    fn with_add_width(mut self, width: f64) -> Self {
        self.width += width;
        self
    }

    fn with_add_height(mut self, height: f64) -> Self {
        self.height += height;
        self
    }

    fn with_add_size(mut self, size: impl Into<Size>) -> Self {
        let size = size.into();
        self.width += size.width;
        self.height += size.height;
        self
    }

    fn with_add_inset(mut self, inset: Insets) -> Self {
        self.width += inset.x_value();
        self.height += inset.y_value();
        self
    }

    fn mul_width(&mut self, mul: f64) {
        self.width *= mul;
    }

    fn mul_height(&mut self, mul: f64) {
        self.height *= mul;
    }

    fn with_mul_width(mut self, mul: f64) -> Self {
        self.mul_width(mul);
        self
    }

    fn with_mul_height(mut self, mul: f64) -> Self {
        self.mul_height(mul);
        self
    }

    fn fit(&mut self, size: impl Into<Size>) {
        let size = size.into();
        if self.width < size.width {
            self.width = size.width;
        }
        if self.height < size.height {
            self.height = size.height;
        }
    }

    fn shrink_to_fit(&mut self, size: impl Into<Size>) {
        let size = size.into();
        if self.width < size.width {
            self.width = size.width;
        }
        if self.height < size.height {
            self.height = size.height;
        }
    }
}

// pub trait WidgetExt<T: Data>: Widget<T> + Sized + 'static {
//     fn explicit(self) -> W2<T, Self> {
//         W2::explicit(self)
//     }

//     fn implicit(self) -> W2<T, Self> {
//         W2::implicit(self)
//     }

//     fn scroll(self) -> Scroll<T, Self> {
//         Scroll::new(self)
//     }

//     fn vertical_scroll(self) -> Scroll<T, Self> {
//         Scroll::new(self).vertical()
//     }
// }

// /// It seams that children changed does not propogate to children when a child widgetpod is old
// /// so we use this trait to recreate widgets such that children changed is issued for self and all children,
// /// and methods such as LifeCycle::WidgetAdded gets called.
// pub trait Recreate {
//     fn recreate(&mut self);
// }


// use druid::widget::prelude::{PaintCtx, Env};

// pub trait OpaqePaint<T> {
//     fn opaque_paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env, opacity: Option<f64>);
// }

pub trait WithOpacity {
    fn opaque(self, opacity: impl Into<Option<f64>>) -> Self;
}

impl WithOpacity for Color {
    fn opaque(self, opacity: impl Into<Option<f64>>) -> Self {
        if let Some(opacity) = opacity.into() {
            return self.with_alpha_factor(opacity as f32);
        }
        self
    }
}