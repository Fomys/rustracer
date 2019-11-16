use crate::raytracer::ray::Ray;
use crate::raytracer::vec3::Vec3;
use crate::raytracer::color::Color;

pub trait Hittable {
    fn compute_hit(&self, rayon: &Ray) -> Option<f32>;

    fn get_intesect(&self, rayon: &Ray) -> Option<Vec3>;

    fn get_color(&self, rayon: &Ray) -> Color;
}