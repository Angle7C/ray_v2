use glam::{Mat4, Vec2, Vec3};
use std::f32::consts::PI;
use std::sync::Arc;

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
    index:usize
}

impl InfiniteLight {
    pub fn new(
        r: f32,
        center: Vec3,
        color: Arc<dyn Texture>,
        obj_to_world: Mat4,
        lemit: Vec3,
        index:usize
    ) -> Self {
        Self {
            r,
            color,
            center,
            obj_to_world,
            lemit,
            index
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
    fn interacect(&self, _ray: RayDiff) -> Option<SurfaceInteraction> {
        return None;
        // let mut interaction = SurfaceInteraction::default();
        // interaction.light=self.get_light();
        // interaction.common.normal=-ray.o.dir+Vec3::ONE*0.002;
        // interaction.common.time=f32::MAX;
        // Some(interaction)
    }
    fn hit_p(&self,_ray:&RayDiff)->bool {
        false
    }
}

impl LightAble for InfiniteLight {

    fn pdf_li(&self, _surface: &SurfaceInteraction<'_>, _w_in: &Vec3) -> f32 {
        1.0
    }

    fn le(&self, ray: &RayDiff) -> Vec3 {
        let w = self.obj_to_world.inverse().transform_vector3(ray.o.dir).normalize();
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
        self.color.evaluate(&common) * self.lemit
        //dpdu,dpdv计算
    }

    fn get_type(&self) -> super::LightType {
        super::LightType::Infinite
    }

    fn li(&self, inter: &InteractionCommon, wi: &Vec3) -> crate::pbrt_core::tool::color::Color {
        let mut phi = wi.y.atan2(wi.x);
        let theta = wi.z;
        if phi < 0.0 {
            phi += 2.0 * PI;
        }
        let (sin_t, cos_t) = theta.sin_cos();
        let (sin_phi, cos_phi) = phi.sin_cos();
        let w_in =
            self.obj_to_world
                .transform_vector3(Vec3::new(sin_t * cos_phi, sin_t * sin_phi, cos_t));
        let hit_p = inter.p + w_in * 2.0 * self.r;
        let common = InteractionCommon::new(*wi, hit_p, *wi, 01.0, Vec2::new(phi / 2.0 * PI, theta / PI));
        self.color.evaluate(&common)
    }

    fn get_n_sample(&self) -> usize {
        32
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
        let phi=u.x*2.0*PI;
        let theta=u.y*PI;
        let (sin_t, cos_t) = theta.sin_cos();
        let (sin_phi, cos_phi) = phi.sin_cos();
        *wi=Vec3::new(sin_t*cos_phi, sin_t*sin_phi, cos_t);
        let p=surface_common.p+*wi*self.r*2.0;
        *light_common=InteractionCommon::new(*wi, p, -*wi, 0.01, u);
        *vis = Visibility {
            a: *light_common,
            b: *surface_common,
        };
        *pdf = 1.0;
        self.color.evaluate(&light_common)*self.lemit
    }
    fn get_index(&self)->usize {
        self.index   
    }

}
