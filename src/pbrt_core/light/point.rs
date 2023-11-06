

use glam::{Vec2, Vec3};

use crate::pbrt_core::{
    primitive::Primitive,
    tool::{Bound, InteractionCommon, Visibility, RayDiff},
};
use crate::pbrt_core::light::LightType;
use crate::pbrt_core::tool::color::Color;

use super::LightAble;

#[derive(Debug, Clone, Copy)]
// #[derive(Debug,)]
pub struct Point {
    p: Vec3,
    lemit: Vec3,
    index:usize
}

impl Point {
    pub fn new(lemit: Vec3, p: Vec3,index:usize) -> Self {
        Self { p, lemit,index }
    }
}

impl LightAble for Point {
    #[inline]
    fn le(&self, _ray: &RayDiff) -> Vec3 {
        Vec3::ZERO
    }
    #[inline]
    fn pdf_li(&self, _surface: &crate::pbrt_core::tool::SurfaceInteraction, _wi: &Vec3) -> f32 {
        0.0
    }
    #[inline]
    fn sample_li(&self, surface_common: &InteractionCommon, light_common: &mut InteractionCommon, _u: Vec2, wi: &mut Vec3, pdf: &mut f32, vis: &mut Visibility) -> Vec3 {
        *wi = (surface_common.p - self.p).normalize();
        *pdf = 1.0;
        light_common.p = self.p;
        light_common.time = surface_common.time;
        light_common.normal=-*wi;
        *vis = Visibility {
            a: *light_common,
            b: *surface_common,
        };
        self.lemit/ self.p.distance_squared(surface_common.p)
    }
    #[inline]
    fn get_type(&self) -> LightType {
        LightType::DeltaPosition
    }
    #[inline]
    fn get_n_sample(&self) -> usize {
        1
    }
    #[inline]
    fn li(&self, inter: &InteractionCommon, _w: &Vec3) -> Color {
        self.lemit*(inter.p-self.p).length_recip()
    }
    fn get_index(&self)->usize {
        self.index   
    }
}
impl Primitive for Point {
    fn get_area(&self) -> f32 {
        1.0
    }
    fn get_light(&self) -> Option<&dyn super::LightAble> {
        Some(self)
    }
    fn interacect(
        &self,
        _ray: crate::pbrt_core::tool::RayDiff,
    ) -> Option<crate::pbrt_core::tool::SurfaceInteraction> {
        None
    }
    fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
        
        Bound::<3>::new(Vec3::splat(-0.0003) + self.p, Vec3::splat(0.0003) + self.p)
    }
    fn hit_p(&self,_ray:&RayDiff)->bool {
        false
    }
}
