
use lazy_static::lazy_static;
use pbrt_core::tool::content::{Content, self};

pub mod pbrt_core;



fn main(){
    let content = Content::new("./file/setting.toml");
    content.render();
    
}