#[cfg(test)]
pub mod test {
    use std::path::Path;

    use glam::Mat4;

    use crate::pbrt_core::load::objload::ObjLoad;
    



    #[test]
    fn test_mipmap() {
        let path = Path::new("image").join("kloofendal_43d_clear_puresky_1k.hdr");
        let image = image::io::Reader::open(path);
        assert!(image.is_ok());
        let image=image.expect("").decode().expect("msg");
        // MipMap::new(ImageData::new_dynimage(image));
        image.save_with_format("skybox.png", image::ImageFormat::Png);
        
    }
    #[test]
    fn obj_load(){
        let path=Path::new("./object/box.obj");
        ObjLoad::load(path.display().to_string().as_str(), Mat4::IDENTITY,None);
    }
}
