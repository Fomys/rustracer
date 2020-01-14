use crate::raytracer::color::Color;
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::Vec3;

pub struct Omnidirectional {
    pub color: Color,
    pub position: Vec3,
    pub power: f32,
}

impl Light for Omnidirectional {
    fn get_positions(&self, rng: &mut rand::XorShiftRng) -> Vec<Vec3> {
        vec![self.position]
    }
    fn get_color(&self, direction: &Vec3) -> Color {
        self.color * self.power / (0.7 * direction.length() + 1.5 * direction.squared_length())
    }
}
