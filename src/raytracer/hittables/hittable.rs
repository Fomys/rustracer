use crate::raytracer::vec::Vec3;
use crate::raytracer::ray::Ray;

pub struct HitInfo {
    pub distance: f32,
    pub normal: Vec3,
    pub point: Vec3,
    pub rayon: Ray,
}

pub trait Hittable {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo>;
    fn extremums(&self) -> (Vec3, Vec3);
}