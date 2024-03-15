use std::f32::consts::PI;


use glam::{Mat4, Vec2, Vec3};
use crate::pbrt_core::tool::{func::{self, unifrom_sample_sphere}, Bound, InteractionCommon, Shading};

use super::ShapeAble;
#[derive(Debug)]
pub struct Sphere {
    r: f32,
    obj_to_world: Mat4,
    reverse_orientation:bool
}
impl Sphere {
    pub fn new(r: f32, obj_to_world: Mat4) -> Self {
        Self {
            r,
            obj_to_world,
            reverse_orientation:false
        }
    }
}
impl ShapeAble for Sphere{
    #[inline]
    fn bound(&self)->Bound<3> {
        let min=Vec3::new(-self.r, -self.r, -self.r);
        let max=Vec3::new(self.r, self.r, self.r);
        Bound::<3>::new(min, max)
    }

    #[inline]
    fn world_bound(&self)->Bound<3> {
        let min=Vec3::new(-self.r, -self.r, -self.r);
        let max=Vec3::new(self.r, self.r, self.r);
        let min = self.obj_to_world.transform_point3(min);
        let max = self.obj_to_world.transform_point3(max);
        Bound::<3>::new(min, max)
    }

    fn intersect(&self, ray: crate::pbrt_core::tool::RayDiff) -> Option<InteractionCommon> {
        let o = self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir);
        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(o);
        let c = o.dot(o) - self.r * self.r;
        let t: f32;
        if let Some((t1, t2)) = func::quadratic(a, b, c) {
            t = t1.min(t2);
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
        let common = InteractionCommon::new(dir, p, n, t, uv,shading);
        let common=func::transform_common(self.obj_to_world, common);
        Some(common)
    }

    fn intersect_p(&self, ray: &crate::pbrt_core::tool::RayDiff) -> bool {
        let o = self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir);
        let a = dir.dot(dir);
        let b = 2.0 * dir.dot(o);
        let c = o.dot(o) - self.r * self.r;
        let t: f32;
        if let Some((t1, t2)) = func::quadratic(a, b, c) {
            t = t1.min(t2);
            if t<ray.o.t_min||t>ray.o.t_max{
                return false;
            }
        } else {
            return false;
        }
        true
    }

    fn area(&self)->f32 {
        PI*self.r*self.r*4.0
    }

    fn sample(&self,u:Vec2,pdf:&mut f32)->InteractionCommon {
        let p=unifrom_sample_sphere(u);
        let mut common=InteractionCommon::default();
       
        common.normal = self
            .obj_to_world
            .inverse()
            .transpose()
            .transform_vector3(p)
            .normalize();
        *pdf=1.0/self.area();
        common
    }

    fn sample_with_ref_point(&self,_common:&InteractionCommon,_u:Vec2,_pdf:&mut f32)->InteractionCommon {
        todo!()
    }

    fn pdf_with_ref_point(&self,_common:&InteractionCommon,_w_in:&Vec3)->f32 {
        todo!()
    }
    fn obj_to_world(&self)->Mat4 {
        self.obj_to_world
    }
}
