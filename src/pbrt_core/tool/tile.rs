use std::{fmt::Debug, path::Path};

use glam::{DVec3, UVec2};
use gltf::json::extensions::texture::Info;
use image::{codecs::hdr::Rgbe8Pixel, ImageBuffer, ImageFormat, Pixel, Rgb, RgbImage};
use log::{info, warn, debug};

use super::color::Color;

///
/// 多线程合并
/// 用于存储渲染数据
pub struct Tile {
    buffer: Vec<Color>,
    index: usize,
}
unsafe impl Send for Tile {}
pub struct Buffer {
    buffer: Vec<Color>,
    width: u32,
    height: u32,
}
impl Buffer {
    pub fn new(size: UVec2) -> Self {
        Self {
            buffer: vec![],
            width: size.x as u32,
            height: size.y as u32,
        }
    }
    pub fn write(self, format: ImageFormat, ssp: f64, name:  impl AsRef<Path>) {
        let mut rbg_buffer = RgbImage::new(self.width, self.height);
        for (index, color) in self.buffer.into_iter().enumerate() {
            let x = index as u32 / self.width;
            let y = index as u32 % self.width;
            rbg_buffer.put_pixel(x, y, Self::to_color(color, ssp))
        }
        rbg_buffer.save_with_format(name, ImageFormat::Jpeg);
    }
    pub fn to_color(color: Color, ssp: f64) -> Rgb<u8> {
        if color.is_nan()||!color.is_finite(){
            info!("color nan:{}",color.is_nan());
            info!("color inf:{}",!color.is_finite());
        }
        let vec = (color / ssp).powf(0.5);
        let rgb = vec * 255.0;
        let color= Rgb([
            rgb.x.clamp(0.0, 255.0) as u8,
            rgb.y.clamp(0.0, 255.0) as u8,
            rgb.z.clamp(0.0, 255.0) as u8,
        ]);
        color
    }
}
impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.index.partial_cmp(&other.index)
    }
}
impl Eq for Tile {}
impl Ord for Tile {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.index.cmp(&other.index)
    }
}
impl Tile {
    pub fn new(index: usize) -> Self {
        Self {
            buffer: vec![],
            index,
        }
    }
    pub fn push(&mut self, color: Color) {
        self.buffer.push(color);
    }
}
pub fn merage_tile(list: Vec<Vec<Tile>>, size: UVec2) -> Buffer {
    let mut list = list
        .into_iter()
        .flat_map(|item| item.into_iter())
        .collect::<Vec<_>>();
    list.sort();
    let mut buffer = Buffer::new(size);
    for (index, tile) in list.iter_mut().enumerate() {
        if (index != tile.index) {
            info!(
                "merage image buffer error index:{}, title_index:{}",
                index, tile.index
            );
        }
        buffer.buffer.append(&mut tile.buffer)
    }
    buffer
}
