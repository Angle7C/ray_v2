use crate::pbrt_core::tool::sence::Sence;

use super::SenceLoad;

pub struct MyLoad {}
impl MyLoad {
    pub fn new() -> Self {
        unimplemented!()
    }
    fn load_material(&mut self) {
        unimplemented!()
    }
    fn load_shape(&mut self){

    }
    fn load_light(&mut self){

    }
    fn load_env(&mut self) {
        
    }
}
impl SenceLoad for MyLoad {
    fn load<'a>(setting: crate::pbrt_core::tool::setting::Setting) -> Sence<'a> {
        unimplemented!()
    }
}
pub struct TextureMessage{
    //Texture材质模型
    pub mode:String,
    //采样模型
    pub mode_sampler:String,
    //图片路径
    pub image_path:String,
}