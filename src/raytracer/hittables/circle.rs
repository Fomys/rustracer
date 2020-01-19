use crate::raytracer::hittables::hittable::{HitInfo, Hittable, Hittables};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::{Vec3, ZERO, ZERO_VEC3};

pub struct Circle {
    center: Vec3,
    radius: f32,
    radius_2: f32,
    // radius^2
    normal: Vec3,
}

impl Circle {
    pub fn new(center: Vec3, radius: f32, normal: Vec3) -> Circle {
        Circle {
            center,
            radius,
            normal,
            radius_2: radius.powi(2),
        }
    }
}

impl Hittable for Circle {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let denom = self.normal | rayon.direction;
        if denom.abs() >= ZERO {
            let t = ((self.center - rayon.origin) | self.normal) / denom;
            if t >= 0.0 {
                let intersect_point = rayon.point_at(t);
                if (self.center - intersect_point).squared_length() < self.radius_2 {
                    return Some(HitInfo {
                        distance: t,
                        normal: self.normal,
                        point: intersect_point,
                        rayon: *rayon,
                        position: ZERO_VEC3,
                    });
                }
            }
        }
        None
    }

    fn get_type(&self) -> Hittables {
        Hittables::Circle
    }

    fn to_circle(&self) -> Option<Circle> {
        Some(Circle {
            radius_2: self.radius_2,
            center: self.center,
            radius: self.radius,
            normal: self.normal,
        })
    }
}
