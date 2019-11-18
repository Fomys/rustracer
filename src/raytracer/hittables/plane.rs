use crate::raytracer::vec3::Vec3;
use crate::raytracer::hittables::hittable::{Hittable, HitInfo};
use crate::raytracer::ray::Ray;

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3
}

impl Hittable for Plane {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let denom= Vec3::dot(&self.normal, &rayon.direction);
        if denom.abs() >= 0.001 {
            let t = Vec3::dot(&(self.origin - &rayon.origin), &self.normal) / denom;
            if t >= 0.0 {
                return Some(HitInfo {
                    distance: t,
                    normal: self.normal,
                    point: rayon.point_at(t),
                    rayon: *rayon,
                })
            }
        }
        None
    }
}
