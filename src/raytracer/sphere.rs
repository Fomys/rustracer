use crate::raytracer::vec3::Vec3;
use crate::raytracer::hittable::Hittable;
use crate::raytracer::ray::Ray;
use crate::raytracer::color::Color;

pub struct Sphere {
    pub center: Vec3,
    pub color: Color,
    pub radius: f32,
}

impl Hittable for Sphere {
    fn compute_hit(&self, rayon: &Ray) -> Option<f32> {
        let oc = &rayon.origin - self.center;
        let a = Vec3::dot(&rayon.direction, &rayon.direction);
        let b = Vec3::dot(&rayon.direction, &oc);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let delta = b * b - a * c;
        if delta > 0.0 {
            let sqrt_delta = delta.sqrt() / a;
            let b_ = -b / a;
            if b_ + sqrt_delta > 0.0 {
                return Some(b_ + sqrt_delta)
            } else if b_ - sqrt_delta > 0.0 {
                return Some(b_ - sqrt_delta)
            }
        }
        None
    }

    fn get_intesect(&self, rayon: &Ray) -> Option<Vec3> {
        None
    }

    fn get_color(&self, rayon: &Ray) -> Color {
        self.color
    }
}