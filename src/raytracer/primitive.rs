use std::sync::Arc;

use crate::raytracer::hittables::Hittable;
use crate::raytracer::materials::Material;
use crate::raytracer::textures::Texture;

pub struct Primitive {
    pub hittable: Arc<dyn Hittable>,
    pub material: Arc<Material>,
    pub texture: Arc<dyn Texture>,
}
