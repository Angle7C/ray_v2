


use crate::pbrt_core::tool::SurfaceInteraction;

use super::{Aggregate, GeometricPrimitive, Primitive};

pub struct BVH {
    geo: Vec<GeometricPrimitive>,
    accel: bvh::bvh::BVH,
}
unsafe impl Sync for BVH{}
impl BVH {
    pub fn new(mut shape: Vec<GeometricPrimitive>) -> Self {
        let bvh = bvh::bvh::BVH::build(&mut shape);
        Self {
            geo: shape,
            accel: bvh,
        }
    }
}

impl Aggregate for BVH {
    fn intersect(&self, ray: &crate::pbrt_core::tool::RayDiff) -> Option<SurfaceInteraction> {
        let o_ray = *ray;
        let bvh_ray = bvh::ray::Ray::new(ray.o.origin, ray.o.dir);
        let iter = self.accel.traverse_iterator(&bvh_ray, &self.geo);
        let mut ans: Option<SurfaceInteraction> = None;
        let t_max = o_ray.o.t_max;
        let t_min = o_ray.o.t_min;
        let mut t = f32::MAX;
        for shape in iter {
            match shape.intersect(o_ray) {
                Some(v) if v.common.time > t_min && v.common.time < t_max && t > v.common.time => {
                    t = v.common.time;
                    ans = Some(v);
                }
                _ => continue,
            }
            
        }
        ans
    }
    fn intersect_p(&self,ray: &crate::pbrt_core::tool::RayDiff)->bool {
        let bvh_ray = bvh::ray::Ray::new(ray.o.origin, ray.o.dir);
        let iter = self.accel.traverse_iterator(&bvh_ray, &self.geo);
        for shape in iter {
           if shape.intersect_p(ray) {
                return true
           }
            
        }
        false
    }
}
