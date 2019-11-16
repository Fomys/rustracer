use crate::raytracer::vec3::Vec3;
#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn point_at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}

#[cfg(test)]
mod tests {
    use crate::raytracer::ray::Ray;
    use crate::raytracer::vec3::Vec3;

    #[test]
    fn point_at() {
        assert_eq!(
            Ray {
                origin: Vec3 { x: 1f32, y: 1f32, z: 1f32 },
                direction: Vec3 { x: 1f32, y: 1f32, z: 1f32 },
            }.point_at(1f32),
            Vec3 { x: 2f32, y: 2f32, z: 2f32 }
        );
    }
}
