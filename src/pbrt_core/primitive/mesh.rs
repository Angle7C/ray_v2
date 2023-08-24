use std::sync::Arc;

use glam::f64::{DVec3, DVec2};

use crate::pbrt_core::material::Material;
#[derive(Debug)]
pub struct Mesh{
    pub point: Vec<DVec3>,
    pub normal:Vec<DVec3>,
    pub uv:Vec<DVec2>,
    pub material:Vec<Arc<dyn Material>>
}
impl Mesh{
    pub fn new(point:Vec<DVec3>,normal:Vec<DVec3>,uv:Vec<DVec2>)->Self{
        Self { point, normal, uv, material: vec![] }
    }
    pub fn add_material(&mut self,material:Arc<dyn Material>)
    {
        self.material.push(material)
    }
}