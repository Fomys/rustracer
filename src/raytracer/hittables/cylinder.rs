use crate::raytracer::hittables::hittable::{HitInfo, Hittable, Hittables};
use crate::raytracer::hittables::plane::Plane;
use crate::raytracer::hittables::sphere::Sphere;
use crate::raytracer::hittables::triangle::Triangle;
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::vec::Vec3;

pub struct Cylinder {
    origin: Vec3,
    direction: Vec3,
    radius: f32,
    radius_2: f32,
    mincoord: Vec3,
    maxcoord: Vec3,
}

impl Cylinder {
    pub fn new(origin: Vec3, direction: Vec3, rayon: f32) -> Cylinder {
        Cylinder {
            origin,
            direction: direction.normalized(),
            radius: rayon,
            radius_2: rayon.powi(2),
            mincoord: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            maxcoord: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        }
    }
}

impl Hittable for Cylinder {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let rayon_norm = rayon.normalized();
        let t_1 =
            rayon_norm.direction.y * self.direction.z - rayon_norm.direction.z * self.direction.y;
        let t_2 =
            rayon_norm.direction.z * self.direction.x - rayon_norm.direction.x * self.direction.z;
        let t_3 =
            rayon_norm.direction.x * self.direction.y - rayon_norm.direction.y * self.direction.x;
        let t_4 = rayon_norm.origin.y * self.direction.z - rayon_norm.origin.z * self.direction.y
            + self.origin.z * self.direction.y
            - self.origin.y * self.direction.z;
        let t_5 = rayon_norm.origin.z * self.direction.x - rayon_norm.origin.x * self.direction.z
            + self.origin.x * self.direction.z
            - self.origin.z * self.direction.x;
        let t_6 = rayon_norm.origin.x * self.direction.y - rayon_norm.origin.y * self.direction.x
            + self.origin.y * self.direction.x
            - self.origin.x * self.direction.y;
        let a_2 = (t_1.powi(2) + t_2.powi(2) + t_3.powi(2)) * 2.0; // 2*a
        let b = 2.0 * (t_1 * t_4 + t_2 * t_5 + t_3 * t_6);
        let c = t_4.powi(2) + t_5.powi(2) + t_6.powi(2) - self.radius_2;
        let delta = b.powi(2) - 2.0 * a_2 * c; // a_2 = 2 * a
        if delta > 0.0 {
            let sqrt_delta = delta.sqrt();
            let distance = (-b - sqrt_delta) / a_2;
            if distance > 0.0 {
                let point = rayon_norm.point_at(distance);
                return Some(HitInfo {
                    distance,
                    point,
                    normal: (point - self.origin)
                        - self.direction * (Vec3::dot(&self.direction, &(point - self.origin))),
                    rayon: *rayon,
                    position: Vec3::zero(),
                });
            }
            let distance = (-b + sqrt_delta) / a_2; // a_2 = 2 * a
            if distance > 0.0 {
                let point = rayon_norm.point_at(distance);
                return Some(HitInfo {
                    distance,
                    point,
                    normal: (point - self.origin)
                        - self.direction * (Vec3::dot(&self.direction, &(point - self.origin))),
                    rayon: *rayon,
                    position: Vec3::zero(),
                });
            }
        }
        None
    }

    fn extremum(&self) -> (Vec3, Vec3) {
        (self.mincoord, self.maxcoord)
    }

    fn get_type(&self) -> Hittables {
        Hittables::Cylinder
    }

    fn to_sphere(&self) -> Option<Sphere> {
        None
    }

    fn to_triangle(&self) -> Option<Triangle> {
        None
    }

    fn to_plane(&self) -> Option<Plane> {
        None
    }

    fn to_cylinder(&self) -> Option<Cylinder> {
        Some(Cylinder {
            origin: self.origin,
            direction: self.direction,
            radius: self.radius,
            radius_2: self.radius_2,
            mincoord: self.mincoord,
            maxcoord: self.maxcoord,
        })
    }
}
