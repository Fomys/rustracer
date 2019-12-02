use crate::raytracer::hittables::hittable::Hittable;
use crate::raytracer::materials::material::{Material, MaterialPrimitive};
use crate::raytracer::textures::texture::Texture;
use std::sync::Arc;

// Ici je peux garder des box, c'est quasiment unique pour chaque primitive
pub struct Primitive {
    pub hittable: Arc<dyn Hittable>,
    pub material: Arc<Material>,
    pub texture: Arc<dyn Texture>,
}
