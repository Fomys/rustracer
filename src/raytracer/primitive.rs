use crate::raytracer::hittables::hittable::Hittable;
use crate::raytracer::materials::material::Material;

pub struct Primitive<'a> {
    pub hittable: &'a dyn Hittable,
    pub material: &'a dyn Material,
}
