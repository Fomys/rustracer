use std::sync::Arc;

use crate::raytracer::color::Color;
use crate::raytracer::hittables::{HitInfo, Hittable};
use crate::raytracer::lights::Light;
use crate::raytracer::materials::Material;
use crate::raytracer::primitive::Primitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::textures::Texture;
use crate::raytracer::utils::ZERO_VEC3;

pub struct Scene {
    pub(crate) primitives: Vec<Primitive>,
    pub(crate) lights: Vec<Arc<dyn Light>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            primitives: vec![],
            lights: vec![],
        }
    }

    pub fn add_light(&mut self, light: Arc<dyn Light>) {
        self.lights.push(light);
    }

    pub fn add_primitive(
        &mut self, hittable: Arc<dyn Hittable>, material: Arc<Material>, texture: Arc<dyn Texture>,
    ) {
        let primitive: Primitive = Primitive {
            hittable,
            material,
            texture,
        };
        self.primitives.push(primitive);
    }

    pub fn background_color(&self, _rayon: &Ray) -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn launch_ray_min_dist(&self, rayon: &Ray, distance: f32) -> Option<HitInfo> {
        for primitive in self.primitives.iter() {
            if let Some(hitinfo) = primitive.hittable.compute_hit(&rayon) {
                if hitinfo.distance < distance {
                    return Some(hitinfo);
                }
            }
        }
        None
    }

    pub fn launch_ray(&self, rayon: &Ray) -> (HitInfo, Option<&Primitive>) {
        let mut closest_primitive: Option<&Primitive> = None;
        let mut closest_hitinfo: HitInfo = HitInfo {
            distance: std::f32::INFINITY,
            normal: ZERO_VEC3,
            point: ZERO_VEC3,
            rayon: *rayon,
            position: ZERO_VEC3,
        };

        // Search visible object
        for primitive in self.primitives.iter() {
            if let Some(hitinfo) = primitive.hittable.compute_hit(&rayon) {
                if hitinfo.distance < closest_hitinfo.distance {
                    closest_primitive = Some(primitive);
                    closest_hitinfo = hitinfo;
                }
            }
        }

        (closest_hitinfo, closest_primitive)
    }

    pub fn trace(&self, rayon: &Ray, max_iter: usize, rng: &mut rand::XorShiftRng) -> Color {
        let (closest_hitinfo, closest_primitive) = self.launch_ray(rayon);
        if let Some(object) = closest_primitive {
            // Get material color (color due to reflect, refract...)
            let material_color = object
                .material
                .get_color(&closest_hitinfo, self, max_iter, rng);
            // Get Texture color
            let texture_color = object.texture.get_color(&closest_hitinfo);
            return texture_color * material_color;
        }

        // Get texture color
        self.background_color(rayon)
    }

    pub fn next_pos(&mut self) {
        for primitive in self.primitives.iter_mut() {
            let mut hittable = Arc::get_mut(&mut primitive.hittable).unwrap();
            hittable.next_pos();
        }
    }
}
