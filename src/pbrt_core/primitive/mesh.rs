use std::sync::Arc;

use glam::{f64::{DVec3, DVec2}, DVec4};

use crate::pbrt_core::material::Material;
#[derive(Debug,Default)]

pub struct Mesh{
    pub point: Vec<DVec3>,
    pub normal:Vec<DVec3>,
    pub tangents:Vec<DVec4>,
    pub uv:Vec<DVec2>,
    pub material:Vec<Arc<dyn Material>>
}

impl Mesh{
    pub fn new(point:Vec<DVec3>,normal:Vec<DVec3>,uv:Vec<DVec2>,tangents:Vec<DVec4>,)->Self{
        Self { point, normal, uv, material: vec![],tangents }
    }
    pub fn add_material(&mut self,material:Arc<dyn Material>)
    {
        self.material.push(material)
    }
    pub fn add_message(&mut self,point:&mut Vec<DVec3>,normal:&mut Vec<DVec3>,uv:&mut Vec<DVec2>,tangents:&mut Vec<DVec4>){
        self.uv.append(uv);
        self.point.append(point);
        self.normal.append(normal);
        self.tangents.append(tangents)
    }
}