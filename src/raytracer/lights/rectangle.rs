use crate::raytracer::color::Color;
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::consts::SOURCE_PER_SURFACE;
use crate::raytracer::utils::vec::Vec3;
use rand::Rng;

pub struct Rectangle {
    pub color: Color,
    pub origin: Vec3,
    pub dir1: Vec3,
    pub dir2: Vec3,
    pub power: f32,
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
        self.color * self.power / (0.7 * direction.length() + 1.5 * direction.squared_length())
    }
}
