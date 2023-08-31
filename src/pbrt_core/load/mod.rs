use std::{sync::Arc, cell::RefCell, collections::{HashMap, HashSet, BTreeSet}};

use glam::{
    f64::{DMat4, DVec2, DVec3},
    u32::UVec3,
    Mat4, Vec2, Vec3, Quat,
};
use gltf::{import, Attribute};

use crate::pbrt_core::primitive::{
    mesh::Mesh,
    shape::{triangle::Triangle, Shape, shpere},
};

use super::primitive::{shape, Primitive};

pub struct GltfLoad;
impl GltfLoad {
    pub fn load(path: &str) -> Vec<Box<dyn Primitive>> {
        let mut meshs = Arc::new(RefCell::new(Mesh::default()));
        if let Ok((gltf, buffer, _images)) = import(path) {
            let mut shape = Vec::<Box<dyn Primitive>>::with_capacity(1000);
            let mut last_set: BTreeSet<u32>=BTreeSet::<u32>::new();
            let mut now_set=BTreeSet::<u32>::new();
            let mut size=0;
            let mut det=UVec3::ZERO;
            for item in gltf.nodes() {
                let transform = match item.transform() {
                    gltf::scene::Transform::Matrix { matrix } => {
                        Mat4::from_cols_array_2d(&matrix).as_dmat4()
                    }
                    gltf::scene::Transform::Decomposed {
                        translation,
                        rotation,
                        scale,
                    } => Mat4::from_scale_rotation_translation(Vec3::from_array(scale), Quat::from_array(rotation),Vec3::from(translation))
                        .as_dmat4(),
                };
                let mut point = vec![];
                let mut normal = vec![];
                let mut uv = vec![];
                let mut index: Vec<UVec3> = vec![];
                
                if let Some(mesh)=item.mesh() {
                    for primitive in mesh.primitives() {
                        let attribute = primitive.attributes();
                        let reader = primitive.reader(|x| Some(&buffer[x.index()].0));
                        index = reader
                            .read_indices()
                            .unwrap()
                            .into_u32()
                            .collect::<Vec<_>>()
                            .chunks(3)
                            .map(|x| UVec3::from_slice(x))
                            .collect();
                        for (s, _) in primitive.attributes() {
                            match s {
                                gltf::Semantic::Positions => {
                                    point = reader
                                        .read_positions()
                                        .unwrap()
                                        .map(|x| Vec3::from_array(x).as_dvec3())
                                        .collect::<Vec<_>>();
                                }
                                gltf::Semantic::Normals => {
                                    normal = reader
                                        .read_normals()
                                        .unwrap()
                                        .map(|x| Vec3::from_array(x).as_dvec3())
                                        .collect::<Vec<_>>();
                                }
                                gltf::Semantic::Tangents => todo!(),
                                gltf::Semantic::Colors(color) => {
                                   
                                }
                                gltf::Semantic::TexCoords(coords) => {
                                    uv = reader
                                        .read_tex_coords(coords)
                                        .unwrap()
                                        .into_f32()
                                        .map(|x| Vec2::from_array(x).as_dvec2())
                                        .collect::<Vec<_>>();
                                }
                                gltf::Semantic::Joints(j) => {},
                                gltf::Semantic::Weights(w) => {},
                            }
                        }
                    }
                };
               
                meshs.borrow_mut().add_message(&mut point,&mut normal,&mut uv,&mut vec![]);
                for i in index {
                    let i=i+det;
                    shape.push(Box::new(Triangle::new(i, meshs.clone(), transform)))
                }
                size=meshs.borrow().point.len() as u32 ;
                det=UVec3::splat(size);
            }
            shape
        }else {
            vec![]
        }
    }
}
