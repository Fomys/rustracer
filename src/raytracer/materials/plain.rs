use crate::raytracer::color::{Color, ColorInfo};
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::materials::material::Material;
use crate::raytracer::scene::Scene;

#[derive(Clone)]
pub struct Plain {}

impl Material for Plain {
    #[allow(unused_variables)]
    fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize) -> ColorInfo {
        ColorInfo { color: Color::black(), ratio: 0.0 }
    }
}