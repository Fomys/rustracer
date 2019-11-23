use crate::raytracer::hittables::hittable::Hittable;
use crate::raytracer::materials::material::Material;
use crate::raytracer::textures::texture::Texture;


// Ici je peux garder des box, c'est quasiment unique pour chaque primitive
pub struct Primitive {
    pub hittable: Box<dyn Hittable>,
    pub material: Box<dyn Material>,
    pub texture: Box<dyn Texture>,
}
