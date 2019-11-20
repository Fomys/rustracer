use crate::raytracer::color::Color;
use crate::raytracer::lights::light::Light;
use crate::raytracer::hittables::hittable::HitInfo;

pub struct Omnidirectional {
    color: Color,
}

impl Light for Omnidirectional {
    fn get_color(&self, hitinfo: HitInfo) -> Color {

    }
}