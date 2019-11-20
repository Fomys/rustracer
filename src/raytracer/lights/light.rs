use crate::raytracer::color::Color;
use crate::raytracer::hittables::hittable::HitInfo;

pub trait Light {
    fn get_color (&self, hitinfo: HitInfo) -> Color;
}