use std::f32::consts::PI;

use glam::{Affine3A, Vec3A, Vec2, Vec3};

use crate::pbrt_core::{
    material::MaterialAble,
    primitives::Primitive,
    tool::{bound::Bound, func, ray::Ray, interaction::{Shading, SurfaceInteraction, InteractionCommon}},
};

use super::ShapeAble;

pub struct Shpere<'a> {
    obj_to_world: Affine3A,
    r: f32,
    material: Option<&'a dyn MaterialAble>,
}
impl<'a> Shpere<'a> {
    pub fn new(    obj_to_world: Affine3A,
        r: f32,
        material: Option<&'a dyn MaterialAble>,)->Self{
            Self { obj_to_world, r, material }
        }
}
impl<'a> ShapeAble for Shpere<'a>{
    fn world_bound(&self)->Bound<3> {
        let min = Vec3A::new(-self.r, -self.r, -self.r);
        let max = Vec3A::new(self.r, self.r, self.r);
        Bound::<3>::new(min, max)
    }

    fn area(&self)->f32 {
      4.0*PI*self.r
    }
}
impl<'a> Primitive for Shpere<'a> {
    fn world_bound(&self) -> crate::pbrt_core::tool::bound::Bound<3> {
        let min = Vec3A::new(-self.r, -self.r, -self.r);
        let max = Vec3A::new(self.r, self.r, self.r);
        Bound::<3>::new(min, max)
    }

    fn intersect(
        &self,
        ray: &Ray,
    ) -> Option<crate::pbrt_core::tool::interaction::SurfaceInteraction> {
        let o = self.obj_to_world.inverse().transform_point3a(ray.o);
        let dir = self.obj_to_world.inverse().transform_vector3a(ray.dir);
        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(o);
        let c = o.dot(o) - self.r * self.r;
        let t: f32;
        if let Some((t1, t2)) = func::quadratic(a, b, c) {
            t = (t1.min(t2)).max(ray.t_min);
            if t < ray.t_max {
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
        let v = theta / PI;
        let u = phi / (2.0 * PI);
        let uv = Vec2::new(u, v);
        //dpdu,dpdv计算
        // let (sin_phi, cos_phi) = phi.sin_cos();
        let z_radius = p.truncate().length();
        let inv_radius = 1.0 / z_radius;
        let (sin_phi, cos_phi) = (p.y * inv_radius, p.x * inv_radius);
        let dpdu = Vec3::new(-p.y, p.x, 0.0);
        let dpdv = Vec3::new(p.z * cos_phi, p.z * sin_phi, -self.r * theta.sin());
        //dndv,dndv计算
        let d2pduu = -4.0 * PI * PI * p.truncate().extend(0.0);
        let d2pduv = PI * p.z * 2.0 * PI * p;
        let d2pdvv = -(PI * PI) * p;

        let e = dpdu.dot(dpdu);
        let f = dpdu.dot(dpdv);
        let g = dpdv.dot(dpdv);
        let n = dpdu.cross(dpdv).normalize();
        let ee = n.dot(d2pduu);
        let ff = n.dot(d2pduv.into());
        let gg = n.dot(d2pdvv.into());
        let inv_egf = 1.0 / (e * g - f * f);
        let dndu = (ff * f - ee * g) * inv_egf * dpdu + (ee * f - ff * e) * inv_egf * dpdv;
        let dndv = (gg * f - ff * g) * inv_egf * dpdu + (ff * f - gg * e) * inv_egf * dpdv;
        let shading = Shading::new(n.into(), dpdu.into(), dpdv.into(), dndu.into(), dndv.into());
        let p = self.obj_to_world.transform_point3a(p_hit);
        let normal = self.obj_to_world.transform_vector3a(p_hit).normalize();
        Some(SurfaceInteraction::new(
            InteractionCommon::new(p, t, -ray.dir, n.into()),
            uv,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            shading,
            None,
        ))
    }
}
