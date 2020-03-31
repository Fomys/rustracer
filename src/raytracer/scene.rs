use std::sync::Arc;

use crate::raytracer::bvh::{Empty, BVH};
use crate::raytracer::color::Color;
use crate::raytracer::hittables::{HitInfo, Hittable};
use crate::raytracer::lights::Light;
use crate::raytracer::materials::Material;
use crate::raytracer::primitive::Primitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::textures::Texture;
use rand::prelude::ThreadRng;

pub struct Scene {
    pub primitives: Vec<Arc<Primitive>>,
    pub lights: Vec<Arc<dyn Light>>,
    pub bvh: Arc<dyn BVH>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            primitives: vec![],
            lights: vec![],
            bvh: Arc::new(Empty {}),
        }
    }

    pub fn preprocess(&mut self) {
        for primitive in &self.primitives {
            self.bvh = self.bvh.insert_primitive(primitive.clone());
        }
        self.bvh.prt(0);
    }

    pub fn add_primitives(&mut self, primitives: Vec<Arc<Primitive>>) {
        self.primitives = [&self.primitives[..], &primitives[..]].concat();
    }

    pub fn add_lights(&mut self, lights: Vec<Arc<dyn Light>>) {
        self.lights = [&self.lights[..], &lights[..]].concat();
    }

    pub fn add_light(&mut self, light: Arc<dyn Light>) {
        self.lights.push(light);
    }

    pub fn add_primitive(&mut self, hittable: Arc<dyn Hittable>, material: Arc<Material>) {
        let primitive: Primitive = Primitive { hittable, material };
        self.primitives.push(Arc::new(primitive));
    }

    pub fn background_color(&self, _rayon: &Ray) -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn launch_ray_min_dist(&self, rayon: &Ray, distance: f32) -> HitInfo {
        let (closest_hitinfo, _) = self.bvh.compute_hit(rayon);
        closest_hitinfo
    }

    pub fn launch_ray(&self, rayon: &Ray) -> (HitInfo, Option<Arc<Primitive>>) {
        // Search visible object
        let (closest_hitinfo, closest_primitive) = self.bvh.compute_hit(rayon);
        (closest_hitinfo, closest_primitive)
    }

    pub fn trace(&self, rayon: &Ray, max_iter: usize, rng: &mut ThreadRng) -> Color {
        let (closest_hitinfo, closest_primitive) = self.launch_ray(rayon);
        if let Some(object) = closest_primitive {
            // Get material color (color due to reflect, refract...)
            let color = object
                .material
                .get_color(&closest_hitinfo, self, max_iter, rng);
            return color
                / (0.05 * closest_hitinfo.distance
                    + 0.02 * closest_hitinfo.distance * closest_hitinfo.distance);
        }

        // Get texture color
        self.background_color(rayon)
    }

    pub fn next_pos(&mut self) {
        for primitive in self.primitives.iter_mut() {
            let hittable = Arc::get_mut(&mut Arc::get_mut(primitive).unwrap().hittable).unwrap();
            hittable.next_pos();
        }
    }
}
