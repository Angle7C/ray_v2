use std::f64::consts::PI;

use glam::{DVec3, DVec2, Vec3, DMat4};

use crate::pbrt_core::{primitive::{shape::Shape, Primitive}, tool::{SurfaceInteraction, InteractionCommon, func::vec3_coordinate_system, Visibility, Bound}, sampler::cosine_sample_hemisphere};

use super::LightAble;

pub trait AreaLight: LightAble+Primitive {
    fn l(&self, surface: &InteractionCommon, w: &DVec3) -> DVec3 {
        todo!()
    }
    fn le(&self, w: &DVec3) -> DVec3 {
        DVec3::ZERO
    }
    fn get_shape(&self)->&Shape;
}
#[derive(Debug)]
pub struct DiffuseAreaLight {
    lemit: DVec3,
    shape: Shape,
    area: f64,
    obj_to_world:DMat4,
}
impl DiffuseAreaLight {
    pub fn new(lemit: DVec3, shape: Shape) -> Self {
        Self {
            lemit,
            area: shape.agt_area(),
            shape,
            obj_to_world:Default::default()
        }
    }
}
impl AreaLight for DiffuseAreaLight {
    fn l(&self, surface: &InteractionCommon, w: &DVec3) -> DVec3 {
        if surface.normal.dot(*w) > 0.0 {
            self.lemit
        } else {
            DVec3::ZERO
        }
    }
    fn le(&self, w: &DVec3) -> DVec3 {
        let w = self.obj_to_world.inverse().transform_vector3(*w);
        let cos = w.dot(DVec3::Z).abs();
        self.lemit*cos
    }
    fn get_shape(&self)->&Shape {
        &self.shape
    }
}
impl LightAble for DiffuseAreaLight {
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
        self.l(&common,&-*w_in)
    }
    fn sample_le(&self)->DVec3 {
        unimplemented!()
    }

    fn pdf_li(&self,surface:&InteractionCommon,w_in:&DVec3)->f64 {
        self.shape.pdf(surface, w_in)
    }
}
impl Primitive for DiffuseAreaLight{
    fn world_bound(&self) -> Bound<3> {
        self.shape.world_bound()
    }
    fn interacect(&self, ray: crate::pbrt_core::tool::RayDiff) -> Option<SurfaceInteraction> {
       unimplemented!()
    }
    fn compute_scattering(&self, isct: &mut SurfaceInteraction, mode: crate::pbrt_core::bxdf::TransportMode) {
        self.shape.compute_scattering(isct, mode)
    }
}