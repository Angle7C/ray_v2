use glam::{Mat4, Vec2, Vec3};
use std::f32::consts::PI;
use std::sync::Arc;

use crate::pbrt_core::light::LightAble;
use crate::pbrt_core::primitive::Primitive;
use crate::pbrt_core::texture::Texture;
use crate::pbrt_core::tool::{Bound, InteractionCommon, Ray, SurfaceInteraction, Visibility};

#[derive(Debug)]
pub struct InfiniteLight {
    r: f32,
    center: Vec3,
    color: Arc<dyn Texture>,
    obj_to_world: Mat4,
    lemit: Vec3,
}

impl InfiniteLight {
    pub fn new(
        r: f32,
        center: Vec3,
        color: Arc<dyn Texture>,
        obj_to_world: Mat4,
        lemit: Vec3,
    ) -> Self {
        Self {
            r,
            color,
            center,
            obj_to_world,
            lemit,
        }
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
    fn interacect(&self, ray: crate::pbrt_core::tool::RayDiff) -> Option<SurfaceInteraction> {
        let mut surface = SurfaceInteraction::default();
        let hit_p = self.center + ray.o.dir * 2.0 * self.r;
        surface.light=self.get_light();
        surface.common =
            InteractionCommon::new(ray.o.dir, hit_p, ray.o.dir, f32::INFINITY, Vec2::ZERO);
        Some(surface)
    }
}

impl LightAble for InfiniteLight {
    fn sample_li(
        &self,
        surface: &SurfaceInteraction,
        u: Vec2,
        w_in: &mut Vec3,
        pdf: &mut f32,
        vis: &mut Visibility,
    ) -> Vec3 {
        let theta = u.x * PI;
        let phi = u.y * 2.0 * PI;
        // let (sin_t, cos_t) = theta.sin_cos();
        // let (sin_phi, cos_phi) = phi.sin_cos();
        // *w_in = self.obj_to_world.transform_vector3(Vec3::new(sin_t * cos_phi, sin_t * sin_phi, cos_t));
        let hit_p = surface.common.p + *w_in * 2.0 * self.r + Vec3::ONE;
        *pdf = 1.0;
        let common = InteractionCommon::new(*w_in, hit_p, hit_p.normalize(), f32::INFINITY, u);
        *vis = Visibility {
            a: surface.common,
            b: common,
        };
        self.color.evaluate(&common)
    }

    fn sample_le(&self, wi: Vec3, vis: &mut Visibility, surface: &InteractionCommon) -> Vec3 {
        // let theta = u.x * PI;
        // let phi = u.y * 2.0 * PI;

        let hit_p = surface.p + wi * 2.0 * self.r + Vec3::ONE;
        let common = InteractionCommon::new(
            wi,
            hit_p,
            hit_p.normalize(),
            f32::INFINITY,
            Default::default(),
        );
        *vis = Visibility {
            a: *surface,
            b: common,
        };
        self.color.evaluate(&common)
    }

    fn power(&self) -> Vec3 {
        let common = InteractionCommon::default();
        PI * self.r * self.r * self.color.evaluate(&common)
    }

    fn pdf_li(&self, surface: &InteractionCommon, w_in: &Vec3) -> f32 {
        1.0
    }

    fn le(&self, wi: Vec3) -> Vec3 {
        let w = self.obj_to_world.transform_vector3(wi);
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
        self.color.evaluate(&common)*self.lemit
        //dpdu,dpdv计算
    }
}
