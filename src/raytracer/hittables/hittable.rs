use crate::raytracer::hittables::plane::Plane;
use crate::raytracer::hittables::sphere::Sphere;
use crate::raytracer::hittables::triangle::Triangle;
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::vec::Vec3;

#[derive(PartialEq)]
pub enum Hittables {
    Triangle,
    Plane,
    Sphere,
}

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
    fn extremums(&self) -> (Vec3, Vec3);
    fn get_type(&self) -> Hittables;
    fn to_sphere(&self) -> Option<Sphere>;
    fn to_triangle(&self) -> Option<Triangle>;
    fn to_plane(&self) -> Option<Plane>;
}
