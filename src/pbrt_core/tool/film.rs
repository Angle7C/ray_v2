use std::sync::atomic::AtomicU32;

use glam::UVec2;

/// 图片抽象
pub struct Film {
    size: (u32, u32),
    atom_count: AtomicU32,
}
unsafe impl Sync for Film {}
unsafe impl Send for Film {}
impl Film {
    pub fn new(size: UVec2) -> Self {
        let (x_size, y_size) = (size.x, size.y);
        Self {
            size: (x_size, y_size),
            atom_count: AtomicU32::new(0),
        }
    }
    pub fn iter(&self) -> Option<FilmIter> {
        let index = self
            .atom_count
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let x_index = index * 2;
        let left_up = (x_index, 0);
        let right_down = (x_index + 2, self.size.1);
        if right_down.0 >= self.size.0 {
            None
        } else {
            Some(FilmIter::new(
                left_up,
                right_down,
                (right_down.0 - left_up.0, right_down.1 - right_down.1),
                index,
            ))
        }
    }
}
pub struct FilmIter {
    pub block_size: (u32, u32),
    pub index: usize,
    pub left_up: (u32, u32),
    pub right_down: (u32, u32),
    pub now: (u32, u32),
}

impl FilmIter {
    pub fn new(
        left_up: (u32, u32),
        right_down: (u32, u32),
        block_size: (u32, u32),
        index: u32,
    ) -> Self {
        Self {
            left_up,
            index: index as usize,
            right_down,
            block_size,
            now: left_up,
        }
    }
    pub fn size(&self) -> u64 {
        let (a, b) = self.block_size;
        (a * b) as u64
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
