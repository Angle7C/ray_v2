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

    mapping: HashMap<Level, Vec<Pixel>>,
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
    pixels: Vec<Pixel>,
}
impl ImageData {
    pub fn new_dynimage(image: DynamicImage) -> Self {
        let mut image_data = ImageData::default();
        let mut pixels = vec![];
        match image {
            DynamicImage::ImageRgb8(image) => {
                image_data.width = image.width();
                image_data.height = image.height();
                for pixel in image.chunks(3).into_iter() {
                    pixels.push(Pixel::from_sclie(pixel))
                }
            }
            DynamicImage::ImageRgba8(_image) => {}
            _ => todo!(),
        }
        image_data.pixels = pixels;
        image_data
    }
    pub fn new(data: &Data) -> Self {
        let mut vec = vec![];
        match data.format {
            gltf::image::Format::R8G8B8A8 => {
                for item in data.pixels.chunks(4) {
                    let pixel = Pixel::from_sclie(item);
                    vec.push(pixel);
                }
            }
            gltf::image::Format::R8G8B8 => {
                for item in data.pixels.chunks(3) {
                    let pixel = Pixel::from_sclie(item);
                    vec.push(pixel);
                }
            }
            _ => unimplemented!("尚未实现该ImageFormat格式"),
        };
        Self {
            width: data.width,
            height: data.height,
            pixels: vec,
        }
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
        let _w_level = f32::log2(image_data.width as f32).floor() as usize;
        let _h_level = f32::log2(image_data.height as f32).floor() as usize;
        let mut mipmap = MipMap::default();
        //分辨率
        mipmap.resolution = UVec2::new(image_data.width as u32, image_data.height as u32);
        //多级纹理
        let mut data: HashMap<Level, Vec<Pixel>> = HashMap::new();
        let _w = image_data.width;
        let _h = image_data.height;
        data.insert(Level { x: 0, y: 0 }, image_data.pixels);
        //生成多级纹理
        // (0,0)->(0,1)->(1,0)->(1,1)
        // for i in 0..w_level {
        //     let last = Level { x: i, y: i };
        //     for j in i + 1..h_level {
        //         //层数
        //         let level = Level { x: i, y: j };
        //         data.insert(
        //             level,
        //             //依据上一层生成下一层和左右两边不规则层数。
        //             Self::build_floor(data.get(&last).unwrap(), w >> i, h >> j),
        //         );
        //     }
        //     for k in i + 1..w_level {
        //         let level = Level { x: k, y: i };
        //         data.insert(
        //             level,
        //             //依据上一层生成下一层和左右两边不规则层数。
        //             Self::build_floor(data.get(&last).unwrap(), w >> i, h >> k),
        //         );
        //     }
        // for i in 1..w_level {
        //     let last = Level { x: i - 1, y: i - 1 };
        //     // for j in 1..h_level {
        //     //层数
        //     let level = Level { x: i, y: i };
        //     data.insert(
        //         level,
        //         //依据上一层生成下一层和左右两边不规则层数。
        //         Self::build_floor(data.get(&last).unwrap(), w >> i, h >> i),
        //     );
        //     // }
        // }
        mipmap.mapping = data;
        mipmap
    }
    pub fn lookup(&self, uv: Vec2, _duvdx: Vec2, _duvdy: Vec2) -> Vec3 {
        // let x_level = duvdx.x.max(duvdy.x).sqrt().log2().floor() as usize;
        // let y_level = duvdx.x.max(duvdy.x).sqrt().log2().floor() as usize;
        let level = Level { x: 0, y: 0 };
        let pixel = self.mapping.get(&level).expect("获取MipMap失败");
        let len = (uv.x * self.resolution.x as f32) as u32 * self.resolution.y
            + (uv.y * self.resolution.y as f32) as u32;
        let pixel = pixel.get(len as usize).expect(
            format!(
                "pixel message: len:{},w: {} h: {},uv:{}",
                len, self.resolution.x, self.resolution.y, uv
            )
            .as_str(),
        );
        Vec4::from(*pixel).truncate()
        // return Vec3::X;
        // let default = vec![];
        // let pixel = self.mapping.get(&level).unwrap_or_else(|| {
        //     error!("mipmap读取错误");
        //     &default
        // });
        // if pixel.is_empty() {
        //     Vec3::ZERO
        // } else {
        //     let len = uv.x * (2 << x_level) as f32 * uv.y * (2 << y_level) as f32;
        //     let pixel = pixel.get(len as usize).unwrap();
        //     let value = Vec4::from(*pixel);
        //     value.truncate()
        // }
    }
    #[allow(dead_code)]
    fn build_floor(data: &Vec<Pixel>, w: u32, h: u32) -> Vec<Pixel> {
        let len = (w * h) as usize;
        let mut pixel: Vec<Pixel> = Vec::with_capacity(len);
        for i in 0..w {
            for j in 0..h {
                let left_up = (i * 2) + (j * 2) * h;
                let right_up = left_up + 1;
                let left_bottom = ((i + 1) * 2) + (j * 2) * h;
                let right_bottom = left_bottom + 1;
                let (a, b, c, d) = (
                    data.get(left_up as usize),
                    data.get(right_up as usize),
                    data.get(left_bottom as usize),
                    data.get(right_bottom as usize),
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
        }
        pixel
    }
}
