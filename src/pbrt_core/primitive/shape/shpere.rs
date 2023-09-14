use std::{f32::consts::PI, sync::Arc};

use glam::{Mat4, Vec2, Vec3};

use crate::pbrt_core::{
    material::Material,
    primitive::Primitive,
    tool::{Bound, Shading, SurfaceInteraction},
};
#[derive(Debug)]
pub struct Shpere {
    r: f32,
    obj_to_world: Mat4,
    material: Option<Arc<dyn Material>>,
}
impl Shpere {
    pub fn new(r: f32, material: Option<Arc<dyn Material>>, obj_to_world: Mat4) -> Self {
        Self {
            r,
            obj_to_world,
            material,
        }
    }
}
impl Primitive for Shpere {
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
        let l = -o;
        let s = l.dot(dir);
        let r_2 = self.r * self.r;
        let l_2 = l.dot(l);
        if s < 0.0 && l_2 > r_2 {
            return None;
        }
        let m = l_2 - r_2;
        if m > r_2 {
            return None;
        }
        let q = (r_2 - m).sqrt();
        let t = if l_2 > r_2 { s - q } else { s + q };
        let p = o + dir * t;
        //uv计算
        let mut phi = (p.z / p.x).atan();
        if phi < 0.0 {
            phi += 2.0 * PI;
        }
        let v = p.y.acos();
        let uv = Vec2::new(phi / (2.0 * PI), 1.0 - v / PI);
        //dpdu,dpdv计算
        let z_radius = p.x * p.x + p.y * p.y;
        let inv_z_radius = 1.0 / z_radius;
        let cos_phi = p.x * inv_z_radius;
        let sin_phi = p.y * inv_z_radius;
        let dpdu = Vec3::new(2.0 * PI * p.y, 2.0 * PI * p.x, 0.0);
        let dpdv = PI * Vec3::new(p.z * cos_phi, p.z * sin_phi, self.r * v.sin());
        //dndv,dndv计算
        let d2pduu = 4.0 * PI * PI * p.truncate().extend(0.0);
        let d2pduv = PI * p.z * 2.0 * PI * Vec3::new(-sin_phi, cos_phi, 0.0);
        let d2pdvv = -(PI * PI) * p;

        let e = dpdu.dot(dpdu);
        let f = dpdu.dot(dpdv);
        let g = dpdv.dot(dpdv);
        let n = dpdu.cross(dpdv);
        let ee = n.dot(d2pduu);
        let ff = n.dot(d2pduv);
        let gg = n.dot(d2pdvv);
        let inv_egf = 1.0 / (e * g - f * f);
        let dndu = (ff * f - ee * g) * inv_egf * dpdu + (ee * f - ff * e) * inv_egf * dpdv;
        let dndv = (gg * f - ff * g) * inv_egf * dpdu + (ff * f - gg * e) * inv_egf * dpdv;
        let shading = Shading::new(n, dpdu, dpdv, dndu, dndv);
        let p = self.obj_to_world.transform_point3(p);
        let normal = self.obj_to_world.transform_vector3(p);
        Some(SurfaceInteraction::new(
            p, uv, normal, -ray.o.dir, shading.dpdu, shading.dpdv, shading.dndu, shading.dndv, t, Some(self), None,
        ))
    }
}
