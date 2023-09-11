use bvh::ray::Ray;

use crate::pbrt_core::tool::SurfaceInteraction;

use super::{Aggregate, GeometricePrimitive, Primitive};

pub struct BVH<'b> {
    geo: Vec<GeometricePrimitive<'b>>,
    accel: bvh::bvh::BVH,
}
impl<'b> BVH<'b> {
    pub fn new(mut shape: Vec<GeometricePrimitive<'b>>) -> Self {
        let flat_bvh = bvh::bvh::BVH::build(&mut shape);
        Self {
            geo: shape,
            accel: flat_bvh,
        }
    }
}
impl<'b> Aggregate for BVH<'b> {
    fn interacect(&self, ray: &crate::pbrt_core::tool::RayDiff) -> Option<SurfaceInteraction> {
        let o_ray = ray.clone();
        let mut ray = Ray::new(o_ray.o.origin.as_vec3(), o_ray.o.dir.as_vec3());
        let iter = self.accel.traverse(&mut ray, &self.geo);
        let mut ans: Option<SurfaceInteraction> = None;
        let t_max = o_ray.o.t_max;
        let t_min = o_ray.o.t_min;
        let mut t = f64::INFINITY;
        for shape in iter {
            match (shape.interacect(o_ray), &ans) {
                (Some(v), None)
                    if v.common.time >= t_min && v.common.time <= t_max && v.common.time <= t =>
                {
                    t = v.common.time;

                    ans = Some(v);
                }
                (Some(v), Some(item))
                    if v.common.time >= t_min
                        && v.common.time <= item.common.time
                        && v.common.time <= t =>
                {
                    t = v.common.time;
                    ans = Some(v)
                }
                _ => continue,
            }
        }
        ans
    }
}
