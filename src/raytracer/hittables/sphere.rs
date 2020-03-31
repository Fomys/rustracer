use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::movements::movement::{Movement, MovementPrimitive};
use crate::raytracer::ray::Ray;
use crate::raytracer::textures::Texture;
use crate::raytracer::utils::Vec3;
use std::sync::Arc;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub r_2: f32,
    pub movements: Movement,
    texture: Arc<dyn Texture>,
}

impl Sphere {
    pub fn new(
        center: Vec3, radius: f32, movements: Movement, texture: Arc<dyn Texture>,
    ) -> Sphere {
        Sphere {
            center,
            radius,
            r_2: radius * radius,
            movements,
            texture,
        }
    }
}

impl Hittable for Sphere {
    fn compute_hit(&self, rayon: &Ray) -> HitInfo {
        let oc = rayon.origin - self.center;
        let a = rayon.direction | rayon.direction;
        let b = rayon.direction | oc;
        let c = (oc | oc) - self.r_2;
        let delta = b * b - a * c;
        if delta >= 0.0 {
            let sqrt_delta = delta.sqrt();
            let distance = (-b - sqrt_delta) / a;
            if distance > 0.0 {
                let point = rayon.point_at(distance);
                return HitInfo {
                    distance,
                    point,
                    normal: (point - self.center),
                    rayon: *rayon,
                    position: Vec3::ZERO,
                    texture: Some(self.texture.clone()),
                };
            }
            let distance = (-b + sqrt_delta) / a;
            if distance > 0.0 {
                let point = rayon.point_at(distance);
                return HitInfo {
                    distance,
                    point,
                    normal: (point - self.center),
                    rayon: *rayon,
                    position: Vec3::ZERO,
                    texture: Some(self.texture.clone()),
                };
            }
        }
        HitInfo::NONE
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
                    self.r_2 = self.radius * self.radius;
                }
                MovementPrimitive::Cycle(_) => {
                    //Nothing here
                }
            }
        }
    }

    fn get_extremums(&self) -> (Vec3, Vec3) {
        (
            self.center
                - self.radius
                    * Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
            self.center
                + self.radius
                    * Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
        )
    }
}
