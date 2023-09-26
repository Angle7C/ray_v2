use glam::Vec3A;

use crate::pbrt_core::tool::{vistest::VisibilityTester, color::Color};

use super::LightAble;

pub struct PointLight {
    p: Vec3A,
    lemit: Color,
}
impl PointLight{
    pub fn new(p:Vec3A,lemit:Color)->Self{
        Self { p, lemit }
    }
}
impl LightAble for PointLight {
    fn get_type(&self) -> super::LightType {
        super::LightType::DeltaPosition
    }
    fn pdf_li(
        &self,
        // vis: &mut crate::pbrt_core::tool::vistest::VisibilityTester,
        inter: &crate::pbrt_core::tool::interaction::SurfaceInteraction,
        wi: Vec3A,
    ) -> f32 {
        1.0
    }
    fn sample_li(
        &self,
        common: &crate::pbrt_core::tool::interaction::InteractionCommon,
        light_common: &mut crate::pbrt_core::tool::interaction::InteractionCommon,
        u: glam::Vec2,
        wi: &mut Vec3A,
        pdf: &mut f32,
        vis: &mut crate::pbrt_core::tool::vistest::VisibilityTester,
    ) -> crate::pbrt_core::tool::color::Color {
        *wi = (self.p - common.p).normalize();
        light_common.p = self.p;
        light_common.n = *wi;
        light_common.t = f32::INFINITY;
        *pdf = 1.0;
        *vis = VisibilityTester::new(*light_common, *common);
        self.lemit / (self.p - common.p).length_squared()
    }

    fn get_n_samples(&self) -> usize {
        1
    }
}
