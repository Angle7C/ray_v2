use std::f32::consts::PI;

use glam::{Mat4, Vec2, Vec3};

use crate::pbrt_core::{
    material::Material,
    primitive::Primitive,
    tool::{func, Bound, Shading, SurfaceInteraction},
};
#[derive(Debug)]
pub struct Shpere<'a> {
    r: f32,
    obj_to_world: Mat4,
    material: Option<&'a dyn Material>,
}
impl<'a> Shpere<'a> {
    pub fn new(r: f32, material: Option<&'a dyn Material>, obj_to_world: Mat4) -> Self {
        Self {
            r,
            obj_to_world,
            material,
        }
    }
}
impl<'a> Primitive for Shpere<'a> {
    fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
        let min = Vec3::splat(-self.r);
        let max = Vec3::splat(self.r);
        let min = self.obj_to_world.transform_point3(min);
        let max = self.obj_to_world.transform_point3(max);
        Bound::<3>::new(min, max)
    }
    fn compute_scattering(
        &self,
        isct: &mut crate::pbrt_core::tool::SurfaceInteraction,
        mode: crate::pbrt_core::bxdf::TransportMode,
    ) {
        if let Some(material) = &self.material {
            material.compute_scattering_functions(isct, mode)
        }
    }
    fn interacect(
        &self,
        ray: crate::pbrt_core::tool::RayDiff,
    ) -> Option<crate::pbrt_core::tool::SurfaceInteraction> {
        let o = self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir);
        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(o);
        let c = o.dot(o) - self.r * self.r;
        let t: f32;
        if let Some((t1, t2)) = func::quadratic(a, b, c) {
            t = (t1.min(t2)).max(ray.o.t_min);
            if t <= ray.o.t_min {
                return None;
            }
        } else {
            return None;
        }
        let p_hit = o + t * dir;
        let p = p_hit;
        let mut phi = (p.y).atan2(p.x);
        //uv计算
        if phi < 0.0 {
            phi += 2.0 * PI;
        }
        let theta = (p.z / self.r).clamp(-1.0, 1.0).acos();
        let uv = Vec2::new(phi / (2.0 * PI), theta / PI);
        //dpdu,dpdv计算
        // let (sin_phi, cos_phi) = phi.sin_cos();
        let z_radius = p.truncate().length();
        let inv_radius = 1.0 / z_radius;
        let (sin_phi, cos_phi) = (p.y * inv_radius, p.x * inv_radius);
        let dpdu = 2.0 * PI * Vec3::new(-p.y, p.x, 0.0);
        let dpdv = PI * Vec3::new(p.z * cos_phi, p.z * sin_phi, -self.r * theta.sin());
        //dndv,dndv计算
        let d2pduu = -4.0 * PI * PI * p.truncate().extend(0.0);
        let d2pduv = -PI * p.z * 2.0 * PI * Vec3::new(-sin_phi, cos_phi, 0.0);
        let d2pdvv = -(PI * PI) * p;

        let e = dpdu.dot(dpdu);
        let f = dpdu.dot(dpdv);
        let g = dpdv.dot(dpdv);
        let n = dpdu.cross(dpdv).normalize();
        let ee = n.dot(d2pduu);
        let ff = n.dot(d2pduv);
        let gg = n.dot(d2pdvv);
        let inv_egf = 1.0 / (e * g - f * f);
        let dndu = (ff * f - ee * g) * inv_egf * dpdu + (ee * f - ff * e) * inv_egf * dpdv;
        let dndv = (gg * f - ff * g) * inv_egf * dpdu + (ff * f - gg * e) * inv_egf * dpdv;
        let shading = Shading::new(dpdu, dpdv, dndu, dndv);
        let mut item = SurfaceInteraction::new(
            p,
            uv,
            Vec3::ZERO-p.normalize(),
            -ray.o.dir,
            shading.dpdu,
            shading.dpdv,
            shading.dndu,
            shading.dndv,
            t,
            Some(self),
            None,
        );
        func::transform_interaction(self.obj_to_world, &mut item);
        Some(item)
    }
    fn hit_p(&self,ray:&crate::pbrt_core::tool::RayDiff)->bool {
        let o = self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir);
        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(o);
        let c = o.dot(o) - self.r * self.r;
        let t: f32;
        if let Some((t1, t2)) = func::quadratic(a, b, c) {
            t = if t1<0.0&&t2>0.0{
                t2
            }else if t2<0.0&&t1>0.0{
                t1
            }else if t1>0.0&&t2>0.0{
                t1.min(t2)
            }else{
                f32::MIN
            };
            if t <ray.o.t_min||t> ray.o.t_max{
                return false;
            }
        } else {
            return false;
        }
        true
    }
}
