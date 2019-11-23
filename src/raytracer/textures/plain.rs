use crate::raytracer::color::Color;
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::textures::texture::Texture;

#[derive(Debug, Clone, Copy)]
pub struct Plain {
    pub color: Color,
}

impl Texture for Plain {
    #[allow(unused_variables)]
    fn get_color(&self, hitinfo: &HitInfo) -> Color {
        self.color
    }
}