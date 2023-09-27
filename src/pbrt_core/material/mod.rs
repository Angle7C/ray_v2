use glam::{Mat3A, Vec3A, Vec2};
use rand::Rng;

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
                f += bxdf.f(&wo, &wi);
            }
        }
        f
    }
    pub fn sample_f(&self,wo: Vec3A, wi: &mut Vec3A,u:&Vec2, pdf: &mut f32,flag: u32,sample_type:&mut u32) -> Color {
        let w0=self.world_to_local(wo);
        let bxdfs = self.bxdfs.iter().filter(|item|item.match_type(flag))
                .collect::<Vec<_>>();
        if bxdfs.len()==0{
            return Color::ZERO;
        }

        let num=rand::thread_rng().gen_range(0..bxdfs.len());
        let bxdf=bxdfs[num];
        let mut f=bxdf.sample_f(&w0, wi, u, pdf, Some(sample_type));
        if *pdf<f32::EPSILON{
            *sample_type=0;
            return Color::ZERO;
        }
        if bxdf.get_type() &BxDFType::Specular as u32 >0 &&bxdfs.len()>1{
            for (i,item) in bxdfs.iter().enumerate(){
                if i!=num{
                    *pdf+=item.pdf(&w0, wi)
                }
            }
            *pdf/=bxdfs.len() as f32;
        }
        *wi=self.local_to_world(*wi);

        if bxdf.get_type() &BxDFType::Specular as u32 ==0{
            let reflect=wi.dot(self.ng)*wo.dot(self.ng)>0.0;
            f=Color::ZERO;
            for item in bxdfs{
                if(reflect&&item.get_type() & BxDFType::Reflection as u32>0)
                || (!reflect&&item.get_type()& BxDFType::Transmission as u32>0){
                    f+=item.f(&w0, wi)
                }
            }
        };
        f
    }
    pub fn pdf(&self,wo:&Vec3A,wi:&Vec3A,flag: u32)->f32{
        unimplemented!()
    }

}
