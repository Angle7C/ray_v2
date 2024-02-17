use glam::f32::Vec3;

use crate::pbrt_core::tool::color::Color;

use super::{
    func::{fr_conductor, fr_dielectric, fr_schlick_spectrum},
    TransportMode,
};

//菲涅尔反射模型
#[allow(unused)]
pub enum Fresnel {
    //100%反射特殊介质
    NoOP(NoOPFresnel),
    //菲涅尔导体
    Conductor(ConductorFresnel),
    //电介质，导体，半导体
    Dielectric(DielectricFresnel),
    //迪尼斯导体
    Disney(DisneyFrenel),
}
#[allow(unused)]
impl Fresnel {
    pub fn evaluate(&self, cos_theta_i: f32) -> Vec3 {
        match self {
            Fresnel::NoOP(noop) => noop.evaluate(cos_theta_i),
            Fresnel::Dielectric(die) => die.evaluate(cos_theta_i),
            Fresnel::Conductor(con) => con.evaluate(cos_theta_i),
            Fresnel::Disney(dis) => dis.evaluate(cos_theta_i),
        }
    }
}
//迪尼斯
#[allow(unused)]
pub struct DisneyFrenel {
    r0: Color,
    metallic: f32,
    eta: f32,
}
#[allow(unused)]

impl DisneyFrenel {
    pub fn new(r0: Color, metallic: f32, eta: f32) -> Self {
        Self { r0, metallic, eta }
    }
    pub fn evaluate(&self, cos_i: f32) -> Vec3 {
        let r = fr_dielectric(cos_i, 1.0, self.eta);
        let a = fr_schlick_spectrum(self.r0.into(), cos_i);
        Vec3::lerp(Vec3::splat(r), a, self.metallic)
    }
}
#[allow(unused)]

//金属
pub struct ConductorFresnel {
    //入射折射率
    pub(crate) eta_i: Vec3,
    //出射折射率
    pub(crate) eta_t: Vec3,
    // 吸收系数
    k: Vec3,
}
#[allow(unused)]

impl ConductorFresnel {
    pub fn new(eta_i: Vec3, eta_t: Vec3, k: Vec3) -> Self {
        Self { eta_i, eta_t, k }
    }
    pub fn evaluate(&self, cos_theta_i: f32) -> Vec3 {
        fr_conductor(cos_theta_i.abs(), self.eta_i, self.eta_t, self.k)
    }
}

//电介质，导体，半导体
#[derive(Clone, Debug)]
#[allow(unused)]
pub struct DielectricFresnel {
    //入射折射率
    pub(crate) eta_i: f32,
    //出射折射率
    pub(crate) eta_t: f32,
}
impl DielectricFresnel {
    /**
     *  cos_theta_i： 入射余弦,
     *  eta: 相对折射率
     *
     */
    pub fn evaluate(&self, cos_theta_i: f32) -> Vec3 {
        Vec3::splat(fr_dielectric(cos_theta_i, self.eta_i, self.eta_t))
    }
    #[allow(unused)]
    pub fn new(eta_i: f32, eta_t: f32) -> Self {
        Self { eta_i, eta_t }
    }
}
//100%反射特殊介质
#[allow(unused)]
pub struct NoOPFresnel;
#[allow(unused)]

impl NoOPFresnel {
    pub fn evaluate(&self, _cos_theta_i: f32) -> Vec3 {
        Vec3::ONE
    }
}

//菲涅尔高光反射
#[allow(unused)]
pub struct FrensnelSpecular {
    r: Vec3,
    t: Vec3,
    eta_a: f32,
    eta_b: f32,
    mode: TransportMode,
    sc_opt: Option<Vec3>,
}
#[allow(unused)]
impl FrensnelSpecular {
    pub fn new(
        r: Vec3,
        t: Vec3,
        eta_a: f32,
        eta_b: f32,
        mode: TransportMode,
        sc_opt: Option<Vec3>,
    ) -> Self {
        Self {
            r,
            t,
            eta_a,
            eta_b,
            mode,
            sc_opt,
        }
    }
}
