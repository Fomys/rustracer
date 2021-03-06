use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::movements::movement::{Movement, MovementPrimitive};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::Vec3;

pub struct Cylinder {
    origin: Vec3,
    direction: Vec3,
    _radius: f32,
    radius_2: f32,
    zy_yz: f32,
    xz_zx: f32,
    yx_xy: f32,
    movements: Movement,
}

impl Cylinder {
    pub fn new(origin: Vec3, direction: Vec3, radius: f32, movements: Movement) -> Cylinder {
        let direction = direction.normalized();
        Cylinder {
            origin,
            direction,
            _radius: radius,
            radius_2: radius * radius,
            zy_yz: origin.z * direction.y - origin.y * direction.z,
            xz_zx: origin.x * direction.z - origin.z * direction.x,
            yx_xy: origin.y * direction.x - origin.x * direction.y,
            movements,
        }
    }
    fn update_temp_var(&mut self) {
        self.radius_2 = self._radius * self._radius;
        self.zy_yz = self.origin.z * self.direction.y - self.origin.y * self.direction.z;
        self.xz_zx = self.origin.x * self.direction.z - self.origin.z * self.direction.x;
        self.yx_xy = self.origin.y * self.direction.x - self.origin.x * self.direction.y;
    }
}

impl Hittable for Cylinder {
    fn compute_hit(&self, rayon: &Ray) -> HitInfo {
        let rayon_norm = rayon.normalized();
        let t_1 =
            rayon_norm.direction.y * self.direction.z - rayon_norm.direction.z * self.direction.y;
        let t_2 =
            rayon_norm.direction.z * self.direction.x - rayon_norm.direction.x * self.direction.z;
        let t_3 =
            rayon_norm.direction.x * self.direction.y - rayon_norm.direction.y * self.direction.x;
        let t_4 = rayon_norm.origin.y * self.direction.z - rayon_norm.origin.z * self.direction.y
            + self.zy_yz;
        let t_5 = rayon_norm.origin.z * self.direction.x - rayon_norm.origin.x * self.direction.z
            + self.xz_zx;
        let t_6 = rayon_norm.origin.x * self.direction.y - rayon_norm.origin.y * self.direction.x
            + self.yx_xy;
        let a_2 = (t_1 * t_1 + t_2 * t_2 + t_3 * t_3) * 2.0; // 2*a
        let b: f32 = 2.0 * (t_1 * t_4 + t_2 * t_5 + t_3 * t_6);
        let c = t_4 * t_4 + t_5 * t_5 + t_6 * t_6 - self.radius_2;
        let delta = b * b - 2.0 * a_2 * c; // a_2 = 2 * a
        if delta > 0.0 {
            let sqrt_delta = delta.sqrt();
            let distance = (-b - sqrt_delta) / a_2;
            if distance > 0.0 {
                let point = rayon_norm.point_at(distance);
                return HitInfo {
                    distance,
                    point,
                    normal: (point - self.origin)
                        - (self.direction | (point - self.origin)) * self.direction,
                    rayon: *rayon,
                    position: Vec3::ZERO,
                };
            }
            let distance = (-b + sqrt_delta) / a_2; // a_2 = 2 * a
            if distance > 0.0 {
                let point = rayon_norm.point_at(distance);
                return HitInfo {
                    distance,
                    point,
                    normal: (point - self.origin)
                        - (self.direction | (point - self.origin)) * self.direction,
                    rayon: *rayon,
                    position: Vec3::ZERO,
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
                    self.origin += distance;
                    self.update_temp_var();
                }
                MovementPrimitive::Scale(scale) => {
                    self._radius *= scale;
                    self.update_temp_var();
                }
                MovementPrimitive::Cycle(_) => { //Nothing here }
                }
            }
        }
    }
}
