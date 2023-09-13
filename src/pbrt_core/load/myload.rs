use glam::{DVec3, Mat4, Quat, Vec2, Vec3, Vec4};
use serde::{Deserialize, Serialize};
// use serde::{Deserialize, Serialize};
#[derive(Deserialize, Debug, Serialize)]
pub struct MyLoad {
    camera: CameraToml,
    shapes: Vec<ShapeToml>,
    material: Vec<MaterialToml>, // images:Vec<String>,
                                 // material:Vec<String>,
                                 // env:Vec<String>
}
impl MyLoad {
    pub fn new() -> Self {
        unimplemented!()
    }
    fn load_material(&mut self) {
        unimplemented!()
    }
    fn load_shape(&mut self) {}
    fn load_light(&mut self) {}
    fn load_env(&mut self) {}
}

#[derive(Deserialize, Debug, Serialize)]
pub struct CameraToml {
    mode: String,
    size: Vec2,
    far: f32,
    near: f32,
}
#[derive(Deserialize, Debug, Serialize)]

pub struct TransformToml {
    r: Vec4,
    s: Vec3,
    t: Vec3,
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum ShapeToml {
    Rect { trans: TransformToml },
    Shpere { trans: TransformToml, r: f32 },
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum MaterialToml {
    Matte { kd: TextureToml,sigma_angle:TextureToml},
    Plastic{kd:TextureToml,ks:TextureToml,roughness:TextureToml},
    Fourier{}

}
#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum TextureToml {
    Image { value: String },
    Constant { value: DVec3 },
    // Value{value:f32}
}
pub enum LightToml {
    Point {
        trans: TransformToml,
        point: Vec3,
        lemit: Vec3,
    },
    Spot {
        trans: TransformToml,
        point:  Vec3,
        lemit:  Vec3,
        end_angle:f32,
        start_angle:f32
    },
    Texture{
        lemit:Vec3,
        texture:TextureToml
    },
    Distant{
        lemit:Vec3,
        dir:Vec3,
        world_center:Vec3,
        world_radius:Vec3,
    },
    Area {
        trans: TransformToml,
        lemit: Vec3,
        shape_index: usize,
    },
    Infinite{
        skybox:TextureToml,
        lemit:Vec3,
        world_center:Vec3,
        world_radius:Vec3,
    }
}
