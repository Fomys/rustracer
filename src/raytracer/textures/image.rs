use crate::raytracer::color::Color;
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::textures::texture::Texture;
use crate::raytracer::texture_maps::texture_map::TextureMap;
use std::rc::Rc;
use crate::raytracer::utils::vec::Vec2;


// Voilà la struct qui pose problème, il contient une copie de la texture map, mais la texture n'est
// pas identique pour tous (car chaque primitive ne va pas afficher la même portion de la texturemap)
pub struct Image {
    image: Rc<dyn TextureMap>,
    origin: Vec2<f32>,
    dir1: Vec2<f32>,
    dir2: Vec2<f32>,
}

impl Image {
    pub fn new(image: Rc<dyn TextureMap>, a: Vec2<f32>, b: Vec2<f32>, c: Vec2<f32>) -> Image {
        Image {
            image,
            origin: a,
            dir1: b - a,
            dir2: c - a,
        }
    }
}

impl Texture for Image {
    fn get_color(&self, hitinfo: &HitInfo) -> Color {
        if hitinfo.position.x > 1.0 {
            println!("Nouveau rayon, contact à: {:?}", hitinfo.position);
        }
        let pos = self.origin + hitinfo.position.x * self.dir1 +
            hitinfo.position.y * self.dir2;
        self.image.get_pixel(pos.x, pos.y)
    }
}