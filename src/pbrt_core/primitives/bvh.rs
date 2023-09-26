use bvh::bvh::BVH;

use crate::pbrt_core::tool::{interaction::SurfaceInteraction, ray::Ray};

use super::{GeometricPrimitive, Primitive};

pub struct Accel<'a> {
    accel: BVH,
    geometry: Vec<GeometricPrimitive<'a>>,
}
impl<'a> Accel<'a> {
    pub fn new(mut geometry: Vec<GeometricPrimitive<'a>>) -> Self {
        let accel = BVH::build(&mut geometry);
        Self { accel, geometry }
    }
    pub fn interacect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let bvh_ray: bvh::ray::Ray = ray.into();
        let iter = self.accel.traverse_iterator(&bvh_ray, &self.geometry);
        let mut ans: Option<SurfaceInteraction> = None;
        for item in iter {
            match (item.intersect(ray), &ans) {
                (Some(v), None) => ans = Some(v),
                (Some(v), Some(i)) if i.common.t > v.common.t => ans = Some(v),
                _ => continue,
            }
        }
        ans
    }
}
