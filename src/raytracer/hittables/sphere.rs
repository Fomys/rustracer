use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::ray::Ray;
use crate::raytracer::vec::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    r_2: f32,
    mincoord: Vec3,
    maxcoord: Vec3,
}

impl Sphere {
    #[allow(dead_code)]
    pub(crate) fn new(center: Vec3, radius: f32) -> Sphere {
        Sphere {
            center,
            radius: radius,
            r_2: radius * radius,
            mincoord: center - Vec3 { x: radius, y: radius, z: radius },
            maxcoord: center + Vec3 { x: radius, y: radius, z: radius },
        }
    }
}

impl Hittable for Sphere {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let oc = &rayon.origin - self.center;
        let a = Vec3::dot(&rayon.direction, &rayon.direction);
        let b = Vec3::dot(&rayon.direction, &oc);
        let c = Vec3::dot(&oc, &oc) - self.r_2;
        let delta = b * b - a * c;
        if delta >= 0.0 {
            let sqrt_delta = delta.sqrt();
            let distance = (-b - sqrt_delta) / a;
            if distance > 0.0 {
                let point = rayon.point_at(distance);
                return Some(HitInfo {
                    distance,
                    point,
                    normal: (point - &self.center),
                    rayon: *rayon,
                    position: (0.0, 0.0),
                });
            }
            let distance = (-b + sqrt_delta) / a;
            if distance > 0.0 {
                let point = rayon.point_at(distance);
                return Some(HitInfo {
                    distance,
                    point,
                    normal: (point - &self.center),
                    rayon: *rayon,
                    position: (0.0, 0.0),
                });
            }
        }
        None
    }

    fn extremums(&self) -> (Vec3, Vec3) {
        (self.mincoord, self.maxcoord)
    }
}