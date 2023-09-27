use std::ops::{Add, AddAssign, Div, Mul, MulAssign, Sub, SubAssign};

use glam::Vec3A;

#[derive(Debug, Clone, Copy)]
pub struct Color(pub f32, pub f32, pub f32);
impl Color {
    pub const ZERO: Color = Color(0.0, 0.0, 0.0);
    pub const ONE: Color = Color(1.0, 1.0, 1.0);
    #[inline]
    pub fn new(x: f32, y: f32, z: f32) -> Color {
        Self(x, y, z)
    }
    pub fn is_black(&self)->bool {
        self.0 < f32::EPSILON && self.1 < f32::EPSILON && self.2 < f32::EPSILON
    }
}
impl AddAssign for Color {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}
impl Add<Color> for Color {
    type Output = Color;
    #[inline]
    fn add(self, rhs: Color) -> Self::Output {
        let x = self.0 + rhs.0;
        let y = self.1 + rhs.1;
        let z = self.2 + rhs.2;
        Self(x, y, z)
    }
}
impl SubAssign for Color {
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}
impl Sub<Color> for Color {
    type Output = Color;
    #[inline]
    fn sub(self, rhs: Color) -> Self::Output {
        let x = self.0 - rhs.0;
        let y = self.1 - rhs.1;
        let z = self.2 - rhs.2;
        Self(x, y, z)
    }
}
impl Mul<Color> for Color {
    type Output = Color;
    #[inline]
    fn mul(self, rhs: Color) -> Self::Output {
        let x = self.0 * rhs.0;
        let y = self.1 * rhs.1;
        let z = self.2 * rhs.2;
        Self(x, y, z)
    }
}
impl Mul<f32> for Color {
    type Output = Color;
    #[inline]
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.0 * rhs;
        let y = self.1 * rhs;
        let z = self.2 * rhs;
        Self(x, y, z)
    }
}
impl MulAssign for Color {
    #[inline]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.1 *= rhs.1;
        self.2 *= rhs.2;
    }
}
impl Div<f32> for Color {
    type Output = Color;
    #[inline]
    fn div(self, rhs: f32) -> Self::Output {
        let x = self.0 / rhs;
        let y = self.1 / rhs;
        let z = self.2 / rhs;
        Self(x, y, z)
    }
}
impl Div<Color> for Color {
    type Output = Color;
    #[inline]
    fn div(self, rhs: Color) -> Self::Output {
        let x = self.0 / rhs.0;
        let y = self.1 / rhs.1;
        let z = self.2 / rhs.2;
        Self(x, y, z)
    }
}

impl Mul<Vec3A> for Color {
    type Output = Color;
    #[inline]
    fn mul(self, rhs: Vec3A) -> Self::Output {
        let x = self.0 * rhs.x;
        let y = self.1 * rhs.y;
        let z = self.2 * rhs.z;
        Self(x, y, z)
    }
}

impl Div<Vec3A> for Color {
    type Output = Color;
    #[inline]
    fn div(self, rhs: Vec3A) -> Self::Output {
        let x = self.0 / rhs.x;
        let y = self.1 / rhs.y;
        let z = self.2 / rhs.z;
        Self(x, y, z)
    }
}

impl From<Vec3A> for Color {
    #[inline]
    fn from(value: Vec3A) -> Self {
        Color(value.x, value.y, value.z)
    }
}

impl From<Color> for Vec3A {
    #[inline]
    fn from(value: Color) -> Self {
        Vec3A::new(value.0, value.1, value.2)
    }
}
