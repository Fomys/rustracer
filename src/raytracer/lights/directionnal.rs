use crate::raytracer::color::{Color, BLACK, WHITE};
use crate::raytracer::lights::light::Light;
use crate::raytracer::utils::vec::Vec3;

pub struct Directional {
    pub color: Color,
    pub position: Vec3,
    pub direction: Vec3,
    pub angle: f32,
}

impl Directional {
    pub fn new(color: Color, position: Vec3, direction: Vec3, angle: f32) -> Directional {
        Directional {
            color,
            position,
            direction: direction.normalized(),
            angle: angle.to_radians(),
        }
    }
}

impl Light for Directional {
    fn get_color(&self, direction: &Vec3) -> Color {
        if (Vec3::dot(&self.direction, &direction) / direction.length())
            .acos()
            .abs()
            > self.angle
        {
            BLACK
        } else {
            self.color
        }
    }
    fn get_position(&self) -> Vec3 {
        self.position
    }
}
