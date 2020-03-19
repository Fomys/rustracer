use crate::raytracer::utils::{Spectrum, Vec3};

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub const NONE: Ray = Ray {
        origin: Vec3::ZERO,
        direction: Vec3::ZERO,
    };

    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
