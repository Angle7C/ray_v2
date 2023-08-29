
use glam::{Affine3A, DMat4, Mat4, Vec2, Vec3,  Vec4, DVec2, DVec3, DVec4};

use super::{tool::{RayDiff, Ray}, sampler::Sampler};

#[derive(Debug,Default,Clone, Copy)]
pub struct Camera {
    eye: DVec3,
    screen_to_camera: DMat4,
    camera_to_world: DMat4,
    mode:CameraMode
}

#[derive(Debug,Default,Clone, Copy)]
pub enum CameraMode {
    #[default]
    P,
    O,
}
pub struct  CameraSample{
    pub film_point:DVec2,
}
impl CameraSample{
    pub fn new(x:f64,y:f64,sampler:&mut Sampler)->Self{
       let point= sampler.sample_2d_d()+DVec2{x,y};
        Self{film_point:point}
    }
}
impl Camera {
    fn computer_viewport(size: DVec2) -> DMat4 {
        let mat = DMat4::from_cols(
            DVec4::new(size.x / 2.0, 0.0, 0.0, 0.0),
            DVec4::new(0.0, size.y / 2.0, 0.0, 0.0),
            DVec4::Z,
            (size.extend(0.0) / 2.0).extend(1.0),
        );
        mat.inverse()
    }
    pub fn new(eye: DVec3, center: DVec3, up: DVec3, size: Vec2, mode: CameraMode, fov: f64)->Self{
        let look_at_lh = DMat4::look_at_lh(eye, center,up);
        let p = match mode {
            CameraMode::O => DMat4::orthographic_lh(-1.0, 1.0, -1.0, 1.0, 0.01, 1000.0),
            CameraMode::P => DMat4::perspective_lh(fov.to_radians(), 1.0, 0.01, 1000.0),
        };
        let world_to_camera = p*look_at_lh;
        let screen_to_camera=Self::computer_viewport(size.as_dvec2());
        Self{
            eye:eye.into(),
            screen_to_camera,
            camera_to_world:world_to_camera.inverse(),
            mode
        }

    }
    pub fn generate_ray(&self,sample:CameraSample)->RayDiff{
        let p = self.screen_to_camera.transform_point3(sample.film_point.extend(0.0));
        match self.mode{
            CameraMode::O=>{
                let p=self.camera_to_world.transform_point3(p);
                let dir=self.camera_to_world.transform_vector3(DVec3::Z);
                RayDiff::new(Ray::new(p.into(), dir.into()))
            },
            CameraMode::P=>{
                let dir_p=self.camera_to_world.project_point3(p);
                let dir=(dir_p-self.eye).normalize();
                RayDiff::new(Ray::new(self.eye, dir))
            }
        }
    
    }
}
