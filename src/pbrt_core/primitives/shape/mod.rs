

use crate::pbrt_core::tool::{interaction::SurfaceInteraction, bound::Bound, ray::Ray};

pub mod shpere;
pub mod rect;
pub mod disk;


pub trait ShapeAble {
    fn world_bound(&self)->Bound<3>;
    fn intersect(&self,ray:&Ray)->Option<SurfaceInteraction>{
        None
    }
    fn area(&self)->f32;
    fn get_index(&self)->usize{0}
}
pub enum Shape {
    
}