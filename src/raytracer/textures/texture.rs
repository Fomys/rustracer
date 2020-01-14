use crate::raytracer::color::Color;
use crate::raytracer::hittables::HitInfo;

pub trait Texture: Sync + Send {
    fn get_color(&self, hitinfo: &HitInfo) -> Color;
}
