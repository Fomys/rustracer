use crate::raytracer::consts;
use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::ray::Ray;
use crate::raytracer::vec::Vec3;

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    normal: Vec3,
    edge0: Vec3,
    edge1: Vec3,
    edge2: Vec3,
    edge0lenght_square: f32,
    edge1lenght_square: f32,
    mincoord: Vec3,
    maxcoord: Vec3,
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        let edge0 = b - a;
        let edge1 = c - a;
        let edge2 = c - b;
        let normal = Vec3::cross_product(&edge0, &edge1);
        Triangle {
            a,
            b,
            c,
            normal,
            edge0,
            edge1,
            edge2,
            edge0lenght_square: edge0.length().powf(2.0),
            edge1lenght_square: edge1.length().powf(2.0),
            mincoord: Vec3::min(a, Vec3::min(b, c)),
            maxcoord: Vec3::max(a, Vec3::max(b, c)),
        }
    }
}

impl Hittable for Triangle {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let denom = Vec3::dot(&self.normal, &rayon.direction);
        if denom.abs() >= consts::ZERO {
            // Find intersection with plane
            let t = Vec3::dot(&(self.a - &rayon.origin), &self.normal) / denom;
            if t >= 0.0 {
                let intersection = rayon.point_at(t);
                // Check if interestion is in triangle
                let vp0 = intersection - self.a;
                let c = Vec3::cross_product(&self.edge0, &vp0);
                if Vec3::dot(&self.normal, &c) < 0.0 {
                    return None;
                }
                let vp1 = intersection - self.c;
                let c = Vec3::cross_product(&(-self.edge1), &vp1);
                if Vec3::dot(&self.normal, &c) < 0.0 {
                    return None;
                }
                let vp2 = intersection - self.b;
                let c = Vec3::cross_product(&self.edge2, &vp2);
                if Vec3::dot(&self.normal, &c) < 0.0 {
                    return None;
                }
                return Some(HitInfo {
                    distance: t,
                    normal: self.normal,
                    point: intersection,
                    rayon: *rayon,
                    position: ((Vec3::dot(&(intersection - self.a), &self.edge0) / self.edge0lenght_square),
                               (Vec3::dot(&(intersection - self.a), &self.edge1) / self.edge1lenght_square)),
                });
            }
        }

        None
    }

    fn extremums(&self) -> (Vec3, Vec3) {
        (self.mincoord, self.maxcoord)
    }
}