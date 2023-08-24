
use bvh::{flat_bvh::FlatBVH, bounding_hierarchy::BoundingHierarchy, ray::Ray};

use crate::pbrt_core::tool::{setting::Build, Bound, SurfaceInteraction};

use super::{GeometricePrimitive, Primitive, Aggregate};

pub struct BVH<'a>{
    geo: Vec<GeometricePrimitive<'a>>,
    accel: bvh::bvh::BVH
}
impl<'a> BVH<'a>{
    pub fn new(mut shape: Vec<GeometricePrimitive<'a>>)->Self{
       let flat_bvh = bvh::bvh::BVH::build(&mut shape);
       Self { geo: shape, accel: flat_bvh }
    }
}
impl<'a> Aggregate for BVH<'a>{
    fn interacect(&self, ray: &crate::pbrt_core::tool::RayDiff) -> Option<SurfaceInteraction> {
        let o_ray=ray.clone();
        let mut ray=Ray::new(o_ray.o.origin.as_vec3(), o_ray.o.dir.as_vec3());
        let iter = self.accel.traverse(&mut ray,&self.geo);
        let mut ans:Option<SurfaceInteraction>=None;
        for shape in iter{
            match (shape.shape.interacect(o_ray),&ans){
                (None,_)=>continue,
                (Some(v),None)=>{
                    ans=Some(v);
                },
                (Some(v),Some(item))=>{
                    if v.common.time<item.common.time{
                        ans=Some(v)
                    }
                }
            }
        };
        ans
    }
}
