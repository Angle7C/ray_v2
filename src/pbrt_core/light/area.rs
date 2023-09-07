use std::f64::consts::PI;

use glam::{DVec3, DVec2, Vec3, DMat4};

use crate::pbrt_core::{primitive::{shape::Shape, Primitive}, tool::{SurfaceInteraction, InteractionCommon, func::vec3_coordinate_system, Visibility, Bound}, sampler::{cosine_sample_hemisphere, self}, bxdf::BxDFType};

use super::LightAble;

pub trait AreaLight: LightAble+Primitive {
    fn l(&self, surface: &InteractionCommon, w: &DVec3) -> DVec3 {
        todo!()
    }
    fn get_shape(&self)->&Shape;
}
#[derive(Debug)]
pub struct DiffuseAreaLight<'a> {
    lemit: DVec3,
    shape: Shape<'a>,
    area: f64,
}
impl<'a> DiffuseAreaLight<'a> {
    pub fn new(lemit: DVec3, shape: Shape<'a>) -> Self {
        Self {
            lemit,
            area: shape.agt_area(),
            shape,
        }
    }
}
impl<'a> AreaLight for DiffuseAreaLight<'a> {
    fn l(&self, surface: &InteractionCommon, w: &DVec3) -> DVec3 {
        if surface.normal.dot(*w) > 0.0 {
            self.lemit
        } else {
            DVec3::ZERO
        }
    }
    fn get_shape(&self)->&Shape {
        &self.shape
    }
}
impl<'a> LightAble for DiffuseAreaLight<'a> {
    fn power(&self) -> DVec3 {
        return self.area * PI * self.lemit;
    }
    fn sample_li(
        &self,
        surface: &SurfaceInteraction,
        //光源采样参数
        u: glam::DVec2,
        w_in: &mut DVec3,
        //光源pdf
        pdf: &mut f64,
        //可见性测试
        vis: &mut crate::pbrt_core::tool::Visibility,
    ) -> DVec3 {
        // 从shape采样到点
        let common=self.shape.sample(u);
        
        *w_in=(surface.common.p-common.p).normalize();
        
        *pdf=self.shape.pdf(&common, &w_in);
        *vis=Visibility{a:surface.common,b:common};
        let f=if let Some(bsdf) = &surface.bsdf {
            //BRDF 采样生成光线
            let w_out = *w_in;
            let mut wi=Default::default();
            let mut w_in = DVec3::default();
            let mut pdf = 0.0;
            let mut flags: u32 = BxDFType::All as u32;
            
                bsdf.sample_f(&w_out, &mut wi, u, &mut pdf, flags)
        }else{
            DVec3::ZERO
        };
        self.l(&common,&-*w_in)
    }
    fn sample_le(&self)->DVec3 {
        unimplemented!()
    }
    fn le(&self,wi:DVec3)->DVec3 {
        let w = self.shape.get_mat().inverse().transform_vector3(wi);
        let cos = w.dot(DVec3::Z).clamp(0.0, 1.0);
        self.lemit*cos

    }
    fn pdf_li(&self,surface:&InteractionCommon,w_in:&DVec3)->f64 {
        self.shape.pdf(surface, w_in)
    }
}
impl<'a> Primitive for DiffuseAreaLight<'a>{
    fn world_bound(&self) -> Bound<3> {
        self.shape.world_bound()
    }
    fn interacect(&self, ray: crate::pbrt_core::tool::RayDiff) -> Option<SurfaceInteraction> {
        let mut inter = self.shape.interacect(ray);
        if let Some( ref mut suface)=inter{
            suface.light=self.get_light();
        }
        inter
    }
    fn compute_scattering(&self, isct: &mut SurfaceInteraction, mode: crate::pbrt_core::bxdf::TransportMode) {
        self.shape.compute_scattering(isct, mode)
    }
    fn get_light(&self)->Option<&dyn LightAble> {
        Some(self)
    }
}