

use crate::pbrt_core::load::toml_load::toml_meta::primitive_toml::PrimitiveToml;
#[test]
pub fn test_build(){
   let a=PrimitiveToml::GeometricPrimitive{     shape_index:0,
        material_index:Some(1),
        light_index:None};
    let a = toml::to_string(&a).unwrap();
    println!("{}",a)
}