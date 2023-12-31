use std::sync::Arc;

use glam::{u32::UVec3, Mat4, Quat, UVec4, Vec2, Vec3, Vec4};
use gltf::{buffer::Data, import, Buffer};

use crate::pbrt_core::primitive::mesh::Mesh;

use super::super::{
    material::{matte::Matte, pbr::PbrMaterial, Material},
    texture::{constant::ConstantTexture, image::ImageTexture, Texture},
    tool::mipmap::{ImageData, MipMap},
};

pub struct GltfLoad;
impl GltfLoad {
    pub fn load<'a, 'b>(path: &str) -> anyhow::Result<(Mesh, Vec<Vec<UVec3>>)>
    where
        'b: 'a,
    {
        if let Ok((gltf, buffer, _)) = import(path) {
            let mut pos_index = vec![];
            let mut norm_index = vec![];
            let mut uv_index = vec![];
            //加载材质
            // material = &*load_material(images, &gltf).leak();
            //加载shape
            let (all_point, all_normal, all_uv, index_vec, nodes, transform_vec, det_index_vec) =
                load_node(&gltf, buffer);
            let mesh = Mesh::new(all_point, all_normal, all_uv, vec![]);
            // let mesh = Arc::new(mesh);
            {
                for i in 0..nodes {
                    let index = index_vec.get(i).unwrap();
                    let det_index = det_index_vec[i];
                    // let transform = transform_vec.get(i).unwrap();
                    for i in index {
                        // let w = i.w as usize;
                        // let a = material.get(w);
                        pos_index.push(*i + det_index);
                        norm_index.push(*i + det_index);
                        uv_index.push(*i + det_index);
                    }
                }
            };
            return Ok((mesh, vec![pos_index, norm_index, uv_index]));
        }
        unimplemented!()
    }
}

fn load_mesh(
    mesh: gltf::Mesh,
    // material_vec: &[Box<dyn Material>],
    buffer: &Vec<Data>,
    index: &mut Vec<UVec3>,
    point: &mut Vec<Vec3>,
    normal: &mut Vec<Vec3>,
    uv: &mut Vec<Vec2>,
) {
    let get_buffer = |x: Buffer| Some(&*buffer[x.index()].0);
    for primitive in mesh.primitives() {
        let material = primitive.material();
        // let (_, material_index) = if let Some(material_index) = material.index() {
        //     (material_vec.get(material_index).unwrap(), material_index)
        // } else {
        //     //默认材质
        //     (material_vec.last().unwrap(), material_vec.len() - 1)
        // };
        let reader = primitive.reader(get_buffer);
        *index = reader
            .read_indices()
            .unwrap()
            .into_u32()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(UVec3::from_slice)
            // .map(|x| x.extend(material_index as u32))
            .collect();

        for (s, _) in primitive.attributes() {
            match s {
                gltf::Semantic::Positions => {
                    *point = reader
                        .read_positions()
                        .unwrap()
                        .map(Vec3::from_array)
                        .collect::<Vec<_>>();
                }
                gltf::Semantic::Normals => {
                    *normal = reader
                        .read_normals()
                        .unwrap()
                        .map(Vec3::from_array)
                        .collect::<Vec<_>>();
                }
                gltf::Semantic::Tangents => {}
                gltf::Semantic::Colors(_) => {}
                gltf::Semantic::TexCoords(coords) => {
                    *uv = reader
                        .read_tex_coords(coords)
                        .unwrap()
                        .into_f32()
                        .map(Vec2::from_array)
                        .collect::<Vec<_>>();
                }
                gltf::Semantic::Joints(_) => {}
                gltf::Semantic::Weights(_) => {}
            }
        }
    }
}

fn load_material<'a, 'b>(
    images: Vec<gltf::image::Data>,
    gltf: &'b gltf::Document,
) -> Vec<Box<dyn Material + 'a>>
where
    'a: 'b,
{
    let mut mip_vec: Vec<_> = vec![];
    let mut material_vec = vec![];
    for image in gltf.images() {
        mip_vec.push(MipMap::new(ImageData::new(&images[image.index()])));
    }
    for material in gltf.materials() {
        add_material(&material, &mip_vec, &mut material_vec);
    }
    material_vec.push(Box::new(Matte::new(
        Arc::new(ConstantTexture::new(Vec3::splat(0.75))),
        0.0,
    )));
    material_vec
}
fn load_node(
    // material_vec: &[Box<dyn Material>],
    gltf: &gltf::Document,
    buffer: Vec<gltf::buffer::Data>,
) -> (
    Vec<Vec3>,
    Vec<Vec3>,
    Vec<Vec2>,
    Vec<Vec<UVec3>>,
    usize,
    Vec<Mat4>,
    Vec<UVec3>,
) {
    let mut transform_vec = vec![];
    let mut index_vec = vec![];
    let mut point_vec = vec![];
    let mut normal_vec = vec![];
    let mut uv_vec = vec![];
    let mut nodes: usize = 0;
    for (_, item) in gltf.nodes().enumerate() {
        let transform = match item.transform() {
            gltf::scene::Transform::Matrix { matrix } => Mat4::from_cols_array_2d(&matrix),
            gltf::scene::Transform::Decomposed {
                translation,
                rotation,
                scale,
            } => Mat4::from_scale_rotation_translation(
                Vec3::from_array(scale),
                Quat::from_array(rotation),
                Vec3::from(translation),
            ),
        };

        transform_vec.push(transform);
        let mut point = vec![];
        let mut normal = vec![];
        let mut uv = vec![];
        let mut index = vec![];
        if let Some(mesh) = item.mesh() {
            load_mesh(
                mesh,
                // material_vec,
                &buffer,
                &mut index,
                &mut point,
                &mut normal,
                &mut uv,
            );
        };
        index_vec.push(index);
        normal_vec.push(normal);
        uv_vec.push(uv);
        point_vec.push(point);
        nodes += 1;
    }
    let mut all_point = vec![];
    let mut all_normal = vec![];
    let mut all_uv = vec![];
    let mut det_point = vec![UVec3::ZERO];
    for i in 0..nodes {
        let point = point_vec.get_mut(i).unwrap();
        let normal = normal_vec.get_mut(i).unwrap();
        let uv = uv_vec.get_mut(i).unwrap();
        det_point.push(det_point[i] + UVec3::splat(point.len() as u32));
        all_point.append(point);
        all_normal.append(normal);
        all_uv.append(uv);
    }
    (
        all_point,
        all_normal,
        all_uv,
        index_vec,
        nodes,
        transform_vec,
        det_point,
    )
}
pub fn add_material(
    material: &gltf::Material,
    mip_map: &Vec<MipMap>,
    material_vec: &mut Vec<Box<dyn Material>>,
) {
    if material_vec.get(material.index().unwrap()).is_some() {
        return;
    }
    //透明度，不做处理
    match material.alpha_mode() {
        _ => (),
    }
    let _ = material.alpha_cutoff();
    //双面贴图
    let _ = material.double_sided();
    //自发光
    let _emissive_texture = material.emissive_factor();
    //自发光贴图
    material.emissive_texture();
    //法线贴图
    material.normal_texture();
    //遮挡贴图
    material.occlusion_texture();
    //pbr材质
    let pbr = material.pbr_metallic_roughness();
    //base_color
    let base_color: Arc<dyn Texture> = if let Some(base_color_texture) = pbr.base_color_texture() {
        let i = base_color_texture.texture().index();
        Arc::new(ImageTexture::new(mip_map.get(i).unwrap().to_owned()))
    } else {
        Arc::new(ConstantTexture::new(
            Vec4::from_array(pbr.base_color_factor()).truncate(),
        ))
    };
    //金属度
    let metailc: Arc<dyn Texture> =
        if let Some(metallic_roughness_texture) = pbr.metallic_roughness_texture() {
            Arc::new(ImageTexture::new(
                mip_map
                    .get(metallic_roughness_texture.texture().index())
                    .unwrap()
                    .to_owned(),
            ))
        } else {
            Arc::new(ConstantTexture::new(Vec3::splat(pbr.metallic_factor())))
        };
    //粗糙度
    let roughness: Arc<dyn Texture> =
        if let Some(roughness_texture) = pbr.metallic_roughness_texture() {
            Arc::new(ImageTexture::new(
                mip_map
                    .get(roughness_texture.texture().index())
                    .unwrap()
                    .to_owned(),
            ))
        } else {
            Arc::new(ConstantTexture::new(Vec3::splat(pbr.roughness_factor())))
        };
    let pbr_material = Box::new(PbrMaterial::new(
        Some(base_color),
        Some(metailc),
        Some(roughness),
        None,
        None,
        None,
    ));
    material_vec.insert(material.index().unwrap(), pbr_material);
}
