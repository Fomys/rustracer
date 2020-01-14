use crate::raytracer::hittables::hittable::{HitInfo, Hittable, Hittables};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::{Vec3, ZERO};

pub struct Plane {
    origin: Vec3,
    normal: Vec3,
    mincoord: Vec3,
    maxcoord: Vec3,
}

impl Plane {
    #[allow(dead_code)]
    pub fn new(origin: Vec3, normal: Vec3) -> Plane {
        let mut mincoord = Vec3 {
            x: std::f32::NEG_INFINITY,
            y: std::f32::NEG_INFINITY,
            z: std::f32::NEG_INFINITY,
        };
        let mut maxcoord = Vec3 {
            x: std::f32::INFINITY,
            y: std::f32::INFINITY,
            z: std::f32::INFINITY,
        };

        if Vec3::dot(
            &normal,
            &Vec3 {
                x: 1.0,
                y: 0.0,
                z: 0.0,
            },
        ) <= ZERO
        {
            mincoord.x = origin.x;
            maxcoord.x = origin.x;
        } else if Vec3::dot(
            &normal,
            &Vec3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
        ) <= ZERO
        {
            mincoord.y = origin.y;
            maxcoord.y = origin.y;
        } else if Vec3::dot(
            &normal,
            &Vec3 {
                x: 0.0,
                y: 0.0,
                z: 1.0,
            },
        ) <= ZERO
        {
            mincoord.z = origin.z;
            maxcoord.z = origin.z;
        }
        Plane {
            origin,
            normal,
            mincoord,
            maxcoord,
        }
    }
}

impl Hittable for Plane {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let denom = Vec3::dot(&self.normal, &rayon.direction);
        if denom.abs() >= ZERO {
            let t = Vec3::dot(&(self.origin - rayon.origin), &self.normal) / denom;
            if t >= 0.0 {
                return Some(HitInfo {
                    distance: t,
                    normal: self.normal,
                    point: rayon.point_at(t),
                    rayon: *rayon,
                    position: Vec3::zero(),
                });
            }
        }
        None
    }

    fn get_type(&self) -> Hittables {
        Hittables::Plane
    }

    fn to_plane(&self) -> Option<Plane> {
        Some(Plane {
            origin: self.origin,
            normal: self.normal,
            mincoord: self.mincoord,
            maxcoord: self.maxcoord,
        })
    }
}
