

use std::sync::Arc;

use glam::{f64::{DVec2, DVec3, DMat4},u32::UVec3};
use gltf::import;

use crate::pbrt_core::primitive::{shape::triangle::Triangle, mesh::Mesh};

use super::primitive::{Primitive, shape};

pub struct GltfLoad;
impl GltfLoad {
    pub fn load(path: &str) -> Vec<Box<dyn Primitive>> {
        if let Ok((gltf, buffer, _images)) = import(path) {
            let mut point = vec![];
            let mut normal = vec![];
            let mut uv = vec![];
            let mut index: Vec<UVec3> = vec![];
            let mut shape = Vec::<Box<dyn Primitive>>::with_capacity(1000);
            for item in gltf.meshes() {
                for ele in item.primitives() {
                    match ele.mode() {
                        gltf::mesh::Mode::Triangles => {
                            let reader = ele
                                .reader(|buff| Some(&buffer[buff.index()]));
                            index.append(
                                &mut reader
                                    .read_indices()
                                    .unwrap()
                                    .into_u32()
                                    .collect::<Vec<u32>>()
                                    .chunks(3)
                                    .map(|x| UVec3::from_slice(x))
                                    .collect(),
                            );
                            for (semantic, _) in ele.attributes() {
                                match semantic {
                                    gltf::Semantic::Positions => {
                                        point.append(
                                            &mut reader
                                                .read_positions()
                                                .unwrap()
                                                .map(|x| [x[0] as f64, x[1] as f64, x[2] as f64])
                                                .map(|x| DVec3::from_array(x))
                                                .collect::<Vec<_>>(),
                                        );
                                    }
                                    gltf::Semantic::Normals => {
                                        normal.append(
                                            &mut reader
                                                .read_normals()
                                                .unwrap()
                                                .map(|x| [x[0] as f64, x[1] as f64, x[2] as f64])
                                                .map(|x| DVec3::from_array(x))
                                                .collect::<Vec<_>>(),
                                        );
                                    }
                                    gltf::Semantic::Tangents => {}
                                    gltf::Semantic::Colors(_) => todo!(),
                                    gltf::Semantic::TexCoords(v) => {
                                        uv.append(
                                            &mut reader
                                                .read_tex_coords(v)
                                                .unwrap()
                                                .into_f32()
                                                .map(|x| [x[0] as f64, x[1] as f64])
                                                .map(|x| DVec2::from_array(x))
                                                .collect(),
                                        );
                                    }
                                    gltf::Semantic::Joints(_) => todo!(),
                                    gltf::Semantic::Weights(_) => todo!(),
                                }
                            }
                        }
                        _ => (),
                    }

                }
            }
            let mesh=Arc::new(Mesh::new(point, normal, uv));
            for item in index{
                shape.push(Box::new(Triangle::new(item, mesh.clone(), DMat4::IDENTITY)))
            }
            shape
        }else{
            vec![]
        }
    }
}
