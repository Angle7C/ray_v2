use std::{sync::mpsc::{self, Sender, Receiver}, thread,  path::Path};

use glam::{f64::DVec3, UVec2};
use image::{Rgb, RgbImage};
use indicatif::ProgressBar;

use self::path::PathIntegrator;

use super::{tool::{sence::Sence, Film, RayDiff}, camera::{Camera, CameraSample}, sampler::{self, Sampler}};

pub mod path;
pub mod direct;
pub enum Integrator{
    Path(Box<PathIntegrator>)
}
pub trait IntegratorAble{
    fn is_next(&self, dept: &mut usize) -> Option<f64>;
    fn fi(&self,ray:RayDiff,sence:&Sence,sampler:&mut Sampler)->DVec3;
}
impl IntegratorAble for Integrator{
    fn is_next(&self, dept: &mut usize) -> Option<f64> {
        match &self{
            Integrator::Path(path) => path.is_next(dept)
        }
    }

    fn fi(&self,ray:RayDiff,sence:&Sence,sampler:&mut Sampler)->DVec3 {
        match  &self {
            Integrator::Path(path) => path.fi(ray, sence, sampler),
        }
    }
}
impl Integrator{
    pub fn render_process(self, name: &str, num: u64, sence: &Sence,size:UVec2,sampler:Sampler) {
        let (sender, receiver) = mpsc::channel::<(u64, u64, DVec3)>();
        let film = Film::new(size);
        let camera = sence.camera;
        thread::scope(|f| {
            for _ in 0..num {
                f.spawn(self.render_core(&film, &camera, sender.clone(), sence,sampler.clone()));
            }
            drop(sender);
            f.spawn(move || Self::output(receiver, size, name, sampler.num as u64));
        });
    }
    fn render_core<'a, 'b>(
        &'b self,
        film: &'a Film,
        camera: &'a Camera,
        send: Sender<(u64, u64, DVec3)>,
        sence: &'a Sence,
        mut sampler:Sampler,
    ) -> impl FnOnce() + 'a where 'b: 'a,
    {
        move || {
            let n = sampler.num;
            let mut sampler = sampler.clone();
            while let Some(item) = film.iter() {
                for (u, v) in item {
                    let mut color = DVec3::ZERO;
                    for _ in 0..n {
                        let camera_sample = CameraSample::new(u, v, &mut sampler);
                        let ray = camera.generate_ray(camera_sample);
                        color += self.fi(ray, sence,&mut  sampler);
                    }
                    send.send((u as u64, v as u64, color))
                        .expect(&format!("传输color失败 x:{u},y:{v},color:{color}"));
                }
            }
        }
    }
    fn output(rece: Receiver<(u64, u64, DVec3)>, size: UVec2, name: &str, num: u64) {
        let bar_size = size.x * size.y;
        let mut bar = ProgressBar::new(bar_size as u64);
        let mut image = RgbImage::new(size.x, size.y);
        for (u, v, color) in rece.iter() {
            image.put_pixel(u as u32, v as u32, to_color(color, num as f64));
            bar.inc(1);
        }
        let path =
            Path::new("./image").join(format!("thread_{}_{}_{name}_{num}.png", size.x, size.y));
        format!("渲染完成，图像输出:{}", path.display());
        bar.finish_with_message("渲染完成，图像输出");
        image
            .save_with_format(path, image::ImageFormat::Jpeg)
            .expect("图片保存失败");
    }

    pub fn render_process_debug(self, name: &str, num: u64, sence: &Sence,size:UVec2,sampler:Sampler) {
        let (sender, receiver) = mpsc::channel::<(u64, u64, DVec3)>();
        let film = Film::new(size);
        let bar_size =size.x * size.y;
        let n = sampler.num;
        let mut sampler = sampler.clone();
        let camera = sence.camera;
        let mut bar = ProgressBar::new(bar_size as u64);
        let mut image = RgbImage::new(size.x, size.y);
        while let Some(item) = film.iter() {
            for (u, v) in item {
                let mut color = DVec3::ZERO;
                for _ in 0..n {
                    let camera_sample = CameraSample::new(u, v, &mut sampler);
                    let ray = camera.generate_ray(camera_sample);
                    color += self.fi(ray, sence,&mut sampler);
                }
                image.put_pixel(u as u32, v as u32, to_color(color, num as f64));
                bar.inc(1);
            }
        }
        let path =
            Path::new("./image").join(format!("thread_{}_{}_{name}_{num}.png", size.x, size.y));
        format!("渲染完成，图像输出:{}", path.display());
        bar.finish_with_message("渲染完成，图像输出");
        image
            .save_with_format(path, image::ImageFormat::Jpeg)
            .expect("图片保存失败");
    }

}
pub fn to_color(color:DVec3,ssp:f64)->Rgb<u8>{
        let vec = (color / ssp).powf(0.5);
        let rgb = vec * 255.0;
        let color= Rgb([
            rgb.x.clamp(0.0, 255.0) as u8,
            rgb.y.clamp(0.0, 255.0) as u8,
            rgb.z.clamp(0.0, 255.0) as u8,
        ]);
        color
}