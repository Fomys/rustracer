use std::sync::Arc;

use crate::raytracer::hittables::Hittable;
use crate::raytracer::materials::Material;
use crate::raytracer::textures::Texture;

#[derive(Clone)]
pub struct Primitive {
    pub hittable: Arc<dyn Hittable>,
    pub material: Arc<Material>,
}

/*use std::sync::Arc;

use crate::raytracer::hittables::{HitInfo, Hittable};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::Vec3;

pub struct Object {
    hittables: Vec<Arc<dyn Hittable>>,
    extremums: (Vec3, Vec3),
}

impl Object {
    pub fn new() -> Object {
        Object {
            hittables: vec![],
            extremums: (Vec3::ZERO, Vec3::ZERO),
        }
    }

    pub fn add_hittable(&mut self, hitable: Arc<dyn Hittable>) {
        self.update_extremums(hitable.get_extremums());
        self.hittables.push(hitable.clone());
    }

    fn update_extremums(&mut self, e: (Vec3, Vec3)) {
        self.extremums = (self.extremums.0.min(&e.0), self.extremums.1.max(&e.1));
    }
}

impl Hittable for Object {
    fn compute_hit(&self, rayon: &Ray) -> HitInfo {
        let tx1: f32 = (self.extremums.0.x - rayon.origin.x) / rayon.direction.x;
        let tx2: f32 = (self.extremums.1.x - rayon.origin.x) / rayon.direction.x;

        let mut tmin: f32 = tx1.min(tx2);
        let mut tmax: f32 = tx1.max(tx2);

        let ty1: f32 = (self.extremums.0.y - rayon.origin.y) / rayon.direction.y;
        let ty2: f32 = (self.extremums.1.y - rayon.origin.y) / rayon.direction.y;

        tmin = tmin.max(ty1.min(ty2));
        tmax = tmin.min(ty1.max(ty2));

        let tz1: f32 = (self.extremums.0.z - rayon.origin.z) / rayon.direction.z;
        let tz2: f32 = (self.extremums.1.z - rayon.origin.z) / rayon.direction.z;

        tmin = tmin.max(tz1.min(tz2));
        tmax = tmin.min(tz1.max(tz2));

        if tmax >= 0.0f32.max(tmin) {
            let mut hitinfo = HitInfo::NONE;
            for hit in &self.hittables {
                hitinfo = hitinfo.min(hit.compute_hit(rayon));
            }
            return hitinfo;
        }
        HitInfo::NONE
    }

    fn get_extremums(&self) -> (Vec3, Vec3) {
        self.extremums
    }
}
*/
