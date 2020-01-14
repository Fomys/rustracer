use crate::raytracer::utils::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        2.0 * normal.normalized() * Vec3::dot(&normal.normalized(), &-self.direction)
            - -self.direction
    }

    pub fn normalized(&self) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction.normalized(),
        }
    }

    #[allow(dead_code)]
    pub fn normalize(&mut self) {
        self.direction.normalize();
    }
}
