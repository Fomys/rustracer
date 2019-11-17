use crate::raytracer::vec3::Vec3;
use crate::raytracer::hittable::{Hittable, HitInfo};
use crate::raytracer::ray::Ray;
use crate::raytracer::color::Color;

pub struct Sphere {
    pub center: Vec3,
    pub color: Color,
    pub radius: f32,
    pub reflect_power_value: f32,
}

impl Hittable for Sphere {
    fn compute_hit(&self, ray: &Ray) -> Option<HitInfo> {
        let rayon = ray.normalized();

        let oc = &rayon.origin - self.center;
        let a = Vec3::dot(&rayon.direction, &rayon.direction);
        let b = Vec3::dot(&rayon.direction, &oc);
        let c = Vec3::dot(&oc, &oc) - self.radius * self.radius;
        let delta = b * b - a * c;
        if delta >= 0.0 {
            let sqrt_delta = delta.sqrt();
            let distance = (-b - sqrt_delta) / a;
            if distance > 0.0 {
                let point = rayon.point_at(distance);
                return Some(HitInfo{distance, point, normal: (point - &self.center)})
            }
            let distance = (-b + sqrt_delta) / a;
            if distance > 0.0 {
                let point = rayon.point_at(distance);
                return Some(HitInfo{distance, point, normal: (point - &self.center)})
            }
        }
        None
    }

    fn get_intersect(&self, rayon: &Ray) -> Option<Vec3> {
        None
    }

    fn get_color(&self, rayon: &Ray) -> Color {
        self.color
    }

    fn get_reflect_factor(&self, point:Vec3) -> f32 {self.reflect_power_value}
}