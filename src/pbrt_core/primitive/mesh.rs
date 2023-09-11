

use glam::{
    f64::{DVec2, DVec3},
    DVec4,
};

#[derive(Debug, Default)]

pub struct Mesh {
    pub point: Vec<DVec3>,
    pub normal: Vec<DVec3>,
    pub tangents: Vec<DVec4>,
    pub uv: Vec<DVec2>,
}

impl Mesh {
    pub fn new(
        point: Vec<DVec3>,
        normal: Vec<DVec3>,
        uv: Vec<DVec2>,
        tangents: Vec<DVec4>,
    ) -> Self {
        Self {
            point,
            normal,
            uv,
            tangents,
        }
    }
}
