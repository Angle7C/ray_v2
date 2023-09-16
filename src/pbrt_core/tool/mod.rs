use std::ops::Add;

use bvh::aabb::AABB;
use glam::{Vec2, Vec3};

use self::sence::Sence;

use super::{bxdf::TransportMode, light::LightAble, material::BSDF, primitive::Primitive};

pub mod color;
pub mod film;
pub mod func;
pub mod log;
pub mod sence;
pub mod setting;
pub mod tile;
pub mod error;
pub mod mipmap;
pub mod build;
/// 光线
#[derive(Debug, Default, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
    pub t_min: f32,
    pub t_max: f32,
}
impl Ray {
    pub fn new(origin: Vec3, dir: Vec3) -> Self {
        Self {
            origin,
            dir: dir.normalize(),
            t_max: f32::INFINITY,
            t_min: 0.0,
        }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.dir * t
    }
    pub fn from_with_t(origin: Vec3, dir: Vec3, t_min: f32, t_max: f32) -> Self {
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
    pub p: Vec3,
    pub p_dx: Option<Vec3>,
    pub p_dy: Option<Vec3>,
}
impl RayDiff {
    pub fn new(o: Ray) -> Self {
        Self {
            o,
            dx: None,
            dy: None,
        }
    }
    pub fn at(&self, t: f32) -> RayDiffHit {
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
    pub min: Vec3,
    pub max: Vec3,
}

impl From<Bound<3>> for AABB {
    fn from(value: Bound<3>) -> Self {
        let min = value.min;
        let max = value.max;
        Self {
            min,
            max,
        }
    }
}

impl Bound<2> {
    pub fn new(min: Vec2, max: Vec2) -> Self {
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
    pub fn center(&self) -> Vec2 {
        let center = (self.min + self.max) / 2.0;
        center.truncate()
    }
}
impl Bound<3> {
    pub fn new(min: Vec3, max: Vec3) -> Self {
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
    pub fn center(&self) -> Vec3 {
        let center = (self.min + self.max) / 2.0;
        center
    }
}

impl<const N: usize> Default for Bound<N> {
    fn default() -> Self {
        Self {
            min: Vec3::splat(f32::INFINITY),
            max: Vec3::splat(-f32::INFINITY),
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

/// 求交集合
#[derive(Default, Clone, Copy)]
pub struct InteractionCommon {
    pub w0: Vec3,
    pub p: Vec3,
    pub normal: Vec3,
    pub time: f32,
    pub uv: Vec2,
}
impl InteractionCommon {
    pub fn new(w0: Vec3, p: Vec3, normal: Vec3, time: f32, uv: Vec2) -> Self {
        Self {
            w0,
            p,
            normal,
            time,
            uv,
        }
    }
}
#[derive(Default)]
pub struct SurfaceInteraction<'a> {
    pub common: InteractionCommon,
    _uv: Vec2,
    _dpdu: Vec3,
    _dpdv: Vec3,
    //求交的图元信息
    shape: Option<&'a dyn Primitive>,
    // 渲染信息与几何信息
    pub shading: Shading,
    // BSDF采样值。表示表面的对光的作用。
    pub bsdf: Option<BSDF>,
    //该交点是不是光源。
    pub light: Option<&'a dyn LightAble>,
}
impl<'a> SurfaceInteraction<'a> {
    pub fn new(
        p: Vec3,
        uv: Vec2,
        normal: Vec3,
        w_out: Vec3,
        dpdu: Vec3,
        dpdv: Vec3,
        dndu: Vec3,
        dndv: Vec3,
        time: f32,
        shape: Option<&'a dyn Primitive>,
        light: Option<&'a dyn LightAble>,
    ) -> Self {
        Self {
            common: InteractionCommon {
                w0: w_out,
                p: p,
                normal: normal,
                time: time,
                uv,
            },
            _uv: uv,
            _dpdu: dpdu,
            _dpdv: dpdv,
            shape,
            shading: Shading {
                n: dpdu.cross(dpdv).normalize(),
                dpdu,
                dpdv,
                dndu,
                dndv,
            },
            bsdf: None,
            light: light,
        }
    }
    pub fn compute_scattering(&mut self, _ray: RayDiff, _mode: TransportMode) {
        if let Some(shape) = self.shape {
            // let primitive = &*shape ;
            shape.compute_scattering(self, TransportMode::Importance);
        }
    }
    pub fn spawn_ray(&self, wi: &Vec3) -> RayDiff {
        let ray = Ray::new(self.common.p, *wi);
        RayDiff::new(ray)
    }
    pub fn le(&self, w_in: Vec3) -> Vec3 {
        if let Some(light) = self.light {
            light.le(w_in)
        } else {
            Vec3::ZERO
        }
    }
}
#[derive(Default)]
pub struct Shading {
    pub n: Vec3,
    pub dpdu: Vec3,
    pub dpdv: Vec3,
    pub dndu: Vec3,
    pub dndv: Vec3,
}
impl Shading {
    pub fn new(n: Vec3, dpdu: Vec3, dpdv: Vec3, dndu: Vec3, dndv: Vec3) -> Self {
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
pub struct Visibility {
    pub a: InteractionCommon,
    pub b: InteractionCommon,
}
impl Visibility {
    //是否可视
    fn is_vis(&self, sence: &Sence) -> f32 {
        let dir = self.a.p - self.b.p;
        let ray_diff = RayDiff::new(Ray::from_with_t(self.b.p, dir, 0.01, dir.length() - 0.001));
        if sence.interacect(ray_diff).is_none() {
            1.0
        } else {
            0.0
        }
    }
    fn g(&self, sence: &Sence) -> f32 {
        let vis = self.is_vis(sence);
        let dir = self.a.p - self.b.p;
        let value=vis * self.a.normal.dot(dir.normalize()).abs() * self.b.normal.dot(dir.normalize()).abs();
        value/ dir.length_squared()
    }
    // fn get_dir(&self,sence:&Sence)->f32{}
}
