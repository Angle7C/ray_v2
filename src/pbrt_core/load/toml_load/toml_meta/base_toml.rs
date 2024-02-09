use glam::{ Mat4, Quat, Vec2, Vec3, Vec4};
use serde::{Deserialize, Serialize};
use crate::pbrt_core::camera::{Camera, CameraMode};


#[derive(Deserialize, Debug, Serialize, Default,Copy,Clone)]
pub struct TransformToml {
    //旋转，
    r: Vec4,
    //放缩
    s: Vec3,
    //平移
    t: Vec3,
}

impl TransformToml {

    pub fn get(&self) -> Mat4 {
        //绕轴 x,y,z 旋转 w 角度
        let angle = self.r.w.to_radians();
        //计算四元数
        let quat = Quat::from_axis_angle(self.r.truncate(), angle);
        //构建矩阵
        Mat4::from_scale_rotation_translation(self.s, quat, self.t)
    }

}
#[derive(Deserialize, Debug, Serialize, Default,Copy,Clone)]
pub struct CameraToml {
    mode: CameraMode,
    pub size: Vec2,
    far: f32,
    near: f32,
    eye: Vec3,
    target: Vec3,
    up: Vec3,
    fov: f32,
}
impl CameraToml{
    pub fn get(&self)->Camera{
        match self.mode{
            CameraMode::P => Camera::new(self.eye,self.target,self.up,self.size,self.mode,self.fov),
            CameraMode::O => Camera::new(self.eye,self.target,self.up,self.size,self.mode,self.fov)
        }
    }
}

use crate::pbrt_core::integrator::Integrator;
use crate::pbrt_core::integrator::direct::{DirectIntegrator, LightStrategy};
use crate::pbrt_core::integrator::path::PathIntegrator;
use crate::pbrt_core::sampler::Sampler;
#[derive(Deserialize,Serialize,Default,Debug,Clone)]
pub struct IntegratorToml {
    mode: String,
    core_num:usize,
    sample_num:usize,
    q:f32,
    max_depth:usize
}


impl IntegratorToml{
    pub fn get(&self)->Integrator{
        match self.mode.as_str() {
            "Path"=> Integrator::Path(Box::new(PathIntegrator::new(self.q, self.max_depth)), self.core_num, Sampler::new(self.sample_num)),
            "Direct"=> Integrator::Direct(Box::new(DirectIntegrator::new(self.max_depth,LightStrategy::UniformAll,Sampler::new(self.sample_num))),self.core_num, Sampler::new(self.sample_num)),
            _=>todo!("该渲染器未实现")
        }
    }
}