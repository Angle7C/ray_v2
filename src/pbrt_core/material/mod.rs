use std::fmt::Debug;

use glam::{f64::DVec3, DVec2};

use super::bxdf::BxDF;
mod matte;

pub trait Material:Debug {
    fn compute_scattering_functions(&self);
}
pub struct BSDF{
    pub eta:f64,
    pub ns: DVec3,
    pub ng: DVec3,
    pub ss: DVec3,
    pub ts: DVec3,
    bxdfs:Vec<BxDF>
}
impl BSDF{
    pub fn sample_f(&self,w_out:&DVec3,w_in:&mut DVec3,u:DVec2,pdf:&mut f64,bsdf_flags:u32)->DVec3{
        todo!()
    }
}