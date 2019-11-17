use crate::raytracer::vec3::Vec3;
use crate::raytracer::ray::Ray;
use crate::raytracer::hittables::hittable::{Hittable, HitInfo};
use crate::raytracer::materials::material::Material;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Box<dyn Material>,
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
    fn get_material(&self) -> &Box<dyn Material> { &self.material }
}