use std::{time::SystemTime, f32::consts::{FRAC_PI_4, FRAC_PI_2}};

use glam::{Vec2, Vec3};
use rand::{rngs::StdRng, SeedableRng, Rng};
pub mod distribution_1d;
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
    pub fn sample_1d_d(&mut self)->f32{
        self.rand.gen_range(0.0..1.0)
    }
    pub fn sample_2d_d(&mut self)->Vec2{
        let x=self.sample_1d_d();
        let y=self.sample_1d_d();
        Vec2{x,y }
    }
    pub fn sample_2d(&mut self)->Vec2{
        self.sample_2d_d()
    }
    pub fn sample_d(&mut self)->f32{
        self.sample_1d_d() as f32
    }
    pub fn smapel_dir(&mut self)->Vec3{
        let x=self.sample_1d_d();
        let y=self.sample_1d_d();
        let z=self.sample_1d_d();
        Vec3 { x,y, z }.normalize()
    }
}
pub fn cosine_sample_hemisphere(u:Vec2)->Vec3{
    let d=concentric_sample_disk(u);
    let z=0.0_f32.max(1.0-d.length_squared())
    .sqrt();
    d.extend(z)
}
pub fn concentric_sample_disk(u:Vec2)->Vec2{
    let offset=u*2.0-Vec2::ONE;
    if offset.x==0.0&&offset.y==0.0{
        return Vec2::ZERO;
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
    Vec2{
        x:theta.cos(),
        y:theta.sin(),
    }*r

}