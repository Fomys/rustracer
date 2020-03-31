use crate::raytracer::color::Color;
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::Vec3;
use rand::prelude::ThreadRng;

pub struct Omnidirectional {
    _color: Color,
    position: Vec3,
    _power: f32,
    color_power: Color,
}

impl Omnidirectional {
    pub fn new(color: Color, position: Vec3, power: f32) -> Omnidirectional {
        Omnidirectional {
            _color: color,
            position,
            _power: power,
            color_power: color * power,
        }
    }
}

impl Light for Omnidirectional {
    fn get_positions(&self, _rng: &mut ThreadRng) -> Vec<Vec3> {
        vec![self.position]
    }
    fn get_color(&self, direction: Vec3) -> Color {
        self.color_power
            * (1.0 / (1.0 + 0.7 * direction.length() + 1.8 * direction.squared_length()))
        //self.color_power / (0.7 * direction.length() + 1.5 * direction.squared_length())
    }
}
