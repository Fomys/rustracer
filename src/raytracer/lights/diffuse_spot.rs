use crate::raytracer::color::{Color, BLACK};
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::Vec3;

pub struct DiffuseSpot {
    pub color: Color,
    pub position: Vec3,
    pub direction: Vec3,
    pub angle: f32,
    pub max_angle: f32,
    pub power: f32,
    pub temp_1: f32,
    pub temp_2: Color,
    pub temp_21: Color,
}

impl DiffuseSpot {
    pub fn new(
        color: Color, position: Vec3, direction: Vec3, angle: f32, max_angle: f32, power: f32,
    ) -> DiffuseSpot {
        DiffuseSpot {
            color,
            position,
            direction: direction.normalized(),
            angle: angle.to_radians(),
            max_angle: max_angle.to_radians(),
            power,
            temp_1: max_angle.to_radians() - angle.to_radians(),
            temp_2: color * power,
            temp_21: 1.0 / (max_angle.to_radians() - angle.to_radians()) * color * power,
        }
    }
}

impl Light for DiffuseSpot {
    fn get_positions(&self, rng: &mut rand::XorShiftRng) -> Vec<Vec3> {
        vec![self.position]
    }
    fn get_color(&self, direction: &Vec3) -> Color {
        let angle = (Vec3::dot(&self.direction, &direction) / direction.length())
            .acos()
            .abs();

        if angle < self.angle {
            self.temp_2 / (0.7 * direction.length() + 1.5 * direction.squared_length())
        } else if angle < self.max_angle {
            (self.max_angle - angle) * self.temp_21
                / (0.7 * direction.length() + 1.5 * direction.squared_length())
        } else {
            BLACK
        }
    }
}
