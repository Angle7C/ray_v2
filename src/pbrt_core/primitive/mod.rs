use std::fmt::Debug;

use ::bvh::{aabb::Bounded, bounding_hierarchy::BHShape};
use glam::f32::Vec3;

use super::{
    bxdf::TransportMode,
    light::{Light, LightAble},
    tool::{Bound, InteractionCommon, RayDiff, SurfaceInteraction},
};

pub mod bvh;
pub mod mesh;
pub mod shape {
    use self::rectangle::Rectangle;
    use super::Primitive;
    use crate::pbrt_core::tool::InteractionCommon;
    use glam::{DVec2, DVec3};
    pub mod rectangle;
    pub mod shpere;
    pub mod triangle;
    #[derive(Debug)]
    pub enum Shape {
        Rect(Rectangle),
    }
    impl Primitive for Shape {
        fn compute_scattering(
            &self,
            isct: &mut crate::pbrt_core::tool::SurfaceInteraction,
            mode: crate::pbrt_core::bxdf::TransportMode,
        ) {
            match &self {
                Shape::Rect(rect) => rect.compute_scattering(isct, mode),
            }
        }
        fn interacect(
            &self,
            ray: crate::pbrt_core::tool::RayDiff,
        ) -> Option<crate::pbrt_core::tool::SurfaceInteraction> {
            match &self {
                Shape::Rect(rect) => rect.interacect(ray),
            }
        }
        fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
            match &self {
                Shape::Rect(rect) => rect.world_bound(),
            }
        }
    }
    impl Shape {
        // 获得面积
        pub fn agt_area(&self) -> f64 {
            match self {
                Shape::Rect(rect) => rect.get_area(),
            }
        }
        // 形状采样
        pub fn sample(&self, smaple_point: DVec2) -> InteractionCommon {
            match self {
                Self::Rect(rect) => rect.sample_interaction(smaple_point),
            }
        }
        //对于在不同点采样的时，会存在不同pdf值
        pub fn pdf(&self, common: &InteractionCommon, wi: &DVec3) -> f64 {
            1.0 / self.agt_area()
        }
    }
}
pub trait Primitive: Debug {
    //世界包围盒
    fn world_bound(&self) -> Bound<3>;
    //求交
    fn interacect(&self, _ray: RayDiff) -> Option<SurfaceInteraction> {
        None
    }
    //包围盒求交
    fn interacect_bound(&self, ray: &RayDiff) -> bool {
        self.world_bound().intesect(ray)
    }
    //材质计算
    fn compute_scattering(&self, isct: &mut SurfaceInteraction, mode: TransportMode) {}
    //获取光源
    fn get_light(&self) -> Option<&dyn LightAble> {
        None
    }
}
pub trait Aggregate: Sync {
    fn interacect(&self, ray: &RayDiff) -> Option<SurfaceInteraction>;
}
#[derive(Debug)]
pub enum ObjectType {
    Light,
    Sence,
}
impl PartialEq for ObjectType {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ObjectType::Light, ObjectType::Light) => true,
            (ObjectType::Sence, ObjectType::Sence) => true,
            _ => false,
        }
    }
}
#[derive(Debug)]
pub struct GeometricePrimitive<'a> {
    primitive: &'a dyn Primitive,
    node_index: usize,
}
unsafe impl<'a> Sync for GeometricePrimitive<'a> {}
unsafe impl<'a> Send for GeometricePrimitive<'a> {}
impl<'a> GeometricePrimitive<'a> {
    pub fn new(primitive: &'a dyn Primitive) -> Self {
        Self {
            primitive,
            node_index: 0,
        }
    }
}
impl<'a> Bounded for GeometricePrimitive<'a> {
    fn aabb(&self) -> ::bvh::aabb::AABB {
        let bound = self.primitive.world_bound();
        bound.into()
    }
}
impl<'a> BHShape for GeometricePrimitive<'a> {
    fn bh_node_index(&self) -> usize {
        self.node_index
    }
    fn set_bh_node_index(&mut self, i: usize) {
        self.node_index = i
    }
}
impl<'a> Primitive for GeometricePrimitive<'a> {
    fn compute_scattering(&self, isct: &mut SurfaceInteraction, mode: TransportMode) {
        self.primitive.compute_scattering(isct, mode)
    }
    fn interacect(&self, ray: RayDiff) -> Option<SurfaceInteraction> {
        self.primitive.interacect(ray)
    }
    fn interacect_bound(&self, ray: &RayDiff) -> bool {
        self.primitive.interacect_bound(ray)
    }
    fn world_bound(&self) -> Bound<3> {
        self.primitive.world_bound()
    }
    fn get_light(&self) -> Option<&dyn LightAble> {
        self.primitive.get_light()
    }
}
