use std::{
    collections::{BTreeMap, HashMap},
    ops::{Add, Deref, DerefMut, Div}, fmt::Debug,
};

use glam::{u32::UVec2, DVec2, DVec3, DVec4};
use gltf::image::Data;
use image::{DynamicImage, ImageBuffer};
use log::error;
#[derive(Default,Clone)]
pub struct MipMap {
    //图像大小
    resolution: UVec2,

    mapping: HashMap<Level, Vec<Pixel>>,
}
impl Debug for MipMap{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}
pub struct ImageData {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}
impl ImageData {
    pub fn new(data: &Data) -> Self {
        let mut vec = vec![];
        match data.format {
            gltf::image::Format::R8G8B8A8 => {
                for item in data.pixels.chunks(4) {
                    let pixel = Pixel::from_sclie(item);
                    vec.push(pixel);
                }
            },
            gltf::image::Format::R8G8B8=>{
                for item in data.pixels.chunks(3) {
                    let pixel = Pixel::from_sclie(item);
                    vec.push(pixel);
                }
            }
           _=>unimplemented!("尚未实现该ImageFormat格式"),
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
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
impl Pixel {
    pub fn from_sclie(array: &[u8]) -> Self {
        Self {
            x: array[0] as f64,
            y: array[1] as f64,
            z: array[2] as f64,
            w: 255.0,
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
impl Div<f64> for Pixel {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}
impl From<DVec4> for Pixel {
    fn from(value: DVec4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}
impl From<Pixel> for DVec4 {
    fn from(value: Pixel) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
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
        let w_level = unsafe { f64::log2(image_data.width as f64).floor() } as usize;
        let h_level = unsafe { f64::log2(image_data.height as f64).floor() } as usize;
        let mut mipmap = MipMap::default();
        //分辨率
        mipmap.resolution = UVec2::new(image_data.width as u32, image_data.height as u32);
        //多级纹理
        let mut data: HashMap<Level, Vec<Pixel>> = HashMap::new();
        let w = image_data.width;
        let h = image_data.height;
        data.insert(Level { x: 0, y: 0 }, image_data.pixels);
        //生成多级纹理
        /// (0,0)->(0,1)->(1,0)->(1,1)
        for i in 0..w_level {
            let last=Level { x: i, y: i };
            for j in i+1..h_level {
                //层数
                let level = Level { x: i, y: i };
                data.insert(
                    level,
                    //依据上一层生成下一层和左右两边不规则层数。
                    Self::build_floor(data.get(&last).unwrap(), w >> i, h >> j),
                );
            }
            for k in i+1..w_level{
                let level=Level{x:k,y:i};
                data.insert(
                    level,
                    //依据上一层生成下一层和左右两边不规则层数。
                    Self::build_floor(data.get(&last).unwrap(), w >> i, h >> k),
                );
            // }
        }
        mipmap.mapping = data;
        mipmap
    }
    fn lookup(&self, uv: DVec2, duvdx: DVec2, duvdy: DVec2) -> DVec3 {
        let x_level = duvdx.x.max(duvdy.x).sqrt().log2().floor() as usize;
        let y_level = duvdx.x.max(duvdy.x).sqrt().log2().floor() as usize;
        let level = Level {
            x: x_level,
            y: y_level,
        };
        let default = vec![];
        let pixel = self.mapping.get(&level).unwrap_or_else(|| {
            error!("mipmap读取错误");
            &default
        });
        if pixel.is_empty() {
            DVec3::ZERO
        } else {
            let len = uv.x * (2 << x_level) as f64 * uv.y * (2 << y_level) as f64;
            let pixel = pixel.get(len as usize).unwrap();
            let value = DVec4::from(*pixel);
            value.truncate()
        }
    }
    fn build_floor(data: &Vec<Pixel>, w: u32, h: u32) -> Vec<Pixel> {
        let len = (w  * h ) as usize;
        let mut pixel: Vec<Pixel> = Vec::with_capacity(len);
        unsafe { pixel.set_len(len) }
        for i in 0..len {
            let left_up = i * 2;
            let right_up = i * 2 + 1;
            let left_bottom = i * 2 + w as usize;
            let right_bottom =left_bottom+1;
            let (a, b, c, d) = (
                data.get(left_up),
                data.get(right_up),
                data.get(left_bottom),
                data.get(right_up),
            );
            match (a, b, c, d) {
                (Some(a), Some(b), Some(c), Some(d)) => {
                    let target = (*a + *b + *c + *d) / 4.0;
                    pixel.insert(i, target);
                }
                (None,None,None,None) => continue,
                _=>unimplemented!("无法获取到指定像素"),
            }
        }
        pixel
    }
}
