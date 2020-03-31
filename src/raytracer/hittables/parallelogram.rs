use crate::raytracer::hittables::{HitInfo, Hittable};
use crate::raytracer::ray::Ray;
use crate::raytracer::textures::Texture;
use crate::raytracer::utils::Vec3;
use std::sync::Arc;

pub struct Parallelogram {
    origin: Vec3,
    edge1: Vec3,
    edge2: Vec3,
    normal: Vec3,
    texture: Arc<dyn Texture>,
}

impl Parallelogram {
    pub fn new(origin: Vec3, edge1: Vec3, edge2: Vec3, texture: Arc<dyn Texture>) -> Parallelogram {
        Parallelogram {
            origin,
            edge1,
            edge2,
            normal: (edge1 ^ edge2).normalized(),
            texture,
        }
    }
}

impl Hittable for Parallelogram {
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
                        let point = rayon.point_at(t);
                        let p = (point - self.origin);
                        return HitInfo {
                            distance: t,
                            normal: self.normal,
                            point: point,
                            rayon: *rayon,
                            position: Vec3 {
                                x: p | self.edge1,
                                y: p | self.edge2,
                                z: 0.0,
                            }, // X, Y coordonnées dans le repère edge1, edge
                            texture: Some(self.texture.clone()),
                        };
                    }
                }
            }
        }
        HitInfo::NONE
    }

    fn get_extremums(&self) -> (Vec3, Vec3) {
        (
            self.origin
                .min(&(self.origin + self.edge1))
                .min(&(self.origin + self.edge2)),
            self.origin
                .max(&(self.origin + self.edge1))
                .max(&(self.origin + self.edge2)),
        )
    }
}
