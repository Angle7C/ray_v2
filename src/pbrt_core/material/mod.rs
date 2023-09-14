use std::fmt::Debug;

use glam::{Vec3, Vec2};
use rand:: Rng;

use super::{
    bxdf::{BxDF,TransportMode},
    texture::Texture,
    tool::SurfaceInteraction,
};
pub mod disney;
pub mod matte;
pub mod mirror;
pub mod pbr;
pub trait Material: Debug {
    fn compute_scattering_functions(&self, suface: &mut SurfaceInteraction, mode: TransportMode);
    fn bump(&self, suface: &SurfaceInteraction, texture: &dyn Texture);
}
// BSDF使用局部坐标系。
pub struct BSDF {
    //折射率，默认值 1.0
    pub eta: f32,
    //阴影法线
    pub ns: Vec3,
    //几何法线
    pub ng: Vec3,
    // 副切线
    pub ss: Vec3,
    //切线
    pub ts: Vec3,
    bxdfs: Vec<BxDF>,
}
impl BSDF {
    pub fn sample_f(
        &self,
        w_out: &Vec3,
        w_in: &mut Vec3,
        u: Vec2,
        pdf: &mut f32,
        flag: u32,
    ) -> Vec3 {
        *pdf = 0.0;
        let w_out = self.world_to_local(*w_out);

        if flag == 0 {
            return Vec3::ZERO;
        }
        if w_out.z == 0.0 {
            return Vec3::ZERO;
        }
        let num = rand::thread_rng().gen_range(0..self.bxdfs.len());
        let bxdf = &self.bxdfs[num];
        let bsdf = bxdf.sample_f(&w_out, w_in, u, pdf);
        *w_in = self.local_to_world(*w_in);
        return bsdf;
    }
    pub fn new(si: &SurfaceInteraction, eta: f32) -> Self {
        let ss = si.shading.dpdu.normalize();
        Self {
            eta,
            ns: si.shading.n,
            ng: si.common.normal,
            ss,
            ts: si.shading.n.cross(ss),
            bxdfs: vec![],
        }
    }
    pub fn pdf(&self, _w_out: &Vec3, _w_in: &mut Vec3, _flag: u32) -> f32 {
        1.0
    }
    pub fn f(&self, w_out: &Vec3, w_in: &Vec3, flag: u32) -> Vec3 {
        let w_in = &mut self.world_to_local(*w_in);
        let w_out = &mut self.world_to_local(*w_out);
        let _reflect = w_in.dot(self.ng) * w_out.dot(self.ng) > 0.0;
        let mut f = Vec3::ZERO;
        for  bxdf in &self.bxdfs {
            if bxdf.match_type(flag)
            //&& (reflect&&self.bxdfs[i].match_type(BxDFType::Reflection as u32))
            // && (!reflect&&self.bxdfs[i].match_type(BxDFType::Transmission as u32))
            {
                f += bxdf.f(&w_out, w_in);
            }
        }
        f
    }
    pub fn world_to_local(&self, v: Vec3) -> Vec3 {
        let x = self.ss.dot(v);
        let y = self.ts.dot(v);
        let z = self.ns.dot(v);
        Vec3 { x, y, z }
    }
    pub fn local_to_world(&self, v: Vec3) -> Vec3 {
        let x = self.ss.x * v.x + self.ts.x * v.y + self.ns.x * v.z;
        let y = self.ss.y * v.x + self.ts.y * v.y + self.ns.y * v.z;
        let z = self.ss.z * v.x + self.ts.z * v.y + self.ns.z * v.z;
        Vec3 { x, y, z }
    }
}
