use glam::Vec3;
use std::ops::*;

pub type Color = Vec3;
pub struct RGB {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
///RGB常量
impl RGB {
    pub const ZERO: RGB = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: RGB = Self {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    pub const X: RGB = Self {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const Y: RGB = Self {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const Z: RGB = Self {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const NEG_X: RGB = Self {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const NEG_Y: RGB = Self {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };
    pub const NEG_Z: RGB = Self {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
}
impl Sub<RGB> for RGB {
    type Output = RGB;
    fn sub(self, rhs: RGB) -> Self::Output {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.y;
        Self { x, y, z }
    }
}
impl SubAssign for RGB {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl Add<RGB> for RGB {
    type Output = RGB;
    fn add(self, rhs: RGB) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.y;
        Self { x, y, z }
    }
}
impl AddAssign for RGB {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Mul<RGB> for RGB {
    type Output = RGB;
    fn mul(self, rhs: RGB) -> Self::Output {
        let x = self.x * rhs.x;
        let y = self.y * rhs.y;
        let z = self.z * rhs.y;
        Self { x, y, z }
    }
}
impl MulAssign for RGB {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl Div<RGB> for RGB {
    type Output = RGB;
    fn div(self, rhs: RGB) -> Self::Output {
        let x = self.x / rhs.x;
        let y = self.y / rhs.y;
        let z = self.z / rhs.y;
        Self { x, y, z }
    }
}
impl DivAssign for RGB {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Mul<f32> for RGB {
    type Output = RGB;
    fn mul(self, rhs: f32) -> Self::Output {
        let x = self.x * rhs;
        let y = self.y * rhs;
        let z = self.z * rhs;
        RGB { x, y, z }
    }
}

impl Div<f32> for RGB {
    type Output = RGB;
    fn div(self, rhs: f32) -> Self::Output {
        let x = self.x / rhs;
        let y = self.y / rhs;
        let z = self.z / rhs;
        RGB { x, y, z }
    }
}
impl Neg for RGB {
    type Output = RGB;
    fn neg(self) -> Self::Output {
        let x = -self.x;
        let y = -self.y;
        let z = -self.z;
        RGB { x, y, z }
    }
}
