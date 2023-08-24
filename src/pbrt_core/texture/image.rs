use glam::u32::UVec2;
use gltf::image::Data;

pub struct MipMap{
    size:UVec2,
    mapping:Vec<Data>
}