use std::fmt::Debug;

use glam::{Vec3, Vec2};
use image::codecs::png::CompressionType::Default;
use image::imageops::dither;
use rand::Rng;
use crate::pbrt_core::bxdf::BxDFType;

use super::{
    bxdf::{BxDF, TransportMode},
    texture::Texture,
    tool::SurfaceInteraction,
};

pub mod disney;
pub mod matte;
pub mod mirror;
pub mod pbr;
pub mod metal;
pub mod plastic;

pub trait Material: Debug {
    fn compute_scattering_functions(&self, suface: &mut SurfaceInteraction, mode: TransportMode);
    fn bump(&self, suface: &SurfaceInteraction, texture: &dyn Texture) {}
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
        let num = self.num_components(flag);
        if flag == 0 || num == 0 {
            *pdf = 0.0;
            *sampled_type = 0;
            return Vec3::ZERO;
        }
        //随机采样到一个符合条件BxDF
        let comp = u32::min((num as f32 * u.x).floor() as u32, num - 1);
        let mut bxdf: Option<&BxDF> = None;
        let mut count = comp;
        let mut i = 0;
        for (index, item) in self.bxdfs.iter().enumerate() {
            let matches = item.match_type(flag);
            if matches && count == 0 {
                count -= 1;
                bxdf = Some(item);
                i = index;
            } else if matches {
                count - 1;
            }
        }
        if let Some(bxdf) = bxdf {
            //采样
            let x = (u.x * num as f32 - comp as f32).min(1.0);
            let y = u.y;
            let u_re = Vec2::new(x, y);
            let mut wi = Vec3::ZERO;
            let w_out = self.world_to_local(*w_out);
            if w_out.z == 0.0 {
                return Vec3::ZERO;
            }
            *pdf = 0.0;
            if *sampled_type != 0 {
                *sampled_type = bxdf.get_type();
            }
            let mut f = bxdf.sample_f(&w_out, &mut wi, u_re, pdf);
            if pdf.abs() < f32::EPSILON {
                if sampled_type != 0 {
                    *sampled_type = 0;
                }
                Vec3::ZERO
            }
            *w_in = self.local_to_world(*wi);

            if bxdf.get_type() & BxDFType::Specular as u32 == 0 && num > 1 {
                let reflect = w_in.dot(self.ng) * w_out.dot(self.ng) > 0.0;
                f = Vec3::ZERO;
                for (index, item) in self.bxdfs.iter().enumerate() {
                    if i != index && item.match_type(flag) {
                        *pdf += item.pdf(&w_out,&w_in)
                    }
                }
            };
            if num > 1 {
                *pdf /= num;
            }
            if bxdf.get_type() & BxDFType::Specular as u8 == 0 {
                let reflect = w_in.dot(self.ng) * w_out.dot(self.ng) > 0.0;
                for item in self.bxdfs.iter() {
                    if item.match_type(flag)
                        && (reflect && item.match_type(BxDFType::Reflection.into()))
                        || (!reflect && item.match_type(BxDFType::Transmission.into())) {
                        f += item.f(&w_out, w_in)
                    }
                }
            }
            f
        } else {
            Vec3::ZERO
        }
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
    pub fn pdf(&self, w_out: &Vec3, w_in: &mut Vec3, flag: u32) -> f32 {
        if self.bxdfs.len() == 0 {
            return 0.0;
        } else {
            let wo = self.world_to_local(*w_out);
            let wi = self.world_to_local(*w_in);
            if wo.z == 0.0 {
                return 0.0;
            }
            let mut pdf=0.0;
            let mut num=0;
            for item in self.bxdfs{
                if item.match_type(flag){
                    num+=1;
                    pdf+=item.pdf(&wo,&wi);
                }
            }
            return if num>0{
                pdf/num
            }else{
                0.0
            }
        };
    }
    pub fn f(&self, w_out: &Vec3, w_in: &Vec3, flag: u32) -> Vec3 {
        let w_in = &mut self.world_to_local(*w_in);
        let w_out = &mut self.world_to_local(*w_out);
        let _reflect = w_in.dot(self.ng) * w_out.dot(self.ng) > 0.0;
        let mut f = Vec3::ZERO;
        for bxdf in &self.bxdfs {
            if bxdf.match_type(flag)
            //&& (reflect&&self.bxdfs[i].match_type(BxDFType::Reflection as u32))
            // && (!reflect&&self.bxdfs[i].match_type(BxDFType::Transmission as u32))
            {
                f += bxdf.f(&w_out, w_in);
            }
        }
        f
    }
    pub fn num_components(&self, flag: u32) -> u32 {
        let mut num = 0;
        for ref item in self.bxdfs {
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
