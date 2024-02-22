use glam::UVec2;
use image::{ImageFormat, Rgb, RgbImage};
use log::info;
use std::path::Path;

use super::{color::Color, film::Film};

///
/// 多线程合并
/// 用于存储渲染数据
pub struct Tile {
    buffer: Vec<Color>,
    index: (u32, u32),
}
unsafe impl Send for Tile {}
pub struct Buffer {
    buffer: Vec<Tile>,
    width: u32,
    height: u32,
}
impl Buffer {
    pub fn new(size: UVec2) -> Self {
        Self {
            buffer: vec![],
            width: size.x,
            height: size.y,
        }
    }
    pub fn write(self, format: ImageFormat, ssp: f32, name:& impl AsRef<Path>) {
        let mut rbg_buffer = RgbImage::new(self.width, self.height);
        for (_, tile) in self.buffer.into_iter().enumerate() {
            let x_start = tile.index.0 * Film::BLOCK_SIZE.x;
            let y_start = tile.index.1 * Film::BLOCK_SIZE.y;
            for i in 0..Film::BLOCK_SIZE.x {
                let x = x_start + i;
                for j in 0..Film::BLOCK_SIZE.y {
                    let y = y_start + j;
                    let index = j + i * Film::BLOCK_SIZE.x;
                    let color = unsafe { tile.buffer.get_unchecked(index as usize) };
                    rbg_buffer.put_pixel(x, y, Self::to_color(*color, ssp))
                }
            }
        }
        let _ = rbg_buffer.save_with_format(name, format);
    }
    pub fn to_color(color: Color, ssp: f32) -> Rgb<u8> {
        let vec = (color / ssp).powf(0.5);
        let rgb = vec * 255.0;
        
        Rgb([
            rgb.x.clamp(0.0, 255.0) as u8,
            rgb.y.clamp(0.0, 255.0) as u8,
            rgb.z.clamp(0.0, 255.0) as u8,
        ])
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
    pub fn new(index: (u32, u32)) -> Self {
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
    buffer.buffer = list;
    buffer
}
