use glam::{UVec2, Vec2};
use std::sync::atomic::AtomicU32;

pub struct Film {
    index: AtomicU32,
    size: UVec2,
}
impl Film {
    pub const BLOCK_SIZE: UVec2 = UVec2::new(16, 16);
    pub fn new(size: UVec2) -> Self {
        Self {
            index: AtomicU32::new(0),
            size,
        }
    }
    pub fn iter(&self) -> FilmIter {
        let index = self
            .index
            .fetch_add(1, std::sync::atomic::Ordering::Acquire);
        let x_index = index / (self.size.x / Self::BLOCK_SIZE.x);
        let y_index = index % (self.size.y / Self::BLOCK_SIZE.y);
        let left_x = x_index * Self::BLOCK_SIZE.x;
        let left_y = y_index * Self::BLOCK_SIZE.y;
        let x_index = (index + 1) / (self.size.x / Self::BLOCK_SIZE.x);
        let y_index = (index + 1) % (self.size.y / Self::BLOCK_SIZE.y);
        let mut right_x = x_index * Self::BLOCK_SIZE.x;
        let mut right_y = y_index * Self::BLOCK_SIZE.y;
        if right_x < self.size.x {
            right_x = self.size.x;
        }
        if right_y < self.size.y {
            right_y = self.size.y;
        }
        let left_up = UVec2::new(left_x, left_y).as_vec2();
        let right_bottom = UVec2::new(right_x, right_y).as_vec2();
        FilmIter { left_up, right_bottom, now: left_up }
    }
}
pub struct FilmIter {
    left_up: Vec2,
    right_bottom: Vec2,
    now: Vec2,
}

impl Iterator for FilmIter {
    type Item = Vec2;
    fn next(&mut self) -> Option<Self::Item> {
        let mut x = self.now.x;
        let mut y = self.now.y;
        x += 1.0;
        if x >= self.right_bottom.x {
            x = self.left_up.x;
            y += 1.0;
        }
        if y >= self.right_bottom.y {
            None
        } else {
            self.now = Vec2::new(x, y);
            Some(self.now)
        }
    }
}
