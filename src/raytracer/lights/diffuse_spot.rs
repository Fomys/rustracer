use crate::raytracer::color::{Color, BLACK, WHITE};
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::vec::Vec3;

pub struct DiffuseSpot {
    pub color: Color,
    pub position: Vec3,
    pub direction: Vec3,
    pub angle: f32,
    pub max_angle: f32,
    pub power: f32,
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
            power: power,
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
            self.color * self.power / (0.7 * direction.length() + 1.5 * direction.squared_length())
        } else if angle < self.max_angle {
            (self.max_angle - angle) / (self.max_angle - self.angle) * self.color * self.power
                / (0.7 * direction.length() + 1.5 * direction.squared_length())
        } else {
            BLACK
        }
    }
}
