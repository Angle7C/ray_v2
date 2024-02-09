use std::{fmt::Debug, sync::Arc};

use ::bvh::{aabb::Bounded, bounding_hierarchy::BHShape};
use glam::{Vec2, Vec3};
use crate::pbrt_core::tool::InteractionCommon;

use super::{
    bxdf::TransportMode,
    light::LightAble,
    tool::{Bound, RayDiff, SurfaceInteraction},
};
// use ::bvh::{aabb::Bounded, bounding_hierarchy::BHShape};

pub mod bvh;
pub mod mesh;
pub mod shape {
    use self::{rectangle::Rectangle, sphere::Sphere, cylinder::Cylinder, disk::Disk};
    use super::Primitive;
    use crate::pbrt_core::tool::InteractionCommon;
    use glam::{Vec2, Vec3};
    pub mod rectangle;
    pub mod sphere;
    pub mod triangle;
    pub mod cylinder;
    pub mod disk;
    #[derive(Debug)]
    pub enum Shape<'a> {
        Rect(Rectangle),
        Sphere(Sphere),
        Cylinder(Cylinder<'a>),
        Disk(Disk<'a>),
    }

    impl<'a> Primitive for Shape<'a> {
        fn compute_scattering(
            &self,
            isct: &mut crate::pbrt_core::tool::SurfaceInteraction,
            mode: crate::pbrt_core::bxdf::TransportMode,
        ) {
            match &self {
                Shape::Rect(rect) => rect.compute_scattering(isct, mode),
                Self::Sphere(sphere) => sphere.compute_scattering(isct, mode),
                Shape::Cylinder(cylinder) => cylinder.compute_scattering(isct, mode),
                Shape::Disk(disk) => disk.compute_scattering(isct, mode),
                _=>todo!()
            }
        }
        fn interact(
            &self,
            ray: crate::pbrt_core::tool::RayDiff,
        ) -> Option<crate::pbrt_core::tool::SurfaceInteraction> {
            match &self {
                Shape::Rect(rect) => rect.interact(ray),
                Shape::Sphere(sphere) => sphere.interact(ray),
                Shape::Cylinder(cylinder) => cylinder.interact(ray),
                Shape::Disk(disk)=>disk.interact(ray)
            }
        }
        fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
            match &self {
                Shape::Rect(rect) => rect.world_bound(),
                Shape::Sphere(sphere) => sphere.world_bound(),
                Shape::Cylinder(cylinder) => cylinder.world_bound(),
                Shape::Disk(disk)=>disk.world_bound()
            }
        }
        fn hit_p(&self, ray: &crate::pbrt_core::tool::RayDiff) -> bool {
            match &self {
                Shape::Rect(rect) => rect.hit_p(ray),
                Shape::Sphere(sphere) => sphere.hit_p(ray),
                Shape::Cylinder(cylinder) => cylinder.hit_p(ray),
                Shape::Disk(disk)=>disk.hit_p(ray)
            }
        }
        
    }
    impl<'a> Shape<'a> {
        // 获得面积
        pub fn agt_area(&self) -> f32 {
            match self {
                Shape::Rect(rect) => rect.get_area(),
                Shape::Sphere(sphere) => sphere.get_area(),
                Shape::Cylinder(cylinder) => cylinder.get_area(),
                Shape::Disk(disk)=>disk.get_area(),
            }
        }
        // 形状采样
        pub fn sample(&self, sample_point: Vec2, common: &mut InteractionCommon, pdf: &mut f32) {
            match self {
                Self::Rect(rect) => rect.sample_interaction(common, sample_point, pdf),
                Self::Sphere(sphere) => sphere.sample_interaction(common, sample_point, pdf),
                Self::Cylinder(cylinder) => cylinder.sample_interaction(common, sample_point, pdf),
                Self::Disk(disk)=>disk.sample_interaction(common, sample_point, pdf),
                _=>todo!()
            }
        }
        //对于在不同点采样的时，会存在不同pdf值。给定指定方向与点，确定是否有交点。
        pub fn pdf(&self, _common: &InteractionCommon, _wi: &Vec3) -> f32 {
            1.0 / self.agt_area()
        }
        pub fn get_mat(&self) -> glam::Mat4 {
            match self {
                Self::Rect(rect) => rect.obj_to_world,
                Self::Sphere(sphere) => sphere.obj_to_world,
                _=>todo!()
            }
        }
        pub fn get_cos(&self,dir:Vec3)->Option<f32>{
            match self {
                Self::Rect(rect) => rect.get_cos(dir),
                Self::Sphere(sphere) => sphere.get_cos(dir),
                _=>todo!()
            }
        }
    }
}
pub trait Primitive: Debug {
    //世界包围盒
    fn world_bound(&self) -> Bound<3>;
    //求交
    fn interact(&self, _ray: RayDiff) -> Option<SurfaceInteraction> {
        None
    }
    //包围盒求交
    fn interact_bound(&self, ray: &RayDiff) -> bool {
        self.world_bound().intesect(ray)
    }
    //材质计算
    fn compute_scattering(&self, _isct: &mut SurfaceInteraction, _mode: TransportMode) {}
    //获取光源
    fn get_light(&self) -> Option<&dyn LightAble> {
        None
    }
    //获取图元面积
    fn get_area(&self) -> f32 {
        1.0
    }
    fn hit_p(&self, ray: &RayDiff) -> bool;
    //在图元上进行采样
    fn sample(&self,uv:Vec2, surface_common: &mut InteractionCommon,pdf:&mut f32)->Vec3{
        Vec3::default()
    }

    fn pdf(&self,interaction_common: &InteractionCommon,wi:&Vec3)->f32{
        1.0
    }
}
pub trait Aggregate: Sync {
    fn interacect(&self, ray: &RayDiff) -> Option<SurfaceInteraction>;
    fn hit_p(&self, ray: &RayDiff) -> bool;
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
pub struct GeometricPrimitive {
    primitive: Arc<dyn Primitive>,
    light: Option<Arc< dyn LightAble>>,
    node_index: usize,
}
unsafe impl Sync for GeometricPrimitive {}
unsafe impl Send for GeometricPrimitive {}
impl GeometricPrimitive {
    pub fn new(primitive: Arc< dyn Primitive>) -> Self {
        Self {
            primitive,
            node_index: 0,
            light: None,
        }
    }
}
impl Bounded for GeometricPrimitive {
    fn aabb(&self) -> ::bvh::aabb::AABB {
        let bound = self.primitive.world_bound();
        bound.into()
    }
}
impl BHShape for GeometricPrimitive{
    fn set_bh_node_index(&mut self, i: usize) {
        self.node_index = i
    }
    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}
impl Primitive for GeometricPrimitive {
    fn world_bound(&self) -> Bound<3> {
        self.primitive.world_bound()
    }
    fn interact(&self, ray: RayDiff) -> Option<SurfaceInteraction> {
        self.primitive.interact(ray)
    }
    fn interact_bound(&self, ray: &RayDiff) -> bool {
        self.primitive.interact_bound(ray)
    }
    fn compute_scattering(&self, isct: &mut SurfaceInteraction, mode: TransportMode) {
        self.primitive.compute_scattering(isct, mode)
    }
    fn get_light(&self) -> Option<&dyn LightAble> {
        self.primitive.get_light()
    }
    fn hit_p(&self, ray: &RayDiff) -> bool {
        self.primitive.hit_p(ray)
    }
}
