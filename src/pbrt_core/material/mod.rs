use std::fmt::Debug;

use crate::pbrt_core::bxdf::BxDFType;
use glam::{Vec2, Vec3};

use super::{
    bxdf::{BxDF, TransportMode},
    tool::SurfaceInteraction,
};

pub mod disney;
pub mod matte;
pub mod metal;
pub mod mirror;
pub mod pbr;
pub mod plastic;

pub trait Material: Debug {
    fn compute_scattering_functions(&self, suface: &mut SurfaceInteraction, mode: TransportMode);
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
        sampled_type: &mut u32,
    ) -> Vec3 {
        let bxdfs = self
            .bxdfs
            .iter()
            .filter(|item| item.match_type(flag))
            .collect::<Vec<_>>();
        if bxdfs.is_empty() {
            return Vec3::ZERO;
        }
        let num = (u.x * bxdfs.len() as f32).clamp(0.0, bxdfs.len() as f32) as usize;

        let bxdf = bxdfs[num];
        let mut wi = Vec3::ZERO;
        let w_out = self.world_to_local(*w_out);
        if w_out.z == 0.0 {
            return Vec3::ZERO;
        }
        *pdf = 0.0;
        *sampled_type = bxdf.get_type();
        let mut f = bxdf.sample_f(&w_out, &mut wi, u, pdf);
        *w_in = self.local_to_world(wi);

        if pdf.abs() < f32::EPSILON {
            *sampled_type = 0;
            return Vec3::ZERO;
        }

        if bxdf.get_type() & BxDFType::Specular as u32 == 0 {
            for (index, item) in bxdfs.iter().enumerate() {
                if index != num && item.match_type(flag) {
                    *pdf += item.pdf(&w_out, w_in)
                }
            }
        };
        *pdf /= bxdfs.len() as f32;
        if bxdf.get_type() & BxDFType::Specular as u32 == 0 {
            f = Vec3::ZERO;
            let _reflect = w_in.dot(self.ng) * w_out.dot(self.ng) > 0.0;
            for item in bxdfs.iter() {
                if item.match_type(flag)
                //    && ((reflect && item.match_type(BxDFType::Reflection.into()))
                //        || (!reflect && item.match_type(BxDFType::Transmission.into())))
                {
                    f += item.f(&w_out, w_in)
                }
            }
        }
        f
    }
    pub fn new(si: &SurfaceInteraction, eta: f32) -> Self {
        let ss = si.shading.dpdu.normalize();
        Self {
            eta,
            ns: si.shading.n,
            ng: si.common.normal,
            ss,
            ts: si.shading.n.cross(ss).normalize(),
            bxdfs: vec![],
        }
    }
    pub fn pdf(&self, w_out: &Vec3, w_in: &Vec3, flag: u32) -> f32 {
        if self.bxdfs.is_empty() {
            0.0
        } else {
            let wo = self.world_to_local(*w_out);
            let wi = self.world_to_local(*w_in);
            if wo.z.abs() < f32::EPSILON {
                return 0.0;
            }
            let mut pdf = 0.0;
            let mut num = 0;
            for item in &self.bxdfs {
                if item.match_type(flag) {
                    num += 1;
                    pdf += item.pdf(&wo, &wi);
                }
            }
            if num > 0 {
                pdf / num as f32
            } else {
                0.0
            }
        }
    }
    pub fn f(&self, w_out: &Vec3, w_in: &Vec3, flag: u32) -> Vec3 {
        let wi: Vec3 = self.world_to_local(*w_in);
        let wo = self.world_to_local(*w_out);
        let _reflect = w_in.dot(self.ng) * w_out.dot(self.ng) > 0.0;
        let mut f = Vec3::ZERO;
        for bxdf in &self.bxdfs {
            if bxdf.match_type(flag)
          //      && ((reflect && bxdf.match_type(BxDFType::Reflection as u32))
          //          || (!reflect && bxdf.match_type(BxDFType::Transmission as u32)))
            {
                f += bxdf.f(&wo, &wi);
            }
        }
        f
    }
    pub fn num_components(&self, flag: u32) -> u32 {
        let mut num = 0;
        for item in &self.bxdfs {
            if item.match_type(flag) {
                num += 1;
            }
        }
        num
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
