use crate::raytracer::color::Color;
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::utils::vec::Vec3;

pub trait Light: Sync + Send {
    fn get_color(&self) -> Color;
    fn get_position(&self) -> Vec3;
}
