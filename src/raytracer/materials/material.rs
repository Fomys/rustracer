use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::scene::Scene;
use crate::raytracer::color::Color;

pub trait Material {
    fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize) -> Color;
}