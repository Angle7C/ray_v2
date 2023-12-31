use std::sync::Arc;

use glam::{u32::UVec3, Mat4, Vec2, Vec3};
use gltf::mesh::util::tex_coords;

use crate::pbrt_core::{
    bxdf::TransportMode,
    material::Material,
    primitive::{mesh::Mesh, Primitive},
    tool::{Bound, RayDiff, Shading, SurfaceInteraction, InteractionCommon},
};
#[derive(Debug)]
pub struct Triangle<'a> {
    point_index: [usize; 3],
    noraml_index: [usize; 3],
    tex_index: [usize; 3],
    mesh: Arc<Mesh>,
    obj_to_world: Mat4,
    materail: Option<&'a Box<dyn Material + 'a>>,
}
#[allow(unused)]
impl<'a> Triangle<'a> {
    pub fn new(
        point_index: UVec3,
        noraml_index: UVec3,
        tex_index: UVec3,
        mesh: Arc<Mesh>,
        obj_to_world: Mat4,
        materail: Option<&'a Box<dyn Material + 'a>>,
    ) -> Self {
        Self {
            point_index: [
                point_index.x as usize,
                point_index.y as usize,
                point_index.z as usize,
            ],
            noraml_index: [
                noraml_index.x as usize,
                noraml_index.y as usize,
                noraml_index.z as usize,
            ],
            tex_index: [
                tex_index.x as usize,
                tex_index.y as usize,
                tex_index.z as usize,
            ],
            mesh,
            materail,
            obj_to_world,
        }
    }
    fn compute_dnuv(&self, n: Vec3) -> Shading {
        let p0 = self.point(0);
        let p1 = self.point(1);
        let p2 = self.point(2);

        let n0 = self.normal(0);
        let n1 = self.normal(1);
        let n2 = self.normal(2);

        let uv0 = self.uv(0);
        let uv1 = self.uv(1);
        let uv2 = self.uv(2);

        let duv_02 = uv0 - uv2;
        let duv_12 = uv1 - uv2;
        let dp_02 = p0 - p2;
        let dp_12 = p1 - p2;

        let deter = duv_02[0] * duv_12[1] - duv_02[1] * duv_12[0];
        let v = (p2 - p0).cross(p1 - p0).normalize();
        let (dpdu, dpdv, dndu, dndv) = if deter.abs() < f32::EPSILON {
            let dpdu = if v.x.abs() > v.y.abs() {
                Vec3::new(-v.z, 0.0, v.x) / (v.x * v.x + v.z * v.z).sqrt()
            } else {
                Vec3::new(-0.0, v.z, -v.y) / (v.y * v.y + v.z * v.z).sqrt()
            };
            let dpdv = v.cross(dpdu);
            (dpdu, dpdv, Vec3::ZERO, Vec3::ZERO)
        } else {
            let inv_det = 1.0 / deter;
            let dn1 = n0 - n2;
            let dn2 = n1 - n2;

            (
                (duv_12[1] * dp_02 - duv_02[1] * dp_12) * inv_det,
                (-duv_12[0] * dp_02 + duv_02[0] * dp_12) * inv_det,
                (duv_12[1] * dn1 - duv_02[1] * dn2) * inv_det,
                (-dp_12[0] * dn1 + duv_02[0] * dn2) * inv_det,
            )
        };
        Shading::new(dpdu, dpdv, Vec3::ZERO, Vec3::ZERO)
    }
    pub fn point(&self, i: u32) -> Vec3 {
        self.obj_to_world
            .transform_point3(self.mesh.point[self.point_index[i as usize]])
    }
    pub fn normal(&self, i: u32) -> Vec3 {
        if self.mesh.normal.is_empty() {
            Vec3::ZERO
        } else {
            self.obj_to_world
                .inverse()
                .transpose()
                .transform_vector3(self.mesh.normal[self.noraml_index[i as usize]])
        }
    }
    pub fn tangent(&self, i: u32) -> Vec3 {
        if self.mesh.normal.is_empty() {
            Vec3::ZERO
        } else {
            // self.mesh.tangent[self.index[i as usize]]
            Vec3::ZERO
        }
    }
    pub fn uv(&self, i: u32) -> Vec2 {
        if self.mesh.uv.is_empty() {
            Vec2::ZERO
        } else {
            self.mesh.uv[self.tex_index[i as usize]]
        }
    }
}
impl<'a> Primitive for Triangle<'a> {
    fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
        let p0 = self.point(0);
        let p1 = self.point(1);
        let p2 = self.point(2);
        let min = p0.min(p1).min(p2);
        let max = p0.max(p1).max(p2);

        Bound::<3>::new(min, max)
    }
    fn interacect(&self, ray: RayDiff) -> Option<crate::pbrt_core::tool::SurfaceInteraction> {
        let p0 = self.point(0);
        let p1 = self.point(1);
        let p2 = self.point(2);

        let n0 = self.normal(0);
        let n1 = self.normal(1);
        let n2 = self.normal(2);

        let uv0 = self.uv(0);
        let uv1 = self.uv(1);
        let uv2 = self.uv(2);

        let e1 = p1 - p0;
        let e2 = p2 - p0;
        let s = ray.o.origin - p0;
        let s1 = ray.o.dir.cross(e2);
        let s2 = s.cross(e1);
        let s1_e1 = s1.dot(e1);

        let t = s2.dot(e2) / s1_e1;
        let a = s1.dot(s) / s1_e1;
        let b = s2.dot(ray.o.dir) / s1_e1;
        let c = 1.0 - a - b;

        if t < 0.0
            || !(0.0..=1.0).contains(&b)
            || !(0.0..=1.0).contains(&a)
            || !(0.0..=1.0).contains(&c)
        {
            None
        } else {
            let (a, b, c) = (c, a, b);
            let p = p0 * a + p1 * b + p2 * c;
            let normal = (n0 * a + n1 * b + n2 * c).normalize();
            let uv = uv0 * a + uv1 * b + uv2 * c;
            let shading = self.compute_dnuv(normal.normalize());
            let common = InteractionCommon::new(ray.o.dir,p , normal, t, uv);
            let item = SurfaceInteraction::new(common, shading, Some(self), None);
            Some(item)
        }
    }
    fn compute_scattering(&self, surface: &mut SurfaceInteraction, mode: TransportMode) {
        match &self.materail {
            Some(material) => material.compute_scattering_functions(surface, mode),
            None => (),
        }
    }
    fn hit_p(&self, _ray: &RayDiff) -> bool {
        false
    }
}
