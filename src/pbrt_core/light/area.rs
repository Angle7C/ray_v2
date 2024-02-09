use std::sync::Arc;
use glam::{Vec2, Vec3};
use log::info;

use crate::pbrt_core::light::LightType;
use crate::pbrt_core::tool::color::Color;
use crate::pbrt_core::{
    primitive::{shape::Shape, Primitive},
    tool::{Bound, InteractionCommon, RayDiff, SurfaceInteraction, Visibility},
};

use super::LightAble;

#[derive(Debug)]
pub struct DiffuseAreaLight {
    lemit: Vec3,
    shape: Arc<dyn Primitive>,
    index: usize,
}

impl DiffuseAreaLight {
    pub fn new(lemit: Vec3, shape: Arc<dyn Primitive>, index: usize) -> Self {
        Self {
            lemit,
            shape,
            index,
        }
    }
}
impl LightAble for DiffuseAreaLight {
    fn get_n_sample(&self) -> usize {
        64
    }
    fn sample_li(
        &self,
        surface_common: &InteractionCommon,
        light_common: &mut InteractionCommon,
        u: Vec2,
        wi: &mut Vec3,
        pdf: &mut f32,
        vis: &mut Visibility,
    ) -> Vec3 {
        self.shape.sample(u, light_common, pdf);
        if pdf.abs() < f32::EPSILON
            || (light_common.p - surface_common.p).length_squared().abs() < f32::EPSILON
        {
            *pdf = 0.0;
            Vec3::ZERO
        } else {
            *wi = (surface_common.p - light_common.p).normalize();
            *vis = Visibility {
                a: *light_common,
                b: *surface_common,
            };
            self.li(light_common, wi)
        }
    }
    fn li(&self, inter: &InteractionCommon, w: &Vec3) -> Color {
        if inter.normal.dot(*w) > 0.0 {
            self.lemit
        } else {
            Vec3::ZERO
        }
    }
    fn pdf_li(&self, surface: &SurfaceInteraction, wi: &Vec3) -> f32 {
        self.shape.pdf(&surface.common, wi)
    }
    fn get_type(&self) -> LightType {
        LightType::Area
    }
    fn get_index(&self) -> usize {
        self.index
    }
    fn le(&self, ray: &RayDiff) -> Color {
        // let cos=self.get_shape().get_cos(-ray.o.dir);
        // if cos.is_some(){
        //     self.lemit
        // } else {
        //     Color::ZERO
        // }
        todo!()
    }
}

impl Primitive for DiffuseAreaLight {
    fn world_bound(&self) -> Bound<3> {
        self.shape.world_bound()
    }
    fn interact(&self, ray: crate::pbrt_core::tool::RayDiff) -> Option<SurfaceInteraction> {
        let mut inter = self.shape.interact(ray);
        if let Some(ref mut suface) = inter {
            info!("{:?}",self.index);
            suface.light = Some(self);
        }
        inter
    }
    fn compute_scattering(
        &self,
        isct: &mut SurfaceInteraction,
        mode: crate::pbrt_core::bxdf::TransportMode,
    ) {
        self.shape.compute_scattering(isct, mode)
    }
    fn get_light(&self) -> Option<&dyn LightAble> {
        Some(self)
    }
    fn hit_p(&self,ray:&RayDiff)->bool {
        self.shape.hit_p(ray)
    }
}
