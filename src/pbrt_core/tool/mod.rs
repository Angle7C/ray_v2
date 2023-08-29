use std::{ops::Add, sync::atomic::AtomicU32};

use bvh::aabb::AABB;
use glam::{
    f64::{DVec2, DVec3},
    UVec2,
};

use self::sence::Sence;

use super::{primitive::Primitive, material::BSDF, bxdf::TransportMode, light::Light};

pub mod sence;
pub mod setting;
pub mod func;
/// 光线
#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    pub origin: DVec3,
    pub dir: DVec3,
    pub t_min: f64,
    pub t_max: f64,
}
impl Ray {
    pub fn new(origin: DVec3, dir: DVec3) -> Self {
        Self {
            origin,
            dir: dir.normalize(),
            t_max: f64::INFINITY,
            t_min: 0.0,
        }
    }
    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + self.dir * t
    }
    pub fn from_with_t(origin: DVec3, dir: DVec3, t_min: f64, t_max: f64) -> Self {
        Self {
            origin,
            dir: dir.normalize(),
            t_max,
            t_min,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct RayDiff {
    pub o: Ray,
    pub dx: Option<Ray>,
    pub dy: Option<Ray>,
}
pub struct RayDiffHit {
    p: DVec3,
    p_dx: Option<DVec3>,
    p_dy: Option<DVec3>,
}
impl RayDiff {
    pub fn new(o: Ray) -> Self {
        Self {
            o,
            dx: None,
            dy: None,
        }
    }
    pub fn at(&self, t: f64) -> RayDiffHit {
        let o = self.o.at(t);
        let p_dx = if let Some(ref dx) = self.dx {
            Some(dx.at(t))
        } else {
            None
        };
        let p_dy = if let Some(ref dy) = self.dy {
            Some(dy.at(t))
        } else {
            None
        };
        RayDiffHit { p_dx, p_dy, p: o }
    }
}

/// 包围盒

#[derive(Debug, Clone, Copy)]
pub struct Bound<const N: usize> {
    pub min: DVec3,
    pub max: DVec3,
}

impl From<Bound<3>> for AABB {
    fn from(value: Bound<3>) -> Self {
        let min = value.min;
        let max = value.max;
        Self {
            min: min.as_vec3(),
            max: max.as_vec3(),
        }
    }
}

impl Bound<2> {
    pub fn new(min: DVec2, max: DVec2) -> Self {
        Self {
            min: min.min(max).extend(0.0),
            max: min.max(max).extend(0.0),
        }
    }
    pub fn merage(&self, bound: Bound<2>) -> Self {
        let min = self.min.min(bound.min);
        let max = self.max.max(bound.max);
        Self { min, max }
    }
    pub fn center(&self) -> DVec2 {
        let center = (self.min + self.max) / 2.0;
        center.truncate()
    }
}
impl Bound<3> {
    pub fn new(min: DVec3, max: DVec3) -> Self {
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
    pub fn center(&self) -> DVec3 {
        let center = (self.min + self.max) / 2.0;
        center
    }
}

impl<const N: usize> Default for Bound<N> {
    fn default() -> Self {
        Self {
            min: DVec3::splat(f64::INFINITY),
            max: DVec3::splat(-f64::INFINITY),
        }
    }
}
impl<const N: usize> Bound<N> {
    pub fn intesect(&self, ray: &RayDiff) -> bool {
        let inv = ray.o.dir.recip();
        let t1 = (self.min - ray.o.origin) * inv;
        let t2 = (self.max - ray.o.origin) * inv;
        let a = t1.min(t2);
        let b = t1.max(t2);
        let t_entry = a.max_element();
        let t_exit = b.min_element();
        t_entry <= t_exit && t_exit > 0.0
    }
}
impl Add<Bound<2>> for Bound<2> {
    type Output = Bound<2>;
    fn add(self, rhs: Bound<2>) -> Self::Output {
        self.merage(rhs)
    }
}
impl Add<Bound<3>> for Bound<3> {
    type Output = Bound<3>;
    fn add(self, rhs: Bound<3>) -> Self::Output {
        self.merage(rhs)
    }
}

/// 图片抽象
///
pub struct Film {
    x_index: u32,
    y_index: u32,
    size: (u32, u32),
    atom_count: AtomicU32,
}
unsafe impl Sync for Film {}
unsafe impl Send for Film {}
impl Film {
    const BLOCK_SIZE: (u32, u32) = (32, 32);
    pub fn new(size: UVec2) -> Self {
        let (x_size, y_size) = (size.x, size.y);
        let x_index = x_size / Self::BLOCK_SIZE.0
            + if x_size % Self::BLOCK_SIZE.0 != 0 {
                1
            } else {
                0
            };
        let y_index = y_size / Self::BLOCK_SIZE.1
            + if y_size % Self::BLOCK_SIZE.1 != 0 {
                1
            } else {
                0
            };

        Self {
            x_index,
            y_index,
            size: (x_size, y_size),
            atom_count: AtomicU32::new(0),
        }
    }
    pub fn iter(&self) -> Option<FilmIter> {
        let index = self
            .atom_count
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        if index >= self.x_index * self.y_index {
            return None;
        }
        let x = index / self.x_index;
        let y = index % self.y_index;

        let mut left_up = (Self::BLOCK_SIZE.0 * x, Self::BLOCK_SIZE.1 * y);
        let mut right_down = (Self::BLOCK_SIZE.0 * (x + 1), Self::BLOCK_SIZE.1 * (y + 1));
        if right_down.1 > self.size.1 {
            left_up.1 = self.size.1
        }
        if right_down.0 > self.size.0 {
            right_down.0 = self.size.0
        }
        Some(FilmIter::new(
            left_up,
            right_down,
            (right_down.0 - left_up.0, right_down.1 - right_down.1),
        ))
    }
}
pub struct FilmIter {
    pub block_size: (u32, u32),
    pub left_up: (u32, u32),
    pub right_down: (u32, u32),
    pub now: (u32, u32),
}

impl FilmIter {
    pub fn new(left_up: (u32, u32), right_down: (u32, u32), block_size: (u32, u32)) -> Self {
        Self {
            left_up,
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
    type Item = (f64, f64);
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
            Some((x as f64, y as f64))
        }
    }
}

/// 求交集合
#[derive(Default,Clone, Copy)]
pub struct InteractionCommon {
    pub w0: DVec3,
    pub p: DVec3,
    pub normal: DVec3,
    pub time: f64,
}
#[derive(Default)]
pub struct SurfaceInteraction<'a> {
    pub common: InteractionCommon,
    uv: DVec2,
    dpdu: DVec3,
    dpdv: DVec3,
    shape: Option<&'a dyn Primitive>,
    pub shading: Shading,
    pub bsdf:Option<BSDF>,
    pub light:Option<&'a Light>
}
// impl Default for SurfaceInteraction{
//     fn default() -> Self {

//     }
// }
impl<'a> SurfaceInteraction<'a> {
    pub fn new(
        p: DVec3,
        uv: DVec2,
        normal:DVec3,
        w_out: DVec3,
        dpdu: DVec3,
        dpdv: DVec3,
        dndu: DVec3,
        dndv: DVec3,
        time: f64,
        shape: Option<&'a dyn Primitive>,
        is_light:bool,
    ) -> Self {
        Self {
            common: InteractionCommon {
                w0: w_out,
                p: p,
                normal: normal,
                time: time
            },
            uv,
            dpdu,
            dpdv,
            shape,
            shading: Shading {
                n: dpdu.cross(dpdv).normalize(),
                dpdu,
                dpdv,
                dndu,
                dndv,
            },
            bsdf:None,
            light:None
        }
    }
    pub fn compute_scattering(&mut self,ray:RayDiff,mode:TransportMode){
        if let Some( shape)=self.shape{
            let primitive = unsafe { &*shape };
            primitive.compute_scattering(self,TransportMode::Importance);
        }
    }
    pub fn spawn_ray(&self,wi:&DVec3)->RayDiff{
        let ray=Ray::new(self.common.p, *wi);
        RayDiff::new(ray)
    }
    pub fn le(&self,w_in:DVec3)->DVec3{
        if let Some(light)=self.light{
            light.le(&w_in)
        }else{  
            DVec3::ZERO
        }

    }
}
#[derive(Default)]
pub struct Shading {
    pub n: DVec3,
    pub dpdu: DVec3,
    pub dpdv: DVec3,
    pub dndu: DVec3,
    pub dndv: DVec3,
}
impl Shading {
    pub fn new(n: DVec3, dpdu: DVec3, dpdv: DVec3, dndu: DVec3, dndv: DVec3) -> Self {
        Self {
            n,
            dpdu,
            dpdv,
            dndu,
            dndv,
        }
    }
}
#[derive(Default)]
pub struct Visibility{
    pub a:InteractionCommon,
    pub b:InteractionCommon,
}
impl Visibility{
    //是否可视
    fn is_vis(&self,sence:&Sence)->f64{
        let dir=self.a.p-self.b.p;
        let ray_diff=RayDiff::new(Ray::from_with_t(self.b.p, dir, 0.01,dir.length()-0.001));
        if sence.interacect(ray_diff).is_none(){
            1.0
        }else{
            0.0
        }
    }
    fn g(&self,sence:&Sence)->f64{
        let vis=self.is_vis(sence);
        let dir=(self.a.p-self.b.p);
        vis*self.a.normal.dot(dir.normalize()).clamp(0.0, 1.0)
            *self.b.normal.dot(-dir.normalize()).clamp(0.0, 1.0)/dir.length_squared()
    }
}