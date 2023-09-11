use std::{time::SystemTime, f64::consts::{FRAC_PI_4, FRAC_PI_2}};

use glam::{f64::DVec2,f32::Vec2, DVec3};
use rand::{rngs::StdRng, SeedableRng, Rng};

pub struct Sampler {
   pub  rand: StdRng,
   pub  num:usize
}
impl Default for Sampler {
    fn default() -> Self {
        unsafe {
            Sampler {
                rand: SeedableRng::seed_from_u64(
                    SystemTime::now().elapsed().unwrap_unchecked().as_secs(),
                ),
                num:1
            }
        }
    }
}
impl Clone for Sampler{
    fn clone(&self) -> Self {
        unsafe {
            Sampler {
                rand: SeedableRng::seed_from_u64(
                    SystemTime::now().elapsed().unwrap_unchecked().as_secs(),
                ),
                num:self.num
            }
        }
    }
}
impl Sampler {
    pub fn new(num:usize)->Self{
        unsafe {
            Sampler {
                rand: SeedableRng::seed_from_u64(
                    SystemTime::now().elapsed().unwrap_unchecked().as_secs(),
                ),
                num
            }
        }
    }
    pub fn sample_1d_d(&mut self)->f64{
        self.rand.gen_range(0.0..1.0)
    }
    pub fn sample_2d_d(&mut self)->DVec2{
        let x=self.sample_1d_d();
        let y=self.sample_1d_d();
        DVec2{x,y }
    }
    pub fn sample_2d(&mut self)->Vec2{
        self.sample_2d_d().as_vec2()
    }
    pub fn sample_d(&mut self)->f32{
        self.sample_1d_d() as f32
    }
    pub fn smapel_dir(&mut self)->DVec3{
        let x=self.sample_1d_d();
        let y=self.sample_1d_d();
        let z=self.sample_1d_d();
        DVec3 { x,y, z }.normalize()
    }
}
pub fn cosine_sample_hemisphere(u:DVec2)->DVec3{
    let d=concentric_sample_disk(u);
    let z=0.0_f64.max(1.0-d.length_squared() as f64)
    .sqrt();
    d.extend(z)
}
pub fn concentric_sample_disk(u:DVec2)->DVec2{
    let offset=u*2.0-DVec2::ONE;
    if offset.x==0.0&&offset.y==0.0{
        return DVec2::ZERO;
    }
    let theta;
    let r;
    if offset.x.abs()>offset.y.abs(){
        r=offset.x;
        theta=FRAC_PI_4* (offset.y/offset.x);
    }else{
        r=offset.y;
        theta=FRAC_PI_2-FRAC_PI_4*(offset.x/offset.y)
    }
    DVec2{
        x:theta.cos(),
        y:theta.sin(),
    }*r

}