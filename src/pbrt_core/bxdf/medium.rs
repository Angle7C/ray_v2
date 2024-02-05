/*
体渲染理论，
在空间存在各种各样的介质（空气，离子体），该理论描述了在不同介质中的渲染效果。
介质在该理论里被抽象成一堆粒子集合。
在介质中光线的传播：
- 吸收 (absorption)
- 发光 ( emission )
- 衰减 (attenuation)
- 散射 (scattering)
*/

/*
 吸收（absorption）：
 当光线从一个介质中射出，经过该介质后，光线会被吸收，不会再继续向外散射。
 被吸收的光线不在产生影响。

 Lo(p,w)-Li(p,-w)=Det(L)=-omage_a(p,w) * rho(p,w)*Li(p,-w)dt
 在p点，w方向的光线，经过介质后，会被吸收，Li(p,w)为吸收光线的能量。
 rho(p,w) 吸收系数，这是一个与位置，方向相关的函数。
 Li(p,-w) 是穿过介质之前的radiance
 Lo(p, w) 是穿过之后的radince

 */
/*
自发光（emssion）
    dL(p,w)=Le(p,w)dt
    Le(p,w)是自发光强度，是一个与位置，方向相关的函数。
*/
/*
散射（sacttering）

Lo(p,w)-Li(p,-w)=omage_b(p,w) * rho(p,-w)dt
omage 为散射系数。

*/
/*
衰减（Attenuation）

由此可以得到衰减系数：
omage_t(p,w)=omage_a(p,w) +omage_b(p,w)
散射系数与衰减系数的比值是个有用的值

abledo=omage_b(p,w)/omage_t(p,w) (0,1)
*/

use std::{f32::consts::PI};

use glam::Vec3;

use crate::pbrt_core::{tool::Ray, sampler::Sampler};

/*
   光线穿透率，是表示两个点之间，光穿过介质之后剩余部分的比例。
   dL(p,w)/dt=-omage_t(p,w) * rho(p,w)
   如果介质的内部是完全均匀的，即各点的衰减系数是完全相同的
   那么积分后的值
   T_r(p->p')=exp(-omage_t(p,w) * |p-p'|)

   如果将光穿过的路程d，均匀分成N份，则穿透成表示成下面的公式：
   T_r(p->p+d)=(T_r(p->p+d/N))N
   当N趋于∞时，
   =(T_r(p->p+dt))^N
    T_r(p->p+dt)=1-abledodt
*/
pub trait PhaseFunction {
   fn p(&self,wo:&Vec3,wi:&Vec3)->f32;

   #[inline]
   fn phase_hg(cos:f32,g:f32)->f32{
    let denom=1.0+g*g+2.0*g*cos;
      (1.0/4.0*PI)*(1.0-g*g)/(denom*denom.sqrt())
   }
}

pub trait Medium {
    fn tr(&self,ray:&Ray,sampler:&Sampler)->Vec3;
}

pub struct HenyeyGreenstein{
    g:f32
}
impl HenyeyGreenstein{
    pub fn new(g:f32)->Self{
        Self{g}
    }
}
impl PhaseFunction for HenyeyGreenstein{
    fn p(&self,wo:&Vec3,wi:&Vec3)->f32{
        let cos=wi.dot(*wo);

        return HenyeyGreenstein::phase_hg(cos,self.g);
    }
}