use glam::f64::DVec3;


use super::{func::{fr_conductor, fr_dielectric, fr_schlick_spectrum}, TransportMode};

//菲涅尔反射模型
#[allow(unused)]
pub enum Fresnel {
    //100%反射特殊介质
    NoOP(NoOPFresnel),
    //菲涅尔导体
    Conductor(ConductorFresnel),
    //电介质导体
    Dielectric(DielectricFresnel),
    //迪尼斯导体
    Disney(DisneyFrenel),
}
#[allow(unused)]
impl Fresnel {
    pub fn evaluate(&self, cos_theta_i: f64) -> DVec3 {
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
    r0: DVec3,
    metallic: f64,
    eta: f64,
}
#[allow(unused)]

impl DisneyFrenel {
    pub fn new(r0: DVec3, metallic: f64, eta: f64) -> Self {
        Self { r0, metallic, eta }
    }
    pub fn evaluate(&self, cos_i: f64) -> DVec3 {
        let r = fr_dielectric(cos_i, 1.0, self.eta);
        let a = fr_schlick_spectrum(self.r0, cos_i);
        DVec3::lerp(DVec3::splat(r), a, self.metallic)
    }
}
#[allow(unused)]

//金属
pub struct ConductorFresnel {
    //入射折射率
    pub(crate) eta_i: DVec3,
    //出射折射率
    pub(crate) eta_t: DVec3,
    k: DVec3,
}
#[allow(unused)]

impl ConductorFresnel {
    pub fn new(eta_i: DVec3, eta_t: DVec3, k: DVec3) -> Self {
        Self { eta_i, eta_t, k }
    }
    pub fn evaluate(&self, cos_theta_i: f64) -> DVec3 {
        fr_conductor(cos_theta_i, self.eta_i, self.eta_t, self.k)
    }
}

//电介质导体
#[derive(Clone, Debug)]
#[allow(unused)]
pub struct DielectricFresnel {
    //入射折射率
    pub(crate) eta_i: f64,
    //出射折射率
    pub(crate) eta_t: f64,
}
impl DielectricFresnel {
    pub fn evaluate(&self, cos_theta_i: f64) -> DVec3 {
        DVec3::splat(fr_dielectric(cos_theta_i, self.eta_i, self.eta_t))
    }
    #[allow(unused)]
    pub fn new(eta_i: f64, eta_t: f64) -> Self {
        Self { eta_i, eta_t }
    }
}
//100%反射特殊介质
#[allow(unused)]
pub struct NoOPFresnel;
#[allow(unused)]

impl NoOPFresnel {
    pub fn evaluate(&self, _cos_theta_i: f64) -> DVec3 {
        DVec3::ONE
    }
}

//菲涅尔高光反射
#[allow(unused)]
pub struct FrensnelSpecular {
    r: DVec3,
    t: DVec3,
    eta_a: f64,
    eta_b: f64,
    mode: TransportMode,
    sc_opt: Option<DVec3>,
}
#[allow(unused)]
impl FrensnelSpecular {
    pub fn new(
        r: DVec3,
        t: DVec3,
        eta_a: f64,
        eta_b: f64,
        mode: TransportMode,
        sc_opt: Option<DVec3>,
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

