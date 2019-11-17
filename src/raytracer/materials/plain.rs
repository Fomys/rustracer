use crate::raytracer::color::Color;
use crate::raytracer::materials::material::Material;
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::scene::Scene;

pub struct Plain {
    pub color: Color,
}

impl Material for Plain {
    fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize) -> Color {
        self.color
    }
}