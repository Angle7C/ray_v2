use std::{
    cell::RefCell,
    collections::{BTreeSet, HashMap, HashSet},
    hash::Hash,
    sync::Arc,
};

use glam::{
    f64::{DMat4, DVec2, DVec3},
    u32::UVec3,
    DVec4, Mat4, Quat, UVec4, Vec2, Vec3, Vec4,
};
use gltf::{import, Attribute, Buffer};

use crate::pbrt_core::primitive::{
    mesh::Mesh,
    shape::{shpere, triangle::Triangle, Shape},
};

use super::{
    material::{self, disney::Disney, pbr::PbrMaterial, Material},
    primitive::{shape, Primitive},
    texture::{
        constant::ConstantTexture,
        image::ImageTexture,
        mipmap::{ImageData, MipMap},
        Texture,
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
            let mut material_map = HashMap::<usize, Arc<dyn Material>>::new();
            //mesh几何
            let mut shape = Vec::<Box<dyn Primitive>>::with_capacity(1000);
            //获取指定buffer
            let get_buffer = |x: Buffer| Some(&*buffer[x.index()].0);
            //获取指定image
            let get_image = |x: Buffer| Some(&images[x.index()]);
            let mut transform_map = HashMap::<usize, DMat4>::new();
            let mut index_map = HashMap::<usize, Vec<UVec4>>::new();
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
                let mut index = vec![];
                let mut material_vec: Vec<&Box<dyn Material>> = vec![];
                if let Some(mesh) = item.mesh() {
                    for primitive in mesh.primitives() {
                        let material = primitive.material();
                        //创建材质
                        let m = {
                            let pbr_metallic = material.pbr_metallic_roughness();
                            let i = material.index().unwrap();
                            Self::add_material(&material, &mip_map, &mut material_map);
                            if let Some(material) = material_map.get(&i) {
                                material
                            } else {
                                if let Some(base_color) = pbr_metallic.base_color_texture() {
                                    let base_color =
                                        mip_map.get(&base_color.texture().index()).unwrap();
                                    let material = Arc::new(Disney::new(Some(Box::new(
                                        ImageTexture::new(base_color.to_owned()),
                                    ))));
                                    material_map.insert(i, material);
                                } else {
                                    let base_color = pbr_metallic.base_color_factor();
                                    let constant_texture = ConstantTexture::new(
                                        Vec3::from_slice(&base_color).as_dvec3(),
                                    );
                                    let material =
                                        Arc::new(Disney::new(Some(Box::new(constant_texture))));
                                    material_map.insert(i, material);
                                };
                                material_map.get(&i).unwrap()
                            }
                        };
                        // material.index();
                        // material_vec.push(m);
                        let attribute = primitive.attributes();
                        let reader = primitive.reader(get_buffer);
                        index = reader
                            .read_indices()
                            .unwrap()
                            .into_u32()
                            .collect::<Vec<_>>()
                            .chunks(3)
                            .map(|x| UVec3::from_slice(x))
                            .map(|x| x.extend(material.index().unwrap() as u32))
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
            // let material_slice=&material_map.iter().map(|(x,y)|y).collect::<Vec<_>>();
            let material_vec = material_map.into_iter().map(|(x, y)| y).collect::<Vec<_>>();
            let mesh = Arc::new(Mesh::new(
                all_point,
                all_normal,
                all_uv,
                vec![],
                material_vec.clone(),
            ));
            {
                let mesh_slice = mesh.clone();
                for i in 0..nodes {
                    let index = index_map.get(&i).unwrap();
                    let det_index = det_point[i];
                    let transform = transform_map.get(&i).unwrap();
                    for i in index {
                        let w = i.w as usize;
                        let material = unsafe { material_vec.get_unchecked(w) };
                        shape.push(Box::new(Triangle::new(
                            i.truncate() + det_index,
                            mesh.clone(),
                            *transform,
                            Some(material.clone()),
                        )));
                    }
                }
            }

            shape
        } else {
            vec![]
        }
    }
    pub fn add_material(
        material: &gltf::Material,
        mip_map: &HashMap<usize, MipMap>,
        material_map:&mut HashMap<usize, Arc<dyn Material>>,
    ) {
        if material_map.contains_key(&material.index().unwrap()){
            return;
        }
        //透明度，不做处理
        match material.alpha_mode() {
            _=>(),
        }
        let _ = material.alpha_cutoff();
        //双面贴图
        let _ = material.double_sided();
        //自发光
        let emissive_texture = material.emissive_factor();
        //自发光贴图
        material.emissive_texture();
        //法线贴图
        material.normal_texture();
        //遮挡贴图
        material.occlusion_texture();
        //pbr材质
        let pbr = material.pbr_metallic_roughness();
        //base_color
        let base_color: Arc<dyn Texture<DVec3>> =
            if let Some(base_color_texture) = pbr.base_color_texture() {
                Arc::new(ImageTexture::new(
                    mip_map
                        .get(&base_color_texture.texture().index())
                        .unwrap()
                        .to_owned(),
                ))
            } else {
                Arc::new(ConstantTexture::new(
                    Vec4::from_array(pbr.base_color_factor())
                        .truncate()
                        .as_dvec3(),
                ))
            };
        //金属度
        let metailc: Arc<dyn Texture<DVec3>> =
            if let Some(metallic_roughness_texture) = pbr.metallic_roughness_texture() {
                Arc::new(ImageTexture::new(
                    mip_map
                        .get(&metallic_roughness_texture.texture().index())
                        .unwrap()
                        .to_owned(),
                ))
            } else {
                Arc::new(ConstantTexture::new(
                    Vec3::splat(pbr.metallic_factor()).as_dvec3(),
                ))
            };
        //粗糙度
        let roughness: Arc<dyn Texture<DVec3>> =
            if let Some(metallic_roughness_texture) = pbr.metallic_roughness_texture() {
                Arc::new(ImageTexture::new(
                    mip_map
                        .get(&metallic_roughness_texture.texture().index())
                        .unwrap()
                        .to_owned(),
                ))
            } else {
                Arc::new(ConstantTexture::new(
                    Vec3::splat(pbr.roughness_factor()).as_dvec3(),
                ))
            };
        let pbr_material = Arc::new(PbrMaterial::new(Some(base_color), Some(metailc), Some(roughness), None, None, None));
        material_map.insert(material.index().unwrap(), pbr_material);
    }
}
