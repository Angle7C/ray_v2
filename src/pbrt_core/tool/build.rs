use std::{fs::File, io::Read};
use glam::UVec2;

use crate::pbrt_core::{integrator::Integrator};

use super::{sence::Scene, setting::Setting};

pub struct Context {
    //场景
    scene: Scene,
    //渲染器
    integrator: Integrator,
    //输出图片名称
    name:String,
    //图片大小
    size:UVec2
}
impl Context {
    //通过工厂创建sence，
    pub fn new( integrator: Integrator,
                name:String,
                size:UVec2,
    scene: Scene) -> Self {
        Self{
            scene,name,integrator,size
        }
    }
    pub fn render(self) {
        #[cfg(not(debug_assertions))]{
            self.integrator     
            .render_process(&self.name, &self.scene, self.size);
        
        }
        #[cfg(debug_assertions)]
        {
            self.integrator     
            .render_process_debug(&self.name,1,&self.scene,self.size);
        }
       
        
    }
}
