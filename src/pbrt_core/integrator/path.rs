use glam::{f64::DVec3, u32::UVec2};
use image::{Rgb, RgbImage};
use pbr::ProgressBar;
use std::{
    path::Path,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use crate::pbrt_core::{
    bxdf::BxDFType,
    camera::{Camera, CameraSample},
    integrator::to_color,
    material::BSDF,
    primitive::Primitive,
    sampler::Sampler,
    tool::{sence::Sence, Film, Ray, RayDiff},
};

//路径追踪积分器
pub struct PathIntegrator {
    q: f64,
    max_path: usize,
    sampler: Sampler,
    size:UVec2,
}
impl Default for PathIntegrator {
    fn default() -> Self {
        Self {
            q: 0.9,
            max_path: 10,
            sampler: Sampler::default(),
            size:UVec2::new(512,512)
        }
    }
}
impl PathIntegrator {
    pub fn new(q: f64, max_path: usize, sampler: Sampler,size:UVec2) -> Self {
        Self {
            q,
            max_path,
            sampler,
            size
        }
    }
    fn is_next(&self, dept: &mut usize) -> Option<f64> {
        *dept += 1;
        let pp = rand::random::<f64>();
        if *dept > self.max_path {
            if pp > self.q {
                None
            } else {
                Some(self.q)
            }
        } else {
            Some(1.0)
        }
    }
    pub fn render_process(self, name: &str, num: u64, sence: &Sence) {
        let (sender, receiver) = mpsc::channel::<(u64, u64, DVec3)>();
        let film = Film::new(self.size);
        let camera = sence.camera;
        thread::scope(|f| {
            for _ in 0..num {
                f.spawn(self.render_core(&film, &camera, sender.clone(), sence));
            }
            drop(sender);
            f.spawn(move || Self::output(receiver, self.size, name, num));
        });
    }
    fn render_core<'a, 'b>(
        &'b self,
        film: &'a Film,
        camera: &'a Camera,
        send: Sender<(u64, u64, DVec3)>,
        sence: &'a Sence,
    ) -> impl FnOnce() + 'a
    where
        'b: 'a,
    {
        move || {
            let n = self.sampler.num;
            let mut sampler = self.sampler.clone();
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
            bar.add(1);
        }
        let path =
            Path::new("./image").join(format!("thread_{}_{}_{name}_{num}.png", size.x, size.y));
        format!("渲染完成，图像输出:{}", path.display());
        bar.finish_print("渲染完成，图像输出");
        image
            .save_with_format(path, image::ImageFormat::Jpeg)
            .expect("图片保存失败");
    }
    fn fi(&self, ray: RayDiff, sence: &Sence, sampler: &mut Sampler) -> DVec3 {
        let mut ans = DVec3::ZERO;
        let mut dept = 0;
        let mut wegith: DVec3 = DVec3::ONE;
        let mut ray = ray.clone();
        while let Some(p) = self.is_next(&mut dept) {
            if let Some(mut item) = sence.interacect(ray) {
                //对光源采样
                ans += wegith * sence.uniform_sample_one_light(&item, sampler);

                item.compute_scattering(ray, crate::pbrt_core::bxdf::TransportMode::Radiance);
                //对SBDF采样
                let mut wi = DVec3::default();
                // let mut pdf = Default::default();
                let wo = -ray.o.dir;
                if let Some(ref bsdf) = item.bsdf {
                    wegith *= (bsdf.f(
                        &wo,
                        &mut wi,
                        // sampler.sample_2d_d(),
                        // &mut pdf,
                        BxDFType::All as u32,
                    ));
                }
                if item.common.is_light {
                    return ans + wegith;
                }
                //生成光线
                ray = RayDiff::new(Ray::new(item.common.p, sampler.smapel_dir()));
            }else{
                return ans;
            }
        }
        ans
    }
    pub fn render_process_debug(self, name: &str, num: u64, sence: &Sence) {
        let (sender, receiver) = mpsc::channel::<(u64, u64, DVec3)>();
        let film = Film::new(self.size);
        let bar_size =self.size.x * self.size.y;
        let n = self.sampler.num;
        let mut sampler = self.sampler.clone();
        let camera = sence.camera;
        let mut bar = ProgressBar::new(bar_size as u64);
        let mut image = RgbImage::new(self.size.x, self.size.y);
        while let Some(item) = film.iter() {
            for (u, v) in item {
                let mut color = DVec3::ZERO;
                for _ in 0..n {
                    let camera_sample = CameraSample::new(u, v, &mut sampler);
                    let ray = camera.generate_ray(camera_sample);
                    color += self.fi(ray, sence,&mut sampler);
                }
                image.put_pixel(u as u32, v as u32, to_color(color, num as f64));
                bar.inc();
            }
        }
        let path =
            Path::new("./image").join(format!("thread_{}_{}_{name}_{num}.png", self.size.x, self.size.y));
        format!("渲染完成，图像输出:{}", path.display());
        bar.finish_print("渲染完成，图像输出");
        image
            .save_with_format(path, image::ImageFormat::Jpeg)
            .expect("图片保存失败");
    }
}
