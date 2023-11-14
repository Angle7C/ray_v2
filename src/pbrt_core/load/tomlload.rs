use std::{ops::Add, path::Path, sync::Arc};

use anyhow::Result;
use glam::{Mat4, UVec3, Vec3, Vec2, Vec4, Quat};
use obj::Object;
use serde::{Deserialize, Serialize};

use crate::pbrt_core::{
    self,
    camera::Camera,
    light::{area::DiffuseAreaLight, inf::InfiniteLight, point::Point, Light},
    material::{
        self, matte::Matte, metal::MetalMaterial, mirror::Mirror, plastic::Plastic, Material,
    },
    primitive::{
        self,
        mesh::Mesh,
        shape::{self, rectangle::Rectangle, shpere::Shpere, triangle::Triangle, Shape},
        Primitive,
    },
    texture::{constant::ConstantTexture, image::ImageTexture, Texture},
    tool::{
        mipmap::{ImageData, MipMap},
        sence::Sence,
    }, integrator::direct::LightStartegy,
};

use super::{
    objload::ObjLoad, gltfload::GltfLoad,
};
static mut SHAPE: Vec<Shape> = vec![];
#[derive(Deserialize, Debug, Serialize, Default)]
struct ObjToml {
    pub transform: TransformToml,
    pub objtype: String,
    pub path: String,
    pub material_index: usize,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct TomlLoader {
    object: Vec<ObjToml>,
    material: Vec<MaterialToml>,
    texture: Vec<TextureToml>,
    #[serde(default)]
    light: Vec<LightToml>,
    #[serde(default)]
    shapes: Vec<ShapeToml>,
}
impl TomlLoader {
    pub fn load_sence(self, camera: Camera) -> Sence {
        let textures = Self::load_texture(self.texture).unwrap().leak();
        let materials = Self::load_material(self.material, textures).unwrap().leak();
        let primitive = Self::load_object(self.object, materials).unwrap();
        Self::load_shape(self.shapes);
        let light = Self::load_light(self.light, unsafe { &SHAPE }, textures);
        Sence::new(primitive, camera, light)
    }
    fn load_texture(textures: Vec<TextureToml>) -> Result<Vec<Arc<dyn Texture>>> {
        let mut vec = vec![];
        for texture in textures {
            let box_texture: Arc<dyn Texture> = match texture {
                TextureToml::Image { path } => {
                    let path = Path::new("image").join(path);
                    let image = image::io::Reader::open(path)?.decode()?;
                    let image_data = ImageData::new_dynimage(image);
                    let mipmap = MipMap::new(image_data);
                    Arc::new(ImageTexture::new(mipmap))
                }
                TextureToml::Constant { value } => Arc::new(ConstantTexture::new(value)),
            };
            vec.push(box_texture);
        }
        Ok(vec)
    }
    fn load_material(
        materials: Vec<MaterialToml>,
        texture: &'static [Arc<dyn Texture>],
    ) -> Result<Vec<Box<dyn material::Material>>> {
        let mut vec = vec![];
        for material in materials {
            let a: Box<dyn Material> = match material {
                MaterialToml::Matte { kd, sigma } => {
                    let kd = texture.get(kd).unwrap().clone();
                    // let sigma = texture.get(*sigma);
                    Box::new(Matte::new(kd, sigma))
                }
                MaterialToml::Plastic { kd, ks, roughness } => {
                    let kd = texture.get(kd).unwrap();
                    let ks = texture.get(ks).unwrap();
                    let roughness = texture.get(roughness).unwrap();
                    Box::new(Plastic::new(kd.clone(), ks.clone(), roughness.clone()))
                }
                MaterialToml::Mirror { kr } => {
                    let kr = texture.get(kr).unwrap();
                    Box::new(Mirror::new(kr.clone()))
                }
                MaterialToml::Metal { eta, k, roughness } => {
                    let eta = texture.get(eta).unwrap();
                    let k = texture.get(k).unwrap();
                    let roughness = texture.get(roughness).unwrap();
                    Box::new(MetalMaterial::new(
                        eta.clone(),
                        k.clone(),
                        roughness.clone(),
                        false,
                    ))
                }
                _ => todo!(),
            };
            vec.push(a)
        }
        Ok(vec)
    }
    fn load_object(
        objects: Vec<ObjToml>,
        materials: &'static mut [Box<dyn material::Material>],
    ) -> Result<Vec<Box<dyn Primitive>>> {
        let mut vec = vec![];
        let mut all_mesh = Default::default();
        let mut primitives = vec![];
        //先获取对应obj的mesh，index。并存储最后进行合并计算
        for object in objects.iter() {
            let sub_vec = Self::load_sigle_object(&object, &mut all_mesh)?;
            vec.push(sub_vec);
        }
        let mesh = Arc::new(all_mesh);
        for index in 0..vec.len() {
            let obj = objects.get(index).unwrap();
            let mat4 = obj.transform.get_mat();
            let material = materials.get(obj.material_index);
            let sub_primitive = vec.get(index).unwrap();
            let len = sub_primitive[0].len();
            for i in 0..len {
                let pos = sub_primitive[0][i];
                let uv = sub_primitive[1][i];
                let normal = sub_primitive[2][i];
                let t: Box<dyn Primitive> =
                    Box::new(Triangle::new(pos, normal, uv, mesh.clone(), mat4, material));
                primitives.push(t);
            }
        }
        Ok(primitives)
    }
    fn load_sigle_object(object: &ObjToml, all_mesh: &mut Mesh) -> Result<Vec<Vec<UVec3>>> {
        let object_path = &object.path;
        let objtype = &object.objtype;
        let (mut mesh, vec) = match objtype.as_str() {
            "obj" => ObjLoad::load(&object_path),
            "gltf" => GltfLoad::load(&object_path),
            _=>unimplemented!("obj type not support")
        }?;
        let mut ans_index = vec![];
        for (index, item) in vec.iter().enumerate() {
            let size = get_type_index(index, &all_mesh);
            let item = item
                .iter()
                .map(|v| v.add(UVec3::splat(size as u32)))
                .collect::<Vec<_>>();
            ans_index.push(item);
        }
        all_mesh.merge(&mut mesh);
        Ok(ans_index)
    }

    fn load_shape(shapes: Vec<ShapeToml>) {
        // let mut vec = vec![];
        for item in &shapes {
            let shape: Shape = match item {
                ShapeToml::Rect {
                    trans,
                    material_index: _,
                } => Shape::Rect(Rectangle::new(trans.get_mat(), None)),

                ShapeToml::Shpere {
                    trans,
                    r,
                    material_index: _,
                } => pbrt_core::primitive::shape::Shape::Shpere(Shpere::new(
                    *r,
                    None,
                    trans.get_mat(),
                )),
            };
            unsafe { SHAPE.push(shape) }
        }
    }
    fn load_light(
        lights: Vec<LightToml>,
        shape: &'static [Shape<'static>],
        texture: &'static [Arc<dyn Texture>],
    ) -> Vec<Light> {
        let mut vec = vec![];
        for (index, item) in lights.iter().enumerate() {
            let light: Light = match item {
                LightToml::Point {
                    // trans,
                    point,
                    lemit,
                } => Light::PointLight(Box::new(Point::new(*lemit, *point, index))),
                LightToml::Area { lemit, shape_index } => Light::AreaLight(Box::new(
                    DiffuseAreaLight::new(*lemit, shape.get(*shape_index).take().unwrap(), index),
                )),
                LightToml::Infinite {
                    world_center,
                    world_radius,
                    skybox,
                } => Light::Infinite(Box::new(InfiniteLight::new(
                    *world_radius,
                    *world_center,
                    texture.get(*skybox).unwrap().clone(),
                    Mat4::default(),
                    Vec3::ONE,
                    index,
                ))),
                _ => todo!(),
            };
            vec.push(light)
        }
        vec
    }
}
#[inline]
fn get_type_index(index: usize, mesh: &Mesh) -> usize {
    match index {
        0 => mesh.pos_size(),
        1 => mesh.norm_size(),
        2 => mesh.uv_size(),
        _ => panic!(),
    }
}


#[derive(Deserialize, Debug, Serialize, Default)]
pub struct MyLoad {
    camera: CameraToml,
    primitive: Vec<ShapeToml>,
    #[serde(default)]
    shapes: Vec<ShapeToml>,
    material: Vec<MaterialToml>,
    #[serde(default)]
    light: Vec<LightToml>,
    // env_light: Vec<LightToml>,
    texture: Vec<TextureToml>,
    pub intergator: IntegratorToml,
    name: String,
}



#[derive(Deserialize, Debug, Serialize, Default)]
pub struct CameraToml {
    pub mode: String,
    pub size: Vec2,
    pub far: f32,
    pub near: f32,
    pub eye: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub fov: f32,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct TransformToml {
    r: Vec4,
    s: Vec3,
    t: Vec3,
}

impl TransformToml {
    pub fn get_mat(&self) -> Mat4 {
        let angle = self.r.w.to_radians();
        let quat = Quat::from_axis_angle(self.r.truncate(), angle);
        Mat4::from_scale_rotation_translation(self.s, quat, self.t)
    }
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum ShapeToml {
    Rect {
        trans: TransformToml,
        material_index: usize,
    },
    Shpere {
        trans: TransformToml,
        r: f32,
        material_index: usize,
    },
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum MaterialToml {
    Matte {
        kd: usize,
        sigma: f32,
    },
    Plastic {
        kd: usize,
        ks: usize,
        roughness: usize,
    },
    Mirror {
        kr: usize,
    },
    Metal {
        eta: usize,
        k: usize,
        roughness: usize,
    },
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum TextureToml {
    Image { path: String },
    Constant { value: Vec3 },
    // Value{value:f32}
}

#[derive(Deserialize, Debug, Serialize)]
#[serde(tag = "mode")]
pub enum LightToml {
    Point {
        point: Vec3,
        lemit: Vec3,
    },
    Spot {
        trans: TransformToml,
        point: Vec3,
        lemit: Vec3,
        end_angle: f32,
        start_angle: f32,
    },
    Texture {
        lemit: Vec3,
        texture: TextureToml,
    },
    Distant {
        lemit: Vec3,
        dir: Vec3,
        world_center: Vec3,
        world_radius: Vec3,
    },
    Area {
        lemit: Vec3,
        shape_index: usize,
    },
    Infinite {
        skybox: usize,
        world_center: Vec3,
        world_radius: f32,
    },
}

#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[serde(tag = "mode")]
pub enum IntegratorToml {
    Path {
        core_num: usize,
        sample_num: usize,
        q: f32,
        max_depth: usize,
    },
    Direct {
        core_num: usize,
        sample_num: usize,
        startegy: LightStartegy,
    },
}

impl Default for IntegratorToml {
    fn default() -> Self {
        Self::Path {
            core_num: 8,
            sample_num: 1,
            q: 0.9,
            max_depth: 5,
        }
    }
}

