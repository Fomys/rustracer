use crate::raytracer::ray::Ray;
use crate::raytracer::vec::Vec3;

#[derive(Debug)]
pub struct HitInfo {
    pub distance: f32,
    pub normal: Vec3,
    pub point: Vec3,
    pub rayon: Ray,
    pub position: (f32, f32),
}

    pub trait Hittable {
        fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo>;
        fn extremums(&self) -> (Vec3, Vec3);
    }