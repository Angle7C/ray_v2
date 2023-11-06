use std::sync::atomic::AtomicU32;

use glam::UVec2;
use serde::de;

/// 图片抽象
pub struct Film {
    size: (u32, u32),
    max_index: (u32, u32),
    atom_count: AtomicU32,
}
unsafe impl Sync for Film {}
unsafe impl Send for Film {}
impl Film {
    pub const BLOCK_SIZE: UVec2 = UVec2 { x: 16, y: 16 };
    pub fn block_size()->u32{
        Self::BLOCK_SIZE.x * Self::BLOCK_SIZE.y
    }
    pub fn render_size(&self) -> u32 {
        self.size() / Self::BLOCK_SIZE.x * Self::BLOCK_SIZE.y
    }
    pub fn size(&self) -> u32 {
        self.size.0 * self.size.1
    }
    pub fn new(size: UVec2) -> Self {
        assert!(size.x % Self::BLOCK_SIZE.x == 0);
        assert!(size.y % Self::BLOCK_SIZE.y == 0);

        let (x_size, y_size) = (size.x, size.y);
        let index_x = x_size / Self::BLOCK_SIZE.x;
        let index_y = y_size / Self::BLOCK_SIZE.y;
        Self {
            size: (x_size, y_size),
            max_index: (index_x, index_y),
            atom_count: AtomicU32::new(0),
        }
    }
    pub fn iter(&self) -> Option<FilmIter> {
        let index = self
            .atom_count
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        // index=x*size.x+size.y;
        //[64*64] -> [0,0,0],[0,1,1],[0,2,3],[0,3,4],[1,0,5],[1,1,6],[1,2,7][1,3,8];
        if index > self.max_index.0 * self.max_index.1 {
            return None;
        }
        let x_index = index / self.max_index.0;
        let y_index = index % self.max_index.0;
        let left_up = (x_index * (Self::BLOCK_SIZE.x), y_index * (Self::BLOCK_SIZE.y));
        let right_down = (
            (x_index+1) * (Self::BLOCK_SIZE.x) ,
            (y_index+1) * (Self::BLOCK_SIZE.y) ,
        );
        if left_up.0 >= self.size.0 {
            return None;
        }
        if left_up.1 >= self.size.1 {
            return None;
        }
        Some(FilmIter::new(left_up, right_down, (x_index, y_index)))
    }
}
pub struct FilmIter {
    // pub block_size: (u32, u32),
    pub index: (u32, u32),
    pub left_up: (u32, u32),
    pub right_down: (u32, u32),
    pub now: (u32, u32),
}

impl FilmIter {
    pub fn new(
        left_up: (u32, u32),
        right_down: (u32, u32),
        // block_size: (u32, u32),
        index: (u32, u32),
    ) -> Self {
        Self {
            left_up,
            index,
            right_down,
            // block_size,
            now: left_up,
        }
    }
}
impl Iterator for FilmIter {
    type Item = (f32, f32);
    fn next(&mut self) -> Option<Self::Item> {
        let (mut x, mut y) = self.now;
        if y >= self.right_down.1 {
            y = self.left_up.1;
            x += 1;
        };
        if x >= self.right_down.0 {
            None
        } else {
            self.now = (x, y + 1);
            Some((x as f32, y as f32))
        }
    }
}
