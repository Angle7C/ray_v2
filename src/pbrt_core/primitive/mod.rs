use std::fmt::Debug;

use ::bvh::{aabb::Bounded, bounding_hierarchy::BHShape};

use super::{
    bxdf::TransportMode,
    light::LightAble,
    tool::{Bound, RayDiff, SurfaceInteraction},
};
// use ::bvh::{aabb::Bounded, bounding_hierarchy::BHShape};

pub mod bvh;
pub mod mesh;
pub mod shape {
    use self::{rectangle::Rectangle, shpere::Shpere, cylinder::Cylinder, disk::Disk};
    use super::Primitive;
    use crate::pbrt_core::tool::InteractionCommon;
    use glam::{Vec2, Vec3};
    pub mod rectangle;
    pub mod shpere;
    pub mod triangle;
    pub mod cylinder;
    pub mod disk;
    #[derive(Debug)]
    pub enum Shape<'a> {
        Rect(Rectangle<'a>),
        Shpere(Shpere<'a>),
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
                Self::Shpere(sphere) => sphere.compute_scattering(isct, mode),
                Shape::Cylinder(cylinder) => cylinder.compute_scattering(isct, mode),
                Shape::Disk(disk) => disk.compute_scattering(isct, mode),
                _=>todo!()
            }
        }
        fn interacect(
            &self,
            ray: crate::pbrt_core::tool::RayDiff,
        ) -> Option<crate::pbrt_core::tool::SurfaceInteraction> {
            match &self {
                Shape::Rect(rect) => rect.interacect(ray),
                Shape::Shpere(sphere) => sphere.interacect(ray),
                Shape::Cylinder(cylinder) => cylinder.interacect(ray),
                Shape::Disk(disk)=>disk.interacect(ray)
            }
        }
        fn world_bound(&self) -> crate::pbrt_core::tool::Bound<3> {
            match &self {
                Shape::Rect(rect) => rect.world_bound(),
                Shape::Shpere(sphere) => sphere.world_bound(),   
                Shape::Cylinder(cylinder) => cylinder.world_bound(),
                Shape::Disk(disk)=>disk.world_bound()
            }
        }
        fn hit_p(&self, ray: &crate::pbrt_core::tool::RayDiff) -> bool {
            match &self {
                Shape::Rect(rect) => rect.hit_p(ray),
                Shape::Shpere(sphere) => sphere.hit_p(ray),
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
                Shape::Shpere(sphere) => sphere.get_area(), 
                Shape::Cylinder(cylinder) => cylinder.get_area(),
                Shape::Disk(disk)=>disk.get_area(),
            }
        }
        // 形状采样
        pub fn sample(&self, smaple_point: Vec2, common: &mut InteractionCommon, pdf: &mut f32) {
            match self {
                Self::Rect(rect) => rect.sample_interaction(common, smaple_point,pdf),
                Self::Shpere(sphere) => sphere.sample_interaction(common, smaple_point,pdf),
                Self::Cylinder(cylinder) => cylinder.sample_interaction(common, smaple_point,pdf),
                Self::Disk(disk)=>disk.sample_interaction(common, smaple_point,pdf),
                _=>todo!(),
            }
        }
        //对于在不同点采样的时，会存在不同pdf值。给定指定方向与点，确定是否有交点。
        pub fn pdf(&self, _common: &InteractionCommon, _wi: &Vec3) -> f32 {
            1.0 / self.agt_area()
        }
        pub fn get_mat(&self) -> glam::Mat4 {
            match self {
                Self::Rect(rect) => rect.obj_to_world,
                Self::Shpere(sphere) => sphere.obj_to_world,
                _=>todo!()
            }
        }
        pub fn get_cos(&self,dir:Vec3)->Option<f32>{
            match self {
                Self::Rect(rect) => rect.get_cos(dir),
                Self::Shpere(sphere) => sphere.get_cos(dir),
                _=>todo!()
            }
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
    fn compute_scattering(&self, _isct: &mut SurfaceInteraction, _mode: TransportMode) {}
    //获取光源
    fn get_light(&self) -> Option<&dyn LightAble> {
        None
    }
    fn get_area(&self) -> f32 {
        1.0
    }
    fn hit_p(&self, ray: &RayDiff) -> bool;
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
pub struct GeometricePrimitive<'a> {
    primitive: &'a dyn Primitive,
    light: Option<&'a dyn LightAble>,
    node_index: usize,
}
unsafe impl<'a> Sync for GeometricePrimitive<'a> {}
unsafe impl<'a> Send for GeometricePrimitive<'a> {}
impl<'a> GeometricePrimitive<'a> {
    pub fn new(primitive: &'a dyn Primitive) -> Self {
        Self {
            primitive,
            node_index: 0,
            light: None,
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
    fn hit_p(&self, ray: &RayDiff) -> bool {
        self.primitive.hit_p(ray)
    }
}
