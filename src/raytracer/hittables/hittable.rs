use crate::raytracer::ray::Ray;
use crate::raytracer::utils::Vec3;

#[derive(Debug)]
pub struct HitInfo {
    pub distance: f32,
    pub normal: Vec3,
    pub point: Vec3,
    pub rayon: Ray,
    pub position: Vec3,
}

pub trait Hittable: Sync + Send {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo>;
    fn next_pos(&mut self) {}
}
