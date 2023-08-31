use glam::u32::UVec2;
use gltf::image::Data;

pub struct MipMap{
    //图像大小
    resolution:UVec2,


    mapping:Vec<Pixel>
}
pub struct Pixel{x:f64,y:f64,z:f64,w:f64}
impl MipMap{
    pub fn new(image_data:Data){
        
    }
    fn get_pixel(image_data:Data,x:usize,y:usize)->Pixel{
        unimplemented!()
    }   
}