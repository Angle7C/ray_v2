use glam::Vec3A;

#[derive(Debug,Default,Clone, Copy)]
pub struct Ray {
    pub o: Vec3A,
    pub dir: Vec3A,
    pub t_min: f32,
    pub t_max: f32,
}
impl Ray {
    #[inline]
    pub fn new(o: Vec3A, dir: Vec3A, t_min: f32, t_max: f32) -> Self {
        Ray {
            o,
            dir: dir.normalize(),
            t_min,
            t_max,
        }
    }
    #[inline]
    pub fn new_default(o: Vec3A, dir: Vec3A) -> Self {
        Self {
            o,
            dir,
            t_min: 0.0,
            t_max: f32::INFINITY,
        }
    }
    #[inline]

    pub fn at(&self, t: Vec3A) -> Vec3A {
        self.o + self.dir * t
    }
}
impl From<bvh::ray::Ray> for Ray {
    fn from(value: bvh::ray::Ray) -> Self {
        Self {
            o: value.origin.into(),
            dir: value.direction.into(),
            t_min: 0.0,
            t_max: f32::INFINITY,
        }
    }
}
impl From<Ray> for bvh::ray::Ray {
    fn from(value: Ray) -> Self {
        Self::new(value.o.into(), value.dir.into())
    }
}
impl From<&Ray> for bvh::ray::Ray {
    fn from(value: &Ray) -> Self {
        Self::new(value.o.into(), value.dir.into())
    }
}
