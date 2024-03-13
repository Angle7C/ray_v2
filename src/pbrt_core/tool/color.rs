
use std::{fmt::Display, ops::*};

use glam::Vec3;

use serde::{Deserialize, Serialize};

pub type Color = RGB;
#[derive(Debug,Default,Clone, Copy,Deserialize,Serialize)]
pub struct RGB {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Display for RGB{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"[{},{},{}]",self.x,self.y,self.z)
    }
}
impl RGB{
    pub fn powf(&self,p:f32)->RGB{
        let x = self.x.powf(p);
        let y = self.y.powf(p);
        let z = self.z.powf(p);
        RGB { x, y, z }
    }
    pub fn abs_diff_eq(&self,value:f32,diff:f32)->bool{
        (self.x-value).abs()<diff||(self.y-value).abs()<diff||(self.z-value).abs()<diff
    }
    pub fn clamp(&self,min:Vec3,max:Vec3)->RGB{
        let x=min.x.max(self.x).min(max.x);
        let y=min.y.max(self.y).min(max.y);
        let z=min.z.max(self.z).min(max.z);
        RGB { x, y, z }
    }
    pub fn new(x:f32,y:f32,z:f32)->RGB{
        Self{x,y,z}
    }
    pub fn splat(value:f32)->RGB{
        Self { x:value, y: value, z: value }
    }
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
        z: 1.0,
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

impl Sub<f32> for RGB{
    type Output = RGB;
    fn sub(self, rhs: f32) -> Self::Output {
        RGB{x:self.x-rhs,
        y:self.y-rhs,
        z:self.z-rhs}
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
impl Add<Vec3> for RGB {
    type Output = RGB;
    fn add(self, rhs: Vec3) -> Self::Output {
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

impl Div<Vec3> for RGB{
    type Output = RGB;
    fn div(self, rhs: Vec3) -> Self::Output {
       Self{ x:self.x/rhs.x,
             y:self.y/rhs.y,
             z:self.z/rhs.z}

    }
}

impl Mul<Vec3> for RGB{
    type Output = RGB;
    fn mul(self, rhs: Vec3) -> Self::Output {
       Self{ x:self.x*rhs.x,
             y:self.y*rhs.y,
             z:self.z*rhs.z}

    }
}

impl From<Vec3> for RGB{
    fn from(value: Vec3) -> Self {
        Self { x:value.x, y: value.y, z: value.z }
    }
}
impl From<RGB> for Vec3{
    fn from(value: RGB) -> Self {
        Vec3::new(value.x, value.y, value.z)
    }
}
