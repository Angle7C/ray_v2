// use super::{MicrofacetDistribution, BxDFAble, func::{sin2_theta, cos_theta, cos2_theta, self}};

// //使用Beckmann概率分布，适用各项同性
// pub struct BeckmannDistribution {
//     alphax: f64,
//     alphay: f64,
//     sample_visible_area: bool,
// }
// impl BeckmannDistribution {
//     pub fn new(alphax: f64, alphay: f64, sample_visible_area: bool) -> Self {
//        Self { alphax, alphay, sample_visible_area } 
//     }
// }
// impl BxDFAble for BeckmannDistribution{
//     fn match_type(&self, flag: u32) -> bool {
//         todo!()
//     }

//     fn fi(&self, w_in: &glam::DVec3, w_out: &glam::DVec3) -> glam::DVec3 {
//         todo!()
//     }

//     fn rho(&self, w_in: glam::DVec3, w_out: glam::DVec3, sample_point: glam::DVec2) -> glam::DVec3 {
//         todo!()
//     }
// }
// impl MicrofacetDistribution for BeckmannDistribution{
//     fn d(&self,wh:&glam::DVec3)->f64 {
//         let tan2=sin2_theta(wh)/cos2_theta(wh);
//         if tan2.is_infinite(){
//             0.0
//         }else{
//             let cos4=cos2_theta(wh)*cos2_theta(wh);
//             func
//             f64::exp(-tan2*)
//         }
//     }

//     fn lamdba(&self,w:&glam::DVec3)->f64 {
//         todo!()
//     }

//     fn sample_wh(&self,w_out:&glam::DVec3,w_in:&glam::DVec3)->glam::DVec3 {
//         todo!()
//     }

//     fn pdf(&self,w_out:&glam::DVec3,wh:&glam::DVec3)->f64 {
//         todo!()
//     }
// }
// //GGX模型
// pub struct TrowbridgeReitz {}
