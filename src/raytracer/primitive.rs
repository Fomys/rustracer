use crate::raytracer::hittables::hittable::Hittable;
use crate::raytracer::materials::material::Material;
use crate::raytracer::textures::texture::Texture;

pub struct Primitive {
    pub hittable: Box<dyn Hittable>,
    pub material: Box<dyn Material>,
    pub texture: Box<dyn Texture>,
}
