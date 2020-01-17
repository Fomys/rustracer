use perlin_noise::PerlinNoise;

use crate::raytracer::color::Color;
use crate::raytracer::hittables::HitInfo;
use crate::raytracer::textures::Texture;
use crate::raytracer::utils::Vec3;

pub struct PerlinTexture {
    color: Color,
    noise: PerlinNoise,
}

impl PerlinTexture {
    pub fn new(color: Color) -> PerlinTexture {
        PerlinTexture {
            color,
            noise: PerlinNoise::new(),
        }
    }
}

impl Texture for PerlinTexture {
    fn get_color(&self, hitinfo: &HitInfo) -> Color {
        self.color
            * self.noise.get3d([
                hitinfo.point.x as f64,
                hitinfo.point.y as f64,
                hitinfo.point.z as f64,
            ]) as f32
    }
}
