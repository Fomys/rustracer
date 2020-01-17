use rand::Rng;

use crate::raytracer::color::Color;
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::Vec3;
use crate::raytracer::utils::SOURCE_PER_SURFACE;

pub struct Rectangle {
    color: Color,
    origin: Vec3,
    dir1: Vec3,
    dir2: Vec3,
    power: f32,
    color_power: Color,
}

impl Rectangle {
    pub fn new(color: Color, origin: Vec3, dir1: Vec3, dir2: Vec3, power: f32) -> Rectangle {
        Rectangle {
            color,
            origin,
            dir1,
            dir2,
            power,
            color_power: color * power,
        }
    }
}

impl Light for Rectangle {
    fn get_positions(&self, rng: &mut rand::XorShiftRng) -> Vec<Vec3> {
        let mut pos: Vec<Vec3> = vec![];
        for i in 0..SOURCE_PER_SURFACE + 1 {
            for j in 0..SOURCE_PER_SURFACE + 1 {
                pos.push(self.origin + self.dir1 * rng.next_f32() + self.dir2 * rng.next_f32());
            }
        }
        pos
    }

    fn get_color(&self, direction: &Vec3) -> Color {
        self.color_power / (0.7 * direction.length() + 1.5 * direction.squared_length())
    }
}
