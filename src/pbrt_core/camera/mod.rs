
use glam::{Mat4, Vec2, Vec3, Vec4};

use super::{tool::{RayDiff, Ray}, sampler::Sampler};

#[derive(Debug,Default,Clone, Copy)]
pub struct Camera {
    //相机原点
    eye: Vec3,
    //屏幕——相机
    screen_to_camera: Mat4,
    //相机-世界
    camera_to_world: Mat4,
    //相机模型
    mode:CameraMode
}

#[derive(Debug,Default,Clone, Copy)]
pub enum CameraMode {
    #[default]
    P,
    O,
}
//相机采样器
pub struct  CameraSample{
    pub film_point:Vec2,
}

impl CameraSample{
    pub fn new(x:f32,y:f32,sampler:&mut Sampler)->Self{
       let point= sampler.sample_2d_d()+Vec2{x,y};
        Self{film_point:point}
    }
}
impl Camera {
    //计算视口矩阵
    fn computer_viewport(size: Vec2) -> Mat4 {
        let mat = Mat4::from_cols(
            Vec4::new(size.x / 2.0, 0.0, 0.0, 0.0),
            Vec4::new(0.0, size.y / 2.0, 0.0, 0.0),
            Vec4::Z,
            (size.extend(0.0) / 2.0).extend(1.0),
        );
        mat.inverse()
    }
    //构造
    pub fn new(eye: Vec3, center: Vec3, up: Vec3, size: Vec2, mode: CameraMode, fov: f32)->Self{
        let look_at_lh = Mat4::look_at_lh(eye, center,up);
        let p = match mode {
            CameraMode::O => Mat4::orthographic_lh(-1.0, 1.0, -1.0, 1.0, 0.01, 1000.0),
            CameraMode::P => Mat4::perspective_lh(fov.to_radians(), 1.0, 0.01, 1000.0),
        };
        let world_to_camera = p*look_at_lh;
        let screen_to_camera=Self::computer_viewport(size);
        Self{
            eye,
            screen_to_camera,
            camera_to_world:world_to_camera.inverse(),
            mode
        }
    }
    pub fn reset_size(&mut self,size:Vec2){
        let screen_to_camera=Self::computer_viewport(size);
        self.screen_to_camera=screen_to_camera;
    }
    pub fn generate_ray(&self,sample:CameraSample)->RayDiff{
        let p = self.screen_to_camera.transform_point3(sample.film_point.extend(0.0));
        match self.mode{
            CameraMode::O=>{
                let p=self.camera_to_world.transform_point3(p);
                let dir=self.camera_to_world.transform_vector3(Vec3::Z);
                RayDiff::new(Ray::new(p, dir))
            },
            CameraMode::P=>{
                let dir_p=self.camera_to_world.project_point3(p);
                let dir=(dir_p-self.eye).normalize();
                RayDiff::new(Ray::new(self.eye, dir))
            }
        }
    
    }
}
