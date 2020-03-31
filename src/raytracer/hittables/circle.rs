use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::movements::movement::{Movement, MovementPrimitive};
use crate::raytracer::ray::Ray;
use crate::raytracer::textures::Texture;
use crate::raytracer::utils::{Vec3, ZERO};
use std::sync::Arc;

pub struct Circle {
    center: Vec3,
    _radius: f32,
    radius_2: f32,
    // radius^2
    normal: Vec3,
    movement: Movement,
    texture: Arc<dyn Texture>,
}

impl Circle {
    pub fn new(
        center: Vec3, radius: f32, normal: Vec3, movement: Movement, texture: Arc<dyn Texture>,
    ) -> Circle {
        Circle {
            center,
            _radius: radius,
            normal,
            radius_2: radius * radius,
            movement,
            texture,
        }
    }
}

impl Hittable for Circle {
    fn compute_hit(&self, rayon: &Ray) -> HitInfo {
        let denom = self.normal | rayon.direction;
        if denom.abs() >= ZERO {
            let t = ((self.center - rayon.origin) | self.normal) / denom;
            if t >= 0.0 {
                let intersect_point = rayon.point_at(t);
                if (self.center - intersect_point).squared_length() < self.radius_2 {
                    return HitInfo {
                        distance: t,
                        normal: self.normal,
                        point: intersect_point,
                        rayon: *rayon,
                        position: Vec3 {
                            x: (intersect_point - self.center).length(),
                            y: 0.0,
                            z: 0.0,
                        }, // distance au centre
                        texture: Some(self.texture.clone()),
                    };
                }
            }
        }
        HitInfo::NONE
    }

    fn next_pos(&mut self) {
        let movements = self.movement.next_movements();
        for movement in movements {
            match movement {
                MovementPrimitive::Translation(distance) => {
                    self.center += distance;
                }
                MovementPrimitive::Scale(scale) => {
                    self._radius *= scale;
                    self.radius_2 = self.radius_2 * self.radius_2;
                }
                MovementPrimitive::Cycle(_) => { //Nothing here }
                }
            }
        }
    }

    fn get_extremums(&self) -> (Vec3, Vec3) {
        (
            self.center
                - self._radius
                    * Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
            self.center
                + self._radius
                    * Vec3 {
                        x: 1.0,
                        y: 1.0,
                        z: 1.0,
                    },
        )
    }
}
