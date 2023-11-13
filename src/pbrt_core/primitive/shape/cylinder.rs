use std::f32::consts::PI;

use glam::{Mat4, Vec2, Vec3};

use crate::pbrt_core::{
    material::Material,
    primitive::Primitive,
    sampler::concentric_sample_disk,
    tool::{
        func::{self, compute_d2, lerp, quadratic},
        Bound, InteractionCommon, Ray, Shading, SurfaceInteraction,
    },
};

#[derive(Debug)]
pub struct Cylinder<'a> {
    radius: f32,
    height: f32,
    pub obj_to_world: Mat4,
    material: Option<&'a dyn Material>,
}

impl<'a> Cylinder<'a> {
    pub fn new(
        radius: f32,
        height: f32,
        obj_to_world: Mat4,
        material: Option<&'a dyn Material>,
    ) -> Self {
        Cylinder {
            radius,
            height,
            obj_to_world,
            material,
        }
    }
    pub fn sample_interaction(&self, common: &mut InteractionCommon, smaple_point: Vec2,pdf:&mut f32) {
        let z = lerp(smaple_point.x, 0.0, self.height);
        let pi = smaple_point.y * 2.0 * PI;
        let mut p_obj = Vec3::new(self.radius * pi.cos(), self.radius * pi.sin(), z);
        common.normal = self
            .obj_to_world
            .transform_vector3(Vec3::new(p_obj.x, p_obj.y, 0.0))
            .normalize();
        let hit_rad = (p_obj.x * p_obj.x + p_obj.y * p_obj.y).sqrt();
        p_obj.x *= self.radius / hit_rad;
        p_obj.y *= self.radius / hit_rad;
        common.p = self.obj_to_world.transform_point3(p_obj);
    }
}
impl<'a> Primitive for Cylinder<'a> {
    fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
        let min = Vec3::new(-self.radius, -self.radius, self.height);
        let max = Vec3::new(self.radius, self.radius, self.height);
        Bound::<3>::new(min, max)
    }

    fn hit_p(&self, ray: &crate::pbrt_core::tool::RayDiff) -> bool {
        let o = self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir);
        let ray = Ray::new(o, dir);
        let a = dir.x * dir.x + dir.y * dir.y;
        let b = 2.0 * (dir.x * o.x + dir.y * o.y);
        let c = o.x * o.x + o.y * o.y - self.radius * self.radius;
        if let Some((t1, t2)) = quadratic(a, b, c) {
            let t = (t1.min(t2)).max(ray.t_min);
            if t.abs() <= ray.t_min {
                false
            } else {
                let p = ray.at(t);
                if p.z < self.height && p.z > 0.0 {
                    return true;
                }
                false
            }
        } else {
            false
        }
    }
    fn interacect(
        &self,
        ray: crate::pbrt_core::tool::RayDiff,
    ) -> Option<crate::pbrt_core::tool::SurfaceInteraction> {
        let o = self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir);
        let ray = Ray::new(o, dir);
        let a = dir.x * dir.x + dir.y * dir.y;
        let b = 2.0 * (dir.x * o.x + dir.y * o.y);
        let c = o.x * o.x + o.y * o.y - self.radius * self.radius;
        let t: f32;
        let p: Vec3;
        if let Some((t1, t2)) = quadratic(a, b, c) {
            t = (t1.min(t2)).max(ray.t_min);
            if t <= ray.t_min {
                return None;
            }
            p = ray.at(t);
            if p.z > self.height || p.z < 0.0 {
                return None;
            }
        } else {
            return None;
        };
        let mut pi = p.y.atan2(p.x);
        if pi < 0.0 {
            pi += 2.0 * PI;
        };
        let u = pi / (2.0 * PI);
        let v = p.z / self.height;
        let uv = Vec2::new(u, v);

        let dpdu = Vec3::new(-p.y, p.x, 0.0);
        let dpdv = Vec3::new(0.0, 0.0, 1.0);

        let d2pduu = -4.0 * PI * PI * p.truncate().extend(0.0);
        let d2pduv = Vec3::ZERO;
        let d2pdvv = Vec3::ZERO;

        let (n, dndu, dndv) = compute_d2(dpdu, dpdv, d2pduu, d2pduv, d2pdvv);
        let shading = Shading::new(dpdu, dpdv, dndu, dndv);
        let common = InteractionCommon::new(-dir, p, n, t, uv);
        let mut item = SurfaceInteraction::new(common, shading, Some(self), None);
        func::transform_interaction(self.obj_to_world, &mut item);
        Some(item)
    }
    fn get_area(&self) -> f32 {
        self.height * 2.0 * PI * self.radius
    }
}
