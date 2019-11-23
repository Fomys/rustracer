use image::{DynamicImage, GenericImageView};

use crate::raytracer::color::Color;
use crate::raytracer::texture_maps::texture_map::TextureMap;


// L'implémentation de la texturemap pour une image png
#[derive(Clone)]
pub struct PngTextureMap {
    pub image: DynamicImage,
}

impl TextureMap for PngTextureMap {
    fn get_pixel(&self, x: f32, y: f32) -> Color {
        let color = self.image.get_pixel(
            ((x % 1.0) * self.image.dimensions().0 as f32).abs() as u32,
            ((y % 1.0) * self.image.dimensions().0 as f32).abs() as u32);
        Color { r: (color.0[0] as f32 / 256.0), g: (color.0[1] as f32 / 256.0), b: (color.0[2] as f32 / 256.0) }
    }
}