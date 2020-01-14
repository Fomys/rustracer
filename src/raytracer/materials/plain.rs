use rand::XorShiftRng;

use crate::raytracer::color::{Color, WHITE};
use crate::raytracer::hittables::HitInfo;
use crate::raytracer::materials::MaterialPrimitive;
use crate::raytracer::scene::Scene;

pub struct Plain {}

impl MaterialPrimitive for Plain {
    fn get_color(
        &self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize, rng: &mut XorShiftRng,
    ) -> Color {
        WHITE
    }
}
