use rtbvh::{Builder, Bvh};

use crate::pbrt_core::tool::SurfaceInteraction;

use super::{Aggregate, GeometricePrimitive, Primitive};

pub struct BVH<'b> {
    geo: Vec<GeometricePrimitive<'b>>,
    accel: rtbvh::Bvh,
}

impl<'b> BVH<'b> {
    pub fn new(mut shape: Vec<GeometricePrimitive<'b>>) -> Self {
        let builder = Builder {
            aabbs: None,
            primitives: shape.as_slice(),
            primitives_per_leaf: None,
        };
        let bvh = builder.construct_binned_sah().unwrap();
        // let flat_bvh = bvh::bvh::BVH::build(&mut shape);
        Self {
            geo: shape,
            accel: bvh,
        }
    }
}

impl<'b> Aggregate for BVH<'b> {
    fn interacect(&self, ray: &crate::pbrt_core::tool::RayDiff) -> Option<SurfaceInteraction> {
        let o_ray = ray.clone();
        let mut bvh_ray = rtbvh::Ray::new(ray.o.origin, ray.o.dir);
        let iter = self.accel.traverse_iter(&mut bvh_ray, &self.geo);
        // let mut ray = bvh::ray::Ray::new(o_ray.o.origin, o_ray.o.dir);
        // let iter = self.accel.traverse_iterator(&mut ray, &self.geo);
        let mut ans: Option<SurfaceInteraction> = None;
        let t_max = o_ray.o.t_max;
        let t_min = o_ray.o.t_min;
        let mut t = f32::MAX;
        for (shape,t_ray) in iter {
            match shape.interacect(o_ray) {
                Some(v) if v.common.time > t_min && v.common.time < t_max && t > v.common.time => {
                    t = v.common.time;
                    ans = Some(v);
                }
                _ => continue,
            }
            
        }
        ans
    }
    fn hit_p(&self,ray: &crate::pbrt_core::tool::RayDiff)->bool {
        let mut bvh_ray = rtbvh::Ray::new(ray.o.origin, ray.o.dir);
        let iter = self.accel.traverse_iter(&mut bvh_ray, &self.geo);
        // let mut ray = bvh::ray::Ray::new(o_ray.o.origin, o_ray.o.dir);
        // let iter = self.accel.traverse_iterator(&mut ray, &self.geo);
        for (shape,_) in iter {
           if shape.hit_p(ray){
                return true
           }
            
        }
        false
    }
}
