use glam::{Mat4, Vec2, Vec3};

use crate::pbrt_core::{
    material::Material,
    primitive::Primitive,
    tool::{Bound, InteractionCommon, SurfaceInteraction},
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
    pub fn sample_interaction(&self, sampler_point: Vec2) -> InteractionCommon {
        let p = self
            .obj_to_world
            .transform_point3(sampler_point.extend(0.0));
        let mut commom = InteractionCommon {
            ..Default::default()
        };
        commom.p = p;
        commom.normal = self.obj_to_world.transform_vector3(Vec3::Z);
        commom
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
        if p.x < 0.0 || p.x > 1.0 {
            return None;
        }
        if p.y < 0.0 || p.y > 1.0 {
            return None;
        }
        let p = self.obj_to_world.transform_point3(p).normalize();
        let n = self.obj_to_world.transform_vector3(Vec3::Z).normalize();
        let dpdu = self.obj_to_world.transform_point3(Vec3::X).normalize();
        let dpdv = self.obj_to_world.transform_point3(Vec3::Y).normalize();
        let surface = SurfaceInteraction::new(
            p,
            p.truncate(),
            n,
            -ray.o.dir,
            dpdu,
            dpdv,
            Vec3::ZERO,
            Vec3::ZERO,
            t,
            Some(self),
            None,
        );
        Some(surface)
    }
    fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
        let min = self.obj_to_world.transform_point3(Vec3::ZERO) - Vec3::splat(0.003);
        let max = self.obj_to_world.transform_point3(Vec3::ONE) + Vec3::splat(0.003);
        Bound::<3>::new(min, max)
    }
}
