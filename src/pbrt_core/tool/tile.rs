use glam::DVec3;
use gltf::json::extensions::texture::Info;
use image::{ImageBuffer, RgbImage, ImageFormat};
use log::info;

use super::color::Color;

///
/// 多线程合并
/// 用于存储渲染数据
pub struct Tile {
    buffer: Vec<Color>,
    size: (usize, usize),
    index: usize,
}
pub struct Buffer {
    buffer: Vec<Color>,
    width: u32,
    height: u32,
}
impl Buffer {
    pub fn new(size: (usize, usize)) -> Self {
        Self {
            buffer: vec![],
            width: size.0 as u32,
            height: size.1 as u32,
        }
    }
    pub fn write(&self,format:ImageFormat)->RgbImage{
        let rbg_buffer = RgbImage::new(self.width, self.height);
        
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
    pub fn new(index: usize, size: (usize, usize)) -> Self {
        Self {
            buffer: vec![],
            index,
            size,
        }
    }
    pub fn push(&mut self, color: Color) {
        self.buffer.push(color);
    }
}
pub fn merage_tile(mut list: Vec<Tile>, size: (usize, usize)) -> Buffer {
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
