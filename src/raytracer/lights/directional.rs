use crate::raytracer::color::Color;
use crate::raytracer::lights::Light;
use crate::raytracer::utils::Vec3;
use rand::prelude::ThreadRng;

pub struct Directional {
    direction: Vec3,
    color: Color,
    position: Vec3,
}

impl Directional {
    pub fn new(direction: Vec3, color: Color, distance: f32) -> Directional {
        Directional {
            direction,
            color,
            position: distance * direction,
        }
    }
}

impl Light for Directional {
    fn get_positions(&self, rng: &mut ThreadRng) -> Vec<Vec3> {
        vec![self.position]
    }

    fn get_color(&self, direction: Vec3) -> Color {
        self.color
    }
}
