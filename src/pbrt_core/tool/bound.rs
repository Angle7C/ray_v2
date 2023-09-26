use bvh::aabb::AABB;
use glam::{Vec2, Vec3A};

use super::ray::Ray;

#[derive(Debug, Clone, Copy)]
pub struct Bound<const N: usize> {
    pub min: Vec3A,
    pub max: Vec3A,
}

impl From<Bound<3>> for AABB {
    fn from(value: Bound<3>) -> Self {
        let min = glam::Vec3::from(value.min);
        let max = glam::Vec3::from(value.max);
        Self { min, max }
    }
}

impl Bound<2> {
    pub fn new(min: Vec2, max: Vec2) -> Self {
        Self {
            min: Vec3A::from(min.min(max).extend(0.0)),
            max: Vec3A::from(min.max(max).extend(0.0)),
        }
    }
    pub fn merage(&self, bound: Bound<2>) -> Self {
        let min = self.min.min(bound.min);
        let max = self.max.max(bound.max);
        Self { min, max }
    }
    pub fn center(&self) -> Vec2 {
        let center = (self.min + self.max) / 2.0;
        center.truncate()
    }
}
impl Bound<3> {
    pub fn new(min: Vec3A, max: Vec3A) -> Self {
        Self {
            min: min.min(max),
            max: max.max(min),
        }
    }
    pub fn merage(&self, bound: Bound<3>) -> Self {
        let min = self.min.min(bound.min);
        let max = self.max.max(bound.max);
        Self { min, max }
    }
    pub fn center(&self) -> Vec3A {
        let center = (self.min + self.max) / 2.0;
        center
    }
}

impl<const N: usize> Default for Bound<N> {
    fn default() -> Self {
        Self {
            min: Vec3A::splat(f32::INFINITY),
            max: Vec3A::splat(-f32::INFINITY),
        }
    }
}
impl<const N: usize> Bound<N> {
    pub fn intesect(&self, ray: &Ray) -> bool {
        let inv = ray.dir.recip();
        let t1 = (self.min - ray.o) * inv;
        let t2 = (self.max - ray.o) * inv;
        let a = t1.min(t2);
        let b = t1.max(t2);
        let t_entry = a.max_element();
        let t_exit = b.min_element();
        t_entry <= t_exit && t_exit > 0.0
    }
}
