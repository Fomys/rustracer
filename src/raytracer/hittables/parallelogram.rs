use crate::raytracer::hittables::{HitInfo, Hittable};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::Vec3;

pub struct Square {
    origin: Vec3,
    edge1: Vec3,
    edge2: Vec3,
    normal: Vec3,
}

impl Square {
    pub fn new(origin: Vec3, edge1: Vec3, edge2: Vec3) -> Square {
        Square {
            origin,
            edge1,
            edge2,
            normal: (edge1 ^ edge2).normalized(),
        }
    }
}

impl Hittable for Square {
    fn compute_hit(&self, rayon: &Ray) -> HitInfo {
        // Intersection rayon/plan
        let denom = self.normal | rayon.direction;
        if denom.abs() >= 0.0 {
            let t = ((self.origin - rayon.origin) | self.normal) / denom;
            let p = rayon.point_at(t);
            if t > 0.0 {
                let a = self.edge1 | (p - self.origin);
                if (0.0 < a) & (a < self.edge1.squared_length()) {
                    let b = self.edge2 | (p - self.origin);
                    if (0.0 < b) & (b < self.edge2.squared_length()) {
                        return HitInfo {
                            distance: t,
                            normal: self.normal,
                            point: rayon.point_at(t),
                            rayon: *rayon,
                            position: Vec3::ZERO,
                        };
                    }
                }
            }
        }
        HitInfo::NONE
    }
}
