use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::movements::movement::Movement;
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::{Vec3, ZERO, ZERO_VEC3};

pub struct Plane {
    origin: Vec3,
    normal: Vec3,
    pub movements: Movement,
}

impl Plane {
    #[allow(dead_code)]
    pub fn new(origin: Vec3, normal: Vec3, movements: Movement) -> Plane {
        Plane {
            origin,
            normal,
            movements,
        }
    }
}

impl Hittable for Plane {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let denom = self.normal | rayon.direction;
        if denom.abs() >= ZERO {
            let t = ((self.origin - rayon.origin) | self.normal) / denom;
            if t >= 0.0 {
                return Some(HitInfo {
                    distance: t,
                    normal: self.normal,
                    point: rayon.point_at(t),
                    rayon: *rayon,
                    position: ZERO_VEC3,
                });
            }
        }
        None
    }
}
