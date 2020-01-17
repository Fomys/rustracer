use crate::raytracer::color::{Color, BLACK};
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::Vec3;

pub struct Spot {
    color: Color,
    position: Vec3,
    direction: Vec3,
    angle: f32,
    power: f32,
    color_power: Color,
}

impl Spot {
    pub fn new(color: Color, position: Vec3, direction: Vec3, angle: f32, power: f32) -> Spot {
        Spot {
            color,
            position,
            direction: direction.normalized(),
            angle: angle.to_radians(),
            power,
            color_power: color * power,
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
            self.color_power / (0.7 * direction.length() + 1.5 * direction.squared_length())
        }
    }
}
