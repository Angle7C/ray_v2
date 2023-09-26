use glam::{Vec2, Vec3A};

use crate::pbrt_core::{light::TransportMode, material::BSDF};

use super::ray::Ray;

#[derive(Debug, Clone, Copy, Default)]
pub struct InteractionCommon {
    pub p: Vec3A,
    pub t: f32,
    pub wo: Vec3A,
    pub n: Vec3A,
}
impl InteractionCommon {
    pub fn new(p: Vec3A, t: f32, wo: Vec3A, n: Vec3A) -> Self {
        Self { p, t, wo, n }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Shading {
    pub n: Vec3A,
    pub dpdu: Vec3A,
    pub dpdv: Vec3A,
    pub dndu: Vec3A,
    pub dndv: Vec3A,
}
impl Shading {
    pub fn new(n: Vec3A, dpdu: Vec3A, dpdv: Vec3A, dndu: Vec3A, dndv: Vec3A) -> Self {
        Self {
            n,
            dpdu,
            dpdv,
            dndu,
            dndv,
        }
    }
}
pub struct SurfaceInteraction {
    pub common: InteractionCommon,
    pub uv: Vec2,
    pub dpdx: Vec3A,
    pub dpdy: Vec3A,
    pub dudx: Vec3A,
    pub dvdx: Vec3A,
    pub dudy: Vec3A,
    pub dvdy: Vec3A,
    pub shading: Shading,
    pub bsdf: Option<BSDF>,
}
impl SurfaceInteraction {
    pub fn new(
        common: InteractionCommon,
        uv: Vec2,
        dpdx: Vec3A,
        dpdy: Vec3A,
        dudx: Vec3A,
        dudy: Vec3A,
        dvdx: Vec3A,
        dvdy: Vec3A,
        shading: Shading,
        bsdf: Option<BSDF>,
    ) -> Self {
        Self {
            common,
            uv,
            dpdx,
            dpdy,
            dudx,
            dvdx,
            dudy,
            dvdy,
            shading,
            bsdf,
        }
    }

    pub fn compute_scattering(&mut self, ray: &Ray, mode: TransportMode) {}
}
