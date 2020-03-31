use crate::raytracer::color::Color;
use crate::raytracer::hittables::HitInfo;
use crate::raytracer::textures::texture::Texture;

pub struct Squares {}

impl Texture for Squares {
    fn get_color(&self, hitinfo: &HitInfo) -> Color {
        let c1 = if ((if hitinfo.point.x > 0.0 {
            hitinfo.point.x
        } else {
            -hitinfo.point.x + 1.0
        }) % 2.0)
            > 1.0
        {
            Color::WHITE
        } else {
            Color::BLACK
        };
        if ((if hitinfo.point.z > 0.0 {
            hitinfo.point.z
        } else {
            -hitinfo.point.z + 1.0
        }) % 2.0)
            > 1.0
        {
            c1
        } else if c1 == Color::BLACK {
            Color::WHITE
        } else {
            Color::BLACK
        }
    }
}
