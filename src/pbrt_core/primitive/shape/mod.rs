use std::fmt::Debug;

use glam::{Vec2, Vec3};

use crate::pbrt_core::tool::{Bound, InteractionCommon, RayDiff, Shading, SurfaceInteraction};

pub mod rectangle;
pub mod sphere;
pub mod triangle;
pub mod cylinder;
pub mod disk;

//几何具有的公有行为
pub trait ShapeAble:Debug {
    //本地包围盒
    fn bound(&self)->Bound<3>;
    //世界包围盒
    fn world_bound(&self)->Bound<3>;
    //表面求交
    fn intersect(&self, ray: RayDiff) -> Option<InteractionCommon>;
    //是否有交点
    fn intersect_p(&self, ray: &RayDiff) -> bool;
    //形状面积
    fn area(&self)->f32;
    //对该形状进行采样
    fn sample(&self,u:Vec2,pdf:&mut f32)->InteractionCommon;
    //对该形状上的某一点进行采样
    fn sample_with_ref_point(&self,common:&InteractionCommon,u:Vec2,pdf:&mut f32)->InteractionCommon;
    //对该点采样的PDF值
    fn pdf_with_ref_point(&self,common:&InteractionCommon,w_in:&Vec3)->f32;
    //计算shading
    fn computer_shadering(&self,_common:&InteractionCommon)->Shading{
        Shading::default()
    }
}