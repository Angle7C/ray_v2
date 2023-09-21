use std::f32::consts::PI;
use std::sync::Arc;
use glam::{Mat4, Vec2, Vec3};

use crate::pbrt_core::light::LightAble;
use crate::pbrt_core::primitive::Primitive;
use crate::pbrt_core::texture::Texture;
use crate::pbrt_core::tool::{Bound, InteractionCommon, RayDiff, SurfaceInteraction, Visibility};

#[derive(Debug)]
pub struct InfiniteLight {
    r: f32,
    center: Vec3,
    color: Arc<dyn Texture>,
    obj_to_world: Mat4,
    lemit: Vec3,
}

impl InfiniteLight {
    pub fn new(r: f32,
               center: Vec3,
               color: Arc<dyn Texture>,
               obj_to_world: Mat4, lemit: Vec3) -> Self {
        Self { r, color, center, obj_to_world, lemit }
    }
}

impl Primitive for InfiniteLight {
    fn world_bound(&self) -> Bound<3> {
        let max = self.center + Vec3::splat(self.r);
        let min = self.center - Vec3::splat(self.r);
        Bound::<3>::new(min, max)
    }
    fn get_light(&self) -> Option<&dyn LightAble> {
        Some(self)
    }
    fn interacect(&self, ray: RayDiff) -> Option<SurfaceInteraction> {
        return None;
        // let mut interaction = SurfaceInteraction::default();
        // interaction.light=self.get_light();
        // interaction.common.normal=-ray.o.dir+Vec3::ONE*0.002;
        // interaction.common.time=f32::MAX;
        // Some(interaction)
    }
}

impl LightAble for InfiniteLight {
    fn sample_li(&self, surface_common: &InteractionCommon, light_common: &mut InteractionCommon, u: Vec2, wi: &mut Vec3, pdf: &mut f32, vis: &mut Visibility) -> Vec3 {
        unimplemented!()
        // let theta = u.x * PI;
        // let phi = u.y * 2.0 * PI;
        // let (sin_t, cos_t) = theta.sin_cos();
        // let (sin_phi, cos_phi) = phi.sin_cos();
        // *w_in = self.obj_to_world.transform_vector3(Vec3::new(sin_t * cos_phi, sin_t * sin_phi, cos_t));
        // let hit_p = surface.common.p + *w_in * 2.0 * self.r;
        // let common = InteractionCommon::new(*w_in, hit_p, *w_in, 01.0, u);
        // *vis = Visibility { a: surface.common, b: common };
        // *pdf=1.0;
        // self.color.evaluate(&common)
    }


    fn power(&self) -> Vec3 {
        let mut common = InteractionCommon::default();
        common.uv=Vec2::splat(0.5);
        PI * self.r * self.r * self.color.evaluate(&common)
    }

    fn pdf_li(&self, surface: &InteractionCommon, w_in: &Vec3) -> f32 {
        1.0
    }

    fn le(&self, wi: Vec3) -> Vec3 {
        let w = self.obj_to_world.inverse().transform_vector3(wi).normalize();
        let mut phi = (w.y).atan2(w.x);
        //uv计算
        if phi < 0.0 {
            phi += 2.0 * PI;
        }
        let theta = w.z.clamp(-1.0, 1.0).acos();
        let v = theta / PI;
        let u = phi / (2.0 * PI);
        let uv = Vec2::new(u, v);
        let mut common = InteractionCommon::default();
        common.uv = uv;
        self.color.evaluate(&common)
        //dpdu,dpdv计算
    }
}