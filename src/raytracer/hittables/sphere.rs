use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::movements::movement::{Movement, MovementPrimitive};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::{Vec3, ZERO_VEC3};

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub r_2: f32,
    pub movements: Movement,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, movements: Movement) -> Sphere {
        Sphere {
            center,
            radius,
            r_2: radius * radius,
            movements,
        }
    }
}

impl Hittable for Sphere {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let oc = rayon.origin - self.center;
        let a = rayon.direction | rayon.direction;
        let b = rayon.direction | oc;
        let c = (oc | oc) - self.r_2;
        let delta = b.powi(2) - a * c;
        if delta >= 0.0 {
            let sqrt_delta = delta.sqrt();
            let distance = (-b - sqrt_delta) / a;
            if distance > 0.0 {
                let point = rayon.point_at(distance);
                return Some(HitInfo {
                    distance,
                    point,
                    normal: (point - self.center),
                    rayon: *rayon,
                    position: ZERO_VEC3,
                });
            }
            let distance = (-b + sqrt_delta) / a;
            if distance > 0.0 {
                let point = rayon.point_at(distance);
                return Some(HitInfo {
                    distance,
                    point,
                    normal: (point - self.center),
                    rayon: *rayon,
                    position: ZERO_VEC3,
                });
            }
        }
        None
    }

    fn next_pos(&mut self) {
        let movements = self.movements.next_movements();
        for movement in movements {
            match movement {
                MovementPrimitive::Translation(distance) => {
                    self.center += distance;
                }
                MovementPrimitive::Scale(scale) => {
                    self.radius *= scale;
                    self.r_2 = self.radius.powi(2);
                }
                MovementPrimitive::Cycle(_) => { //Nothing here }
                }
            }
        }
    }
}
