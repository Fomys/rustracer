use crate::raytracer::color::{Color, BLACK};
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::Vec3;

pub struct Spot {
    pub color: Color,
    pub position: Vec3,
    pub direction: Vec3,
    pub angle: f32,
    pub power: f32,
}

impl Spot {
    pub fn new(color: Color, position: Vec3, direction: Vec3, angle: f32, power: f32) -> Spot {
        Spot {
            color,
            position,
            direction: direction.normalized(),
            angle: angle.to_radians(),
            power: power,
        }
    }
}

impl Light for Spot {
    fn get_positions(&self, rng: &mut rand::XorShiftRng) -> Vec<Vec3> {
        vec![self.position]
    }
    fn get_color(&self, direction: &Vec3) -> Color {
        if (Vec3::dot(&self.direction, &direction) / direction.length())
            .acos()
            .abs()
            > self.angle
        {
            BLACK
        } else {
            self.color * self.power / (0.7 * direction.length() + 1.5 * direction.squared_length())
        }
    }
}
