use crate::raytracer::vec::Vec3;
use crate::raytracer::ray::Ray;

pub struct Camera {
    pub position: Vec3,
    pub left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, direction: Vec3, fov_horiontal: f32, fov_vertical: f32) -> Camera{
        let left_corner = Vec3 {
            x: -fov_horiontal / 2.0,
            y: -fov_vertical / 2.0,
            z: -1.0
        };
        let horizontal = Vec3 {
            x: fov_horiontal,
            y: 0.0,
            z: 0.0
        };
        let vertical = Vec3 {
            x: 0.0,
            y: fov_vertical,
            z: 0.0
        };
        Camera {position, left_corner, horizontal, vertical}
    }

    pub fn get_ray(&self, x: f32, y: f32) -> Ray {
        Ray {
            origin: self.position,
            direction: self.left_corner + self.horizontal * x + self.vertical * y,
        }.normalized()
    }
}