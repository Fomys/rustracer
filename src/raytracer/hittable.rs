use crate::raytracer::ray::Ray;
use crate::raytracer::vec3::Vec3;
use crate::raytracer::color::Color;

pub struct HitInfo {
    pub distance: f32,
    pub normal: Vec3,
    pub point: Vec3,
}

pub trait Hittable {
    fn get_reflect_factor(&self, point: Vec3) -> f32;

    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo>;

    fn get_intersect(&self, rayon: &Ray) -> Option<Vec3>;

    fn get_color(&self, rayon: &Ray) -> Color;
}