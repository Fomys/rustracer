use crate::raytracer::color::Color;
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::vec::Vec3;

pub struct Omnidirectional {
    pub color: Color,
    pub position: Vec3,
}

impl Light for Omnidirectional {
    fn get_color(&self, direction: &Vec3) -> Color {
        self.color
    }
    fn get_position(&self) -> Vec3 {
        self.position
    }
}
