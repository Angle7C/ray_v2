use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
    sync::Arc,
};

use glam::{
    f64::{DMat4, DVec2, DVec3},
    u32::UVec3,
    DVec4, Mat4, Quat, Vec2, Vec3, Vec4,
};
use gltf::{import, Attribute, Buffer};

use crate::pbrt_core::primitive::{
    mesh::Mesh,
    shape::{shpere, triangle::Triangle, Shape},
};

use super::{
    material::{self, disney::Disney, Material},
    primitive::{shape, Primitive},
    texture::{
        constant::ConstantTexture,
        image::ImageTexture,
        mipmap::{ImageData, MipMap},
    },
};

pub struct GltfLoad;
impl GltfLoad {
    pub fn load(path: &str) -> Vec<Box<dyn Primitive>> {
        if let Ok((gltf, buffer, images)) = import(path) {
            let mut mip_map = HashMap::<usize, MipMap>::new();
            // 变换MipMap
            for i in gltf.images() {
                mip_map.insert(i.index(), MipMap::new(ImageData::new(&images[i.index()])));
            }
            //材质映射
            let mut material_map = HashMap::<usize, Box<dyn Material>>::new();
            //mesh几何
            let mut shape = Vec::<Box<dyn Primitive>>::with_capacity(1000);
            //获取指定buffer
            let get_buffer = |x: Buffer| Some(&*buffer[x.index()].0);
            //获取指定image
            let get_image = |x: Buffer| Some(&images[x.index()]);
            let mut transform_map = HashMap::<usize, DMat4>::new();
            let mut index_map = HashMap::<usize, Vec<UVec3>>::new();
            let mut point_map = HashMap::<usize, Vec<DVec3>>::new();
            let mut normal_map = HashMap::<usize, Vec<DVec3>>::new();
            let mut uv_map = HashMap::<usize, Vec<DVec2>>::new();
            let mut nodes: usize = 0;

            for (i, item) in gltf.nodes().enumerate() {
                let transform = match item.transform() {
                    gltf::scene::Transform::Matrix { matrix } => {
                        Mat4::from_cols_array_2d(&matrix).as_dmat4()
                    }
                    gltf::scene::Transform::Decomposed {
                        translation,
                        rotation,
                        scale,
                    } => Mat4::from_scale_rotation_translation(
                        Vec3::from_array(scale),
                        Quat::from_array(rotation),
                        Vec3::from(translation),
                    )
                    .as_dmat4(),
                };

                transform_map.insert(i, transform);
                let mut point = vec![];
                let mut normal = vec![];
                let mut uv = vec![];
                let mut index: Vec<UVec3> = vec![];
                if let Some(mesh) = item.mesh() {
                    for primitive in mesh.primitives() {
                        let material = primitive.material();
                        //创建材质
                        let m ={
                            let pbr_metallic = material.pbr_metallic_roughness();
                            let i = material.index().unwrap();
                             if let Some(material) = material_map.get(&i) {
                                material
                            } else {
                                if let Some(base_color) = pbr_metallic.base_color_texture() {
                                    let base_color =
                                        mip_map.get(&base_color.texture().index()).unwrap();
                                    let material = Box::new(Disney::new(Some(Box::new(
                                        ImageTexture::new(base_color.to_owned()),
                                    ))));
                                    material_map.insert(i, material);
                                } else {
                                    let base_color = pbr_metallic.base_color_factor();
                                    let constant_texture = ConstantTexture::new(
                                        Vec4::from_array(base_color).as_dvec4(),
                                    );
                                    let material =
                                        Box::new(Disney::new(Some(Box::new(constant_texture))));
                                    material_map.insert(i, material);
                                };
                                material_map.get(&i).unwrap()
                            }
                        };
                        let attribute = primitive.attributes();
                        let reader = primitive.reader(get_buffer);
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
                                gltf::Semantic::Tangents => {}
                                gltf::Semantic::Colors(color) => {}
                                gltf::Semantic::TexCoords(coords) => {
                                    uv = reader
                                        .read_tex_coords(coords)
                                        .unwrap()
                                        .into_f32()
                                        .map(|x| Vec2::from_array(x).as_dvec2())
                                        .collect::<Vec<_>>();
                                }
                                gltf::Semantic::Joints(j) => {}
                                gltf::Semantic::Weights(w) => {}
                            }
                        }
                    }
                };
                index_map.insert(i, index);
                normal_map.insert(i, normal);
                uv_map.insert(i, uv);
                point_map.insert(i, point);
                nodes += 1;
            }
            let mut all_point = vec![];
            let mut all_normal = vec![];
            let mut all_uv = vec![];
            let mut det_point = vec![UVec3::ZERO];
            for i in 0..nodes {
                let point = point_map.get_mut(&i).unwrap();
                let normal = normal_map.get_mut(&i).unwrap();
                let uv = uv_map.get_mut(&i).unwrap();
                det_point.push(det_point[i] + UVec3::splat(point.len() as u32));
                all_point.append(point);
                all_normal.append(normal);
                all_uv.append(uv);
            }
            let mesh = Arc::new(Mesh::new(all_point, all_normal, all_uv, vec![]));
            for i in 0..nodes {
                let index = index_map.get(&i).unwrap();
                let det_index = det_point[i];
                let transform = transform_map.get(&i).unwrap();
                for i in index {
                    shape.push(Box::new(Triangle::new(
                        *i + det_index,
                        mesh.clone(),
                        *transform,
                    )));
                }
            }
            shape
        } else {
            vec![]
        }
    }
}
