use crate::raytracer::hittables::hittable::{HitInfo, Hittable, Hittables};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::{Vec3, ZERO, ZERO_VEC3};

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
}

impl Triangle {
    #[allow(dead_code)]
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        let edge0 = b - a;
        let edge1 = c - a;
        let edge2 = c - b;
        let normal = edge0 ^ edge1;

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
        }
    }
}

impl Hittable for Triangle {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let denom = self.normal | rayon.direction;
        // Check if ray intersect triangle plane
        if denom.abs() >= ZERO {
            // Find intersection with plane
            let t = ((self.a - rayon.origin) | self.normal) / denom;
            if t >= 0.0 {
                let intersection = rayon.point_at(t);
                // Check if interestion is in triangle
                let vp0 = intersection - self.a;
                let c = self.edge0 ^ vp0;
                if (self.normal | c) < 0.0 {
                    return None;
                }
                let vp1 = intersection - self.c;
                let c = -self.edge1 ^ vp1;
                if (self.normal | c) < 0.0 {
                    return None;
                }
                let vp2 = intersection - self.b;
                let c = self.edge2 ^ vp2;
                if (self.normal | c) < 0.0 {
                    return None;
                }
                return Some(HitInfo {
                    distance: t,
                    normal: self.normal,
                    point: intersection,
                    rayon: *rayon,
                    position: ZERO_VEC3,
                });
            } else {
            }
        }

        None
    }

    fn get_type(&self) -> Hittables {
        Hittables::Triangle
    }

    fn to_triangle(&self) -> Option<Triangle> {
        Some(Triangle {
            a: self.a,
            b: self.b,
            c: self.c,
            normal: self.normal,
            edge0: self.edge0,
            edge1: self.edge1,
            edge2: self.edge2,
            edge0lenght_square: self.edge0lenght_square,
            edge1lenght_square: self.edge1lenght_square,
        })
    }
}
