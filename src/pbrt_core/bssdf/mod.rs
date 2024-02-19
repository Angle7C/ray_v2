use std::{f32::consts::PI, fmt::Debug};

use glam::{Vec2, Vec3};
use log4rs::encode::Color;


use super::{sampler::{Sampler}, tool::RayDiff};

// pub mod base_phase;
pub mod henyey_phase;
pub mod greenstein_phase;

pub mod homogeneous_medium;
/*
相位函数
用于计算光线从w_in方向反射到w_out方向的概率

*/
const INV_4_PI:f32=1.0/PI;
pub trait PhaseFunction {
    //计算
   fn p(&self,_wo:&Vec3,_wi:Vec3)->f32{
        1.0/(4.0*PI)
   }
   //采样计算
   fn sample_p(&self,wo:&Vec3,wi:&mut Vec3,u:Vec2)->f32;
}
pub fn phase_h_g(cos_theta:f32,g:f32)->f32{
    let denom=1.0+g*g+2.0*g*cos_theta;
    INV_4_PI * (1.0-g*g)/(denom*denom.sqrt())
}

pub trait MediumAble:Debug {
    fn tr(&self,ray:&RayDiff,sampler:Sampler)->Color;
    fn sample_tr(&self,ray:&RayDiff,sampler:Sampler,medium:&mut dyn MediumAble );
    fn get_type(&self)->u32;
}

#[derive(Debug)]
pub struct MediumInterface{
    inside:Box<dyn MediumAble>,

    outside:Box<dyn MediumAble>
}
impl MediumInterface{
    pub fn new(inside:Box<dyn MediumAble>,outside:Box<dyn MediumAble>)->Self{
        Self { inside, outside }
    }
    pub fn is_medium_transition(&self)->bool{
        self.inside.get_type()==self.outside.get_type()
    }
}
