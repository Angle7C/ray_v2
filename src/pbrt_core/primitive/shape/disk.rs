use std::f32::consts::PI;

use glam::{Mat4, Vec2, Vec3};

use crate::pbrt_core::{
    material::Material,
    primitive::Primitive,
    tool::{func, Bound, InteractionCommon, Ray, SurfaceInteraction}, sampler::concentric_sample_disk,
};

#[derive(Debug)]
pub struct Disk<'a> {
    radius: f32,
    inner_radius: f32,
    pub obj_to_world: Mat4,
    material: Option<&'a dyn Material>,
    height: f32,
}

impl<'a> Disk<'a> {
    pub fn new(
        radius: f32,
        inner_radius: f32,
        obj_to_world: Mat4,
        material: Option<&'a dyn Material>,
        height: f32,
    ) -> Self {
        Self {
            radius,
            inner_radius,
            obj_to_world,
            material,
            height,
        }
    }
    pub fn sample_interaction(&self,common: &mut InteractionCommon,smaple_point: Vec2,pdf:&mut f32){
        *pdf=1.0/self.get_area();
        let pd=concentric_sample_disk(smaple_point);
        let p= Vec3::new(pd.x*self.radius, pd.y*self.radius,self.height);
        common.normal=self.obj_to_world.transform_vector3(Vec3::Z).normalize();
        common.p=self.obj_to_world.transform_point3(p);
        
    }
}
impl<'a> Primitive for Disk<'a> {
    fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
        let min = Vec3::new(-self.radius, -self.radius, self.height);
        let max = Vec3::new(self.radius, self.radius, self.height);
        Bound::<3>::new(min, max)
    }

    fn hit_p(&self, ray: &crate::pbrt_core::tool::RayDiff) -> bool {
        let o = self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir);
        let ray = Ray::new(o, dir);
        let t = (self.height - o.z) / dir.z;
        let p = ray.at(t);
        p.x * p.x + p.y * p.y < self.radius * self.radius && p.x * p.x + p.y * p.y > self.inner_radius * self.inner_radius
    }
    fn get_area(&self) -> f32 {
        self.radius * self.radius * std::f32::consts::PI
            - self.inner_radius * self.inner_radius * std::f32::consts::PI
    }
    fn interacect(
        &self,
        ray: crate::pbrt_core::tool::RayDiff,
    ) -> Option<crate::pbrt_core::tool::SurfaceInteraction> {
        let o = self.obj_to_world.inverse().transform_point3(ray.o.origin);
        let dir = self.obj_to_world.inverse().transform_vector3(ray.o.dir);
        let ray = Ray::new(o, dir);
        let t = (self.height - o.z) / dir.z;
        let p = ray.at(t);
        let dist = p.x * p.x + p.y * p.y;
        if p.x * p.x + p.y * p.y < self.radius * self.radius
            && p.x * p.x + p.y * p.y > self.inner_radius * self.inner_radius
        {
           
            let mut pi = p.y.atan2(p.x);
            if pi < 0.0 {
                pi += 2.0 * std::f32::consts::PI;
            }
            let u = pi / (2.0 * PI);

            let one_minus_v = (dist.sqrt() - self.inner_radius) / (self.radius - self.inner_radius);
            let v = 1.0 - one_minus_v;
            let uv = Vec2::new(u, v);


            let dpdu = 2.0 * PI * Vec3::new(-p.x, p.y, 0.0);
            let dpdv = Vec3::new(p.x, p.y, 0.0) * (self.inner_radius - self.radius) / dist.sqrt();
            let dndu = Vec3::new(0.0, 0.0, 0.0);
            let dndv = Vec3::new(0.0, 0.0, 0.0);

            let shading = crate::pbrt_core::tool::Shading::new(dpdu, dpdv, dndu, dndv);
            let n = Vec3::Z;
            let common = InteractionCommon::new(-dir, p, n, t, uv);
            let mut item = SurfaceInteraction::new(common, shading, Some(self), None);
            func::transform_interaction(self.obj_to_world, &mut item);
            Some(item)
        } else {
            None
        }
    }
}
