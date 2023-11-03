use glam::{Mat4, Vec2, Vec3};

use crate::pbrt_core::{
    material::Material,
    primitive::Primitive,
    tool::{func::transform_interaction, Bound, InteractionCommon, SurfaceInteraction, Shading, color::Color},
};
#[derive(Debug)]
pub struct Rectangle<'a> {
    pub obj_to_world: Mat4,
    material: Option<&'a dyn Material>,
}
impl<'a> Rectangle<'a> {
    pub fn new(obj_to_world: Mat4, material: Option<&'a dyn Material>) -> Self {
        Self {
            obj_to_world,
            material,
        }
    }
    pub fn get_area(&self) -> f32 {
        let p1 = self.obj_to_world.transform_vector3(Vec3::X);
        let p2 = self.obj_to_world.transform_vector3(Vec3::Y);
        p1.cross(p2).length()
        // DMat2::from_cols(self.obj_to_world.x_axis.xy(), self.obj_to_world.y_axis.xy()).determinant()
    }
    pub fn sample_interaction(&self, commom: &mut InteractionCommon, sampler_point: Vec2) {
        let p = self
            .obj_to_world
            .transform_point3(sampler_point.extend(0.0));
        commom.p = p;
        commom.normal = self
            .obj_to_world
            .inverse()
            .transpose()
            .transform_vector3(Vec3::Z);
    }
    pub fn get_cos(&self,dir:Vec3)->Option<f32>{
        let dir = self.obj_to_world.inverse().transform_vector3(dir);
        let cos = Vec3::Z.dot(dir);
        if cos>0.0{
            Some(cos)
        }else{
            None
        }
    }
}
impl<'a> Primitive for Rectangle<'a> {
    fn compute_scattering(
        &self,
        isct: &mut crate::pbrt_core::tool::SurfaceInteraction,
        mode: crate::pbrt_core::bxdf::TransportMode,
    ) {
        if let Some(materil) = &self.material {
            materil.compute_scattering_functions(isct, mode)
        }
    }
    fn interacect(
        &self,
        ray: crate::pbrt_core::tool::RayDiff,
    ) -> Option<crate::pbrt_core::tool::SurfaceInteraction> {
        let o = self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir);
        let t = -o.z / dir.z;
        let p = o + dir * t;
        if t < ray.o.t_min || t > ray.o.t_max{
            return None;
        }
        if p.x < 0.0 || p.x > 1.0 {
            return None;
        }
        if p.y < 0.0 || p.y > 1.0  {
            return None;
        }
        let common=InteractionCommon::new(-dir, p, Vec3::Z, t, p.truncate());
        let shading=Shading::new(Vec3::X, Vec3::Y, Vec3::ZERO, Vec3::ZERO);
        let mut surface = SurfaceInteraction::new(
            common,
            shading,
            Some(self),
            None
        );
        transform_interaction(self.obj_to_world, &mut surface);
        Some(surface)
    }
    fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
        let min = self.obj_to_world.transform_point3(Vec3::ZERO) - Vec3::splat(0.003);
        let max = self.obj_to_world.transform_point3(Vec3::ONE) + Vec3::splat(0.003);
        Bound::<3>::new(min, max)
    }
    fn hit_p(&self, ray: &crate::pbrt_core::tool::RayDiff) -> bool {
        let o = self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir);
        let t = -o.z / dir.z;
        let p = o + dir * t;
        if t < ray.o.t_min || t > ray.o.t_max{
            return false;
        }
        if p.x <= 0.0 || p.x >= 1.0 {
            return false;
        }
        if p.y <= 0.0 || p.y >= 1.0 {
            return false;
        }
        true
    }
}
