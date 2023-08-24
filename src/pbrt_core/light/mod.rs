use std::ops::{BitAnd, BitOrAssign, BitOr};

use glam::{DVec2, DVec3};

use self::point::PointLight;

use super::tool::{SurfaceInteraction, Visibility};

pub mod point;
pub mod area;
pub enum Light{
    PointLight(Box<PointLight>)
}
pub enum LightType{
    DeltaPosition = 1,
    DeltaDirection = 2,
    Area=4,
    Infinite=8
}
impl LightType{
    fn is_delta(flag:u32)->bool{
        (flag & LightType::DeltaPosition>0) || (flag& LightType::DeltaDirection>0) 
    }
       
}
impl BitAnd<u32> for LightType{
    type Output = u32;
    fn bitand(self, rhs: u32) -> Self::Output {
        rhs & self as u32
    }
}
impl BitOr<u32> for LightType{
    type Output=u32;
    fn bitor(self, rhs: u32) -> Self::Output {
        rhs | self as u32
    }
}
impl BitAnd<LightType> for u32{
    type Output = u32;
    fn bitand(self, rhs: LightType) -> Self::Output {
        rhs&self
    }
}
impl BitOr<LightType> for u32{
    type Output=u32;
    fn bitor(self, rhs: LightType) -> Self::Output {
        rhs|self
    }
}
pub trait LightAble {
    fn sample_f(&self,surface:&SurfaceInteraction,u:DVec2,w_in:&mut DVec3,pdf:& mut f64,vis:&mut Visibility)->DVec3;
    fn power(&self)->DVec3;
}