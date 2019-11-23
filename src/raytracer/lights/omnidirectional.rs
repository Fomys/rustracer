use crate::raytracer::color::Color;
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::lights::light::Light;

pub struct Omnidirectional {
    color: Color,
}

impl Light for Omnidirectional {
    fn get_color(&self, hitinfo: HitInfo) -> Color {}
}