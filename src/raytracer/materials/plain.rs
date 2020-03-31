use crate::raytracer::color::Color;
use crate::raytracer::hittables::HitInfo;
use crate::raytracer::materials::MaterialPrimitive;
use crate::raytracer::scene::Scene;
use rand::prelude::ThreadRng;

pub struct Plain {}

impl MaterialPrimitive for Plain {
    fn get_color(
        &self, _hitinfo: &HitInfo, _scene: &Scene, _max_iter: usize, _rng: &mut ThreadRng,
    ) -> Color {
        Color::WHITE
    }
}
