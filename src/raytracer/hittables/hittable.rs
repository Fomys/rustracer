use crate::raytracer::ray::Ray;
use crate::raytracer::vec3::Vec3;
use crate::raytracer::materials::material::Material;

pub struct HitInfo {
    pub distance: f32,
    pub normal: Vec3,
    pub point: Vec3,
}

pub trait Hittable {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo>;
    fn get_material(&self) -> &Box<dyn Material>;
}