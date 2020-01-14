use crate::raytracer::color::Color;
use crate::raytracer::utils::Vec3;

pub trait Light: Sync + Send {
    fn get_positions(&self, rng: &mut rand::XorShiftRng) -> Vec<Vec3>;
    fn get_color(&self, direction: &Vec3) -> Color;
}
