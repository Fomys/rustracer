use crate::raytracer::color::{Color, BLACK};
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::Vec3;

pub struct Spot {
    _color: Color,
    position: Vec3,
    direction: Vec3,
    angle: f32,
    _power: f32,
    color_power: Color,
}

impl Spot {
    #[allow(dead_code)]
    pub fn new(color: Color, position: Vec3, direction: Vec3, angle: f32, power: f32) -> Spot {
        Spot {
            _color: color,
            position,
            direction: direction.normalized(),
            angle: angle.to_radians(),
            _power: power,
            color_power: color * power,
        }
    }
}

impl Light for Spot {
    fn get_positions(&self, _rng: &mut rand::XorShiftRng) -> Vec<Vec3> {
        vec![self.position]
    }
    fn get_color(&self, direction: Vec3) -> Color {
        if ((self.direction | direction) / direction.length())
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
