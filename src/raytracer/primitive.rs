use crate::raytracer::hittables::hittable::Hittable;
use crate::raytracer::materials::material::{Material, MaterialPrimitive};
use crate::raytracer::textures::texture::Texture;
use std::sync::Arc;

pub struct Primitive {
    pub hittable: Arc<dyn Hittable>,
    pub material: Arc<Material>,
    pub texture: Arc<dyn Texture>,
}
