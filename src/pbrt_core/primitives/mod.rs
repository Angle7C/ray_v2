use std::sync::Arc;

use ::bvh::{aabb::AABB, bounding_hierarchy::BHShape};

use self::shape::ShapeAble;

use super::{
    light::LightAble,
    material::MaterialAble,
    tool::{bound::Bound, interaction::SurfaceInteraction, ray::Ray},
};

pub mod bvh;
pub mod shape;

pub trait Primitive {
    fn world_bound(&self) -> Bound<3>;
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;
    fn get_light(&self) -> Option<&dyn LightAble> {
        None
    }
    fn get_material(&self) -> Option<&dyn MaterialAble> {
        None
    }
}
pub struct GeometricPrimitive {
    pub shape: Arc<dyn ShapeAble>,
    pub light: Option<Arc<dyn LightAble>>,
    index: usize,
}
impl GeometricPrimitive {
    pub fn new(shape: Arc<dyn ShapeAble>) -> Self {
        Self {
            shape,
            light: None,
            index: 0,
        }
    }
}
impl ::bvh::aabb::Bounded for GeometricPrimitive {
    fn aabb(&self) -> AABB {
        self.shape.world_bound().into()
    }
}
impl BHShape for GeometricPrimitive {
    fn bh_node_index(&self) -> usize {
        self.index
    }
    fn set_bh_node_index(&mut self, i: usize) {
        self.index = i
    }
}
impl Primitive for GeometricPrimitive {
    fn world_bound(&self) -> Bound<3> {
        todo!()
    }

    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let mut item = self.shape.intersect(ray);
        match (item, &self.light) {
            (Some(mut  item), (light)) => {
                item.light = light;()
            }
            _ => (),
        };
        item
    }
}
