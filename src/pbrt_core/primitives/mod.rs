

use ::bvh::{aabb::AABB, bounding_hierarchy::BHShape};

use self::shape::ShapeAble;

use super::{
    light::LightAble,
    material::MaterialAble,
    tool::{bound::Bound, interaction::SurfaceInteraction, ray::Ray},
};

pub mod shape;
pub mod bvh;

pub trait Primitive{
    fn world_bound(&self) -> Bound<3>;
    fn intersect(&self,ray:&Ray) -> Option<SurfaceInteraction>;
    fn get_light(&self) -> Option<&dyn LightAble> {
        None
    }
    fn get_material(&self) -> Option<&dyn MaterialAble> {
        None
    }
}
pub struct GeometricPrimitive<'a>{
   pub shape: &'a dyn  ShapeAble,
   pub material: Option<&'a dyn MaterialAble>,
   pub light: Option<&'a dyn LightAble>,
    index: usize,
}
impl<'a> ::bvh::aabb::Bounded for GeometricPrimitive<'a> {
    fn aabb(&self) -> AABB {
        self.shape.world_bound().into()
    }
}
impl<'a> BHShape for GeometricPrimitive<'a>{
    fn bh_node_index(&self) -> usize {
        self.index
    }
    fn set_bh_node_index(&mut self, i: usize) {
        self.index=i
    }
}
impl Primitive for GeometricPrimitive<'_>{
    fn world_bound(&self) -> Bound<3> {
        todo!()
    }

    fn intersect(&self,ray:&Ray) -> Option<SurfaceInteraction> {
        todo!()
    }
}