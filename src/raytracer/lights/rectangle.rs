use rand::Rng;

use crate::raytracer::color::Color;
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::Vec3;
use crate::raytracer::utils::SOURCE_PER_SURFACE;
use rand::prelude::ThreadRng;

pub struct Rectangle {
    _color: Color,
    origin: Vec3,
    dir1: Vec3,
    dir2: Vec3,
    _power: f32,
    color_power: Color,
}

impl Rectangle {
    pub fn new(color: Color, origin: Vec3, dir1: Vec3, dir2: Vec3, power: f32) -> Rectangle {
        Rectangle {
            _color: color,
            origin,
            dir1,
            dir2,
            _power: power,
            color_power: color * power,
        }
    }
}

impl Light for Rectangle {
    fn get_positions(&self, rng: &mut ThreadRng) -> Vec<Vec3> {
        let mut pos: Vec<Vec3> = vec![];
        for _ in 0..=SOURCE_PER_SURFACE {
            for _ in 0..=SOURCE_PER_SURFACE {
                pos.push(self.origin + rng.gen::<f32>() * self.dir1 + rng.gen::<f32>() * self.dir2);
            }
        }
        pos
    }

    fn get_color(&self, direction: Vec3) -> Color {
        self.color_power / (0.7 * direction.length() + 1.5 * direction.squared_length())
    }
}
