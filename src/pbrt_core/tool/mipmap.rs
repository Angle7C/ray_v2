use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, Deref, DerefMut, Div},
};

use glam::{u32::UVec2, Vec2, Vec3, Vec4};
use gltf::image::Data;
use image::DynamicImage;
#[derive(Default, Clone)]
pub struct MipMap {
    //图像大小
    resolution: UVec2,

    mapping: Box<HashMap<Level, Vec<Vec<Pixel>>>>,
}
impl Debug for MipMap {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
#[derive(Default)]
pub struct ImageData {
    width: u32,
    height: u32,
    pixels: Vec<Vec<Pixel>>,
}
impl ImageData {
    pub fn new_dynimage(image: DynamicImage) -> Self {
        let mut image_data = ImageData::default();
        image_data.pixels=vec![];
        image_data.width = image.width();
        image_data.height = image.height();
        match image {
            DynamicImage::ImageRgb8(image) => {
                for i in 0..image_data.width{
                    let mut vec=vec![];
                    for j in 0..image_data.height{
                        let pixel = image.get_pixel(i, j);
                        vec.push(Pixel::new(pixel.0));
                    }
                    image_data.pixels.push(vec);
                }
            }
            DynamicImage::ImageRgba8(image) => {
                for i in 0..image_data.width{
                    let mut vec=vec![];
                    for j in 0..image_data.height{
                        let pixel = image.get_pixel(i, j);
                        vec.push(Pixel::from_sclie(&pixel.0));
                    }
                    image_data.pixels.push(vec);
                }
            }
            DynamicImage::ImageRgb16(image)=>{
               unimplemented!()
            }
            _=> todo!(),
        }
        image_data
    }
    pub fn new(data: &Data) -> Self {
        unimplemented!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Level {
    x: usize,
    y: usize,
}
#[derive(Clone, Copy, Default)]
struct Pixel {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}
impl Pixel {
    pub fn from_sclie(array: &[u8]) -> Self {
        Self {
            x: array[0] as f32 / 255.0,
            y: array[1] as f32 / 255.0,
            z: array[2] as f32 / 255.0,
            w: 1.0,
        }
    }
    pub fn new(arr:[u8;3])->Self{
        Self { x:arr[0] as f32, y: arr[1] as f32, z: arr[2] as f32, w: 255.0 }
    }
}
impl Add for Pixel {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x + rhs.x;
        let y = self.y + rhs.y;
        let z = self.z + rhs.z;
        let w = self.w + rhs.w;
        Self { x, y, z, w }
    }
}
impl Div<f32> for Pixel {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
impl From<Vec4> for Pixel {
    fn from(value: Vec4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}
impl From<Pixel> for Vec4 {
    fn from(value: Pixel) -> Self {
        Self::new(value.x, value.y, value.z, value.w)
    }
}
impl Deref for Pixel {
    type Target = Pixel;
    fn deref(&self) -> &Self::Target {
        self
    }
}
impl DerefMut for Pixel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self
    }
}

impl MipMap {
    pub fn new(image_data: ImageData) -> Self {
        let w_level = f32::log2(image_data.width as f32).ceil() as usize+1;
        let h_level = f32::log2(image_data.height as f32).ceil() as usize+1;
        let level=w_level.min(h_level);
        let mut mipmap = MipMap::default();
        //分辨率
        mipmap.resolution = UVec2::new(image_data.width as u32, image_data.height as u32);
        //多级纹理
        let mut data= Box::new(HashMap::<Level,Vec<Vec<Pixel>>>::new());
        let w = image_data.width;
        let h = image_data.height;
        data.insert(Level { x: 0, y: 0 }, image_data.pixels);
        //生成多级纹理
        // (0,0)->(0,1)->(1,0)->(1,1)
        for i in 1..level{
            let last = Level { x: i-1, y: i-1 };
            let a=data.get(&last).unwrap();
            data.insert(
                Level { x: i, y: i },
                
                //依据上一层生成下一层和左右两边不规则层数。
                Self::build_floor(a, w >> (i), h >> (i)),
            );
        }
        mipmap.mapping = data;
        mipmap
    }
    pub fn lookup(&self, uv: Vec2, _duvdx: Vec2, _duvdy: Vec2) -> Vec3 {
        // let x_level = duvdx.x.max(duvdy.x).sqrt().log2().floor() as usize;
        // let y_level = duvdx.x.max(duvdy.x).sqrt().log2().floor() as usize;
        let level = Level { x: 0, y: 0 };
        let pixel = self.mapping.get(&level).expect("获取MipMap失败");
        let x=uv.x* (self.resolution.x as f32-1.0);
        let y=uv.y* (self.resolution.y as f32-1.0);
        let pixel = pixel.get(x as usize).unwrap().get(y as usize).unwrap();
        Vec4::from(*pixel).truncate()

    }
    #[allow(dead_code)]
    fn build_floor(data: &Vec<Vec<Pixel>>, w: u32, h: u32) -> Vec<Vec<Pixel>> {        
        let mut vec=vec![];
        for i in 0..w {
            let mut pixel: Vec<Pixel> = Vec::new();
            for j in 0..h {
                let (a, b, c, d) = (
                    data.get(i as usize).unwrap().get(j as usize),
                    data.get(i as usize).unwrap().get((j+1) as usize),
                    data.get((i*2) as usize).unwrap().get(j as usize),
                    data.get((i*2) as usize).unwrap().get((j+1) as usize),
                );
                match (a, b, c, d) {
                    (Some(a), Some(b), Some(c), Some(d)) => {
                        let target = (*a + *b + *c + *d) / 4.0;
                        pixel.push(target);
                    }
                    (None, None, None, None) => continue,
                    _ => unimplemented!("无法获取到指定像素"),
                }
            }
            vec.push(pixel)
        }
        vec
    }

}
