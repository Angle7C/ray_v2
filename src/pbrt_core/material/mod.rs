use glam::{Mat3A, Vec3A};

use super::{
    bxdf::{BxDFAble, BxDFType},
    tool::{color::Color, interaction::SurfaceInteraction},
};
pub mod matte;
pub mod mirror;




pub trait MaterialAble {
    fn compute(&self, inter: &mut SurfaceInteraction);
}
pub struct BSDF {
    eta: f32,
    ns: Vec3A,
    ng: Vec3A,
    ss: Vec3A,
    ts: Vec3A,
    mat: Mat3A,
    bxdfs: Vec<Box<dyn BxDFAble>>,
}
impl BSDF {
    pub fn new(inter: &SurfaceInteraction, eta: f32) -> Self {
        let mut bsdf = Self {
            eta,
            ns: inter.shading.n,
            ng: inter.common.n,
            ss: inter.shading.dpdu,
            ts: inter.shading.dpdu.cross(inter.shading.n),
            mat: Mat3A::default(),
            bxdfs: vec![],
        };
        bsdf.mat = Mat3A::from_cols(bsdf.ss, bsdf.ts, bsdf.ns).inverse();
        bsdf
    }
    pub fn world_to_local(&self, w: Vec3A) -> Vec3A {
        let x = w.dot(self.ss);
        let y = w.dot(self.ts);
        let z = w.dot(self.ns);
        Vec3A::new(x, y, z)
    }
    pub fn local_to_world(&self, w: Vec3A) -> Vec3A {
        self.mat.mul_vec3a(w)
    }
    pub fn f(&self, wo: Vec3A, wi: Vec3A, flag: u32) -> Color {
        let wi = self.world_to_local(wi);
        let wo = self.world_to_local(wo);
        let reflect = wi.dot(self.ng) * wo.dot(self.ng) > 0.0;
        let mut f = Color::ZERO;
        for bxdf in &self.bxdfs {
            if bxdf.match_type(flag)
                && (reflect && bxdf.get_type() & u32::from(BxDFType::Reflection) > 0)
                || (!reflect && bxdf.get_type() & u32::from(BxDFType::Transmission) > 0)
            {
                f += bxdf.f(wo, wi);
            }
        }
        f
    }
    pub fn sample_f(&self,wo: Vec3A, wi: &mut Vec3A, pdf: &mut f32,flag: u32) -> Color {
        unimplemented!()
    }

}
