#[cfg(test)]
pub mod test {
    use std::path::Path;
    



    #[test]
    fn test_mipmap() {
        let path = Path::new("image").join("kloofendal_43d_clear_puresky_1k.hdr");
        let image = image::io::Reader::open(path);
        assert!(image.is_ok());
        let image=image.expect("").decode().expect("msg");
        // MipMap::new(ImageData::new_dynimage(image));
        image.save_with_format("skybox.png", image::ImageFormat::Png);
        
    }
}
