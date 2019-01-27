use crate::Vector2;

use std::ops::*;

impl Vector2 {
    /// Dot product of two vectors.
    #[inline]
    pub fn dot(&self, other: Vector2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Cross product of two vectors.
    ///
    /// This is signed so that (0, 1) × (1, 0) = 1.
    #[inline]
    pub fn cross(&self, other: Vector2) -> f32 {
        self.x * other.y - self.y * other.x
    }

    /// Magnitude of vector.
    #[inline]
    pub fn hypot(&self) -> f32 {
        self.x.hypot(self.y)
    }

    /// Magnitude squared of vector.
    #[inline]
    pub fn hypot2(&self) -> f32 {
        self.dot(*self)
    }

    /// Angle of vector.
    ///
    /// If the vector is interpreted as a complex number, this is the argument.
    #[inline]
    pub fn atan2(&self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn normalize(&self) -> Vector2 {
        let len = self.hypot();
        Vector2::new(self.x / len, self.y / len)
    }

    /// A unit vector of the given angle.
    ///
    /// With `th` at zero, the result is the positive X unit vector, and
    /// at π/2, it is the positive Y unit vector.
    ///
    /// Thus, in a Y-down coordinate system (as is common for graphics),
    /// it is a clockwise rotation, and in Y-up (traditional for math), it
    /// is anti-clockwise. This convention is consistent with
    /// [`Affine::rotate`](struct.Affine.html#method.rotate).
    #[inline]
    pub fn from_angle(th: f32) -> Vector2 {
        Vector2::new(th.cos(), th.sin())
    }

    /// Linearly interpolate between two points.
    #[inline]
    pub fn lerp(&self, other: Vector2, t: f32) -> Vector2 {
        *self + t * (other - *self)
    }
}

impl From<(f32, f32)> for Vector2 {
    #[inline]
    fn from(v: (f32, f32)) -> Vector2 {
        Vector2::new(v.0, v.1)
    }
}

impl From<Vector2> for (f32, f32) {
    #[inline]
    fn from(v: Vector2) -> (f32, f32) {
        (v.x, v.y)
    }
}

impl Add for Vector2 {
    type Output = Vector2;

    #[inline]
    fn add(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl AddAssign for Vector2 {
    #[inline]
    fn add_assign(&mut self, other: Vector2) {
        *self = Vector2::new(self.x + other.x, self.y + other.y)
    }
}

impl Sub for Vector2 {
    type Output = Vector2;

    #[inline]
    fn sub(self, other: Vector2) -> Vector2 {
        Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign for Vector2 {
    #[inline]
    fn sub_assign(&mut self, other: Vector2) {
        *self = Vector2::new(self.x - other.x, self.y - other.y)
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;

    #[inline]
    fn mul(self, other: f32) -> Vector2 {
        Vector2::new(self.x * other, self.y * other)
    }
}

impl MulAssign<f32> for Vector2 {
    #[inline]
    fn mul_assign(&mut self, other: f32) {
        *self = Vector2::new(self.x * other, self.y * other);
    }
}

impl Mul<Vector2> for f32 {
    type Output = Vector2;

    #[inline]
    fn mul(self, other: Vector2) -> Vector2 {
        other * self
    }
}

impl Div<f32> for Vector2 {
    type Output = Vector2;

    /// Note: division by a scalar is implemented by multiplying by the reciprocal.
    ///
    /// This is more efficient but has different roundoff behavior than division.
    #[inline]
    fn div(self, other: f32) -> Vector2 {
        self * other.recip()
    }
}

impl DivAssign<f32> for Vector2 {
    #[inline]
    fn div_assign(&mut self, other: f32) {
        *self *= other.recip();
    }
}

impl Neg for Vector2 {
    type Output = Vector2;

    #[inline]
    fn neg(self) -> Vector2 {
        Vector2::new(-self.x, -self.y)
    }
}

// Conversions to and from mint
#[cfg(feature = "mint")]
impl From<Vector2> for mint::Vector2<f32> {
    #[inline]
    fn from(p: Vector2) -> mint::Vector2<f32> {
        mint::Vector2 { x: p.x, y: p.y }
    }
}

#[cfg(feature = "mint")]
impl From<mint::Vector2<f32>> for Vector2 {
    #[inline]
    fn from(p: mint::Vector2<f32>) -> Vector2 {
        Vector2 { x: p.x, y: p.y }
    }
}
