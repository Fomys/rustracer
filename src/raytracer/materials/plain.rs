use crate::raytracer::color::{Color, WHITE};
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::materials::material::MaterialPrimitive;
use crate::raytracer::scene::Scene;
use rand::XorShiftRng;

pub struct Plain {}

impl MaterialPrimitive for Plain {
    fn get_color(
        &self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize, rng: &mut XorShiftRng,
    ) -> Color {
        WHITE
    }
}
