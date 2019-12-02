use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::sync::Arc;

use crate::raytracer::color::Color;
use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::hittables::triangle::Triangle;
use crate::raytracer::lights::light::Light;
use crate::raytracer::materials::material::{MaterialPrimitive, Material};
use crate::raytracer::primitive::Primitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::texture_maps::texture_map::TextureMap;
use crate::raytracer::textures;
use crate::raytracer::textures::texture::Texture;
use crate::raytracer::utils::vec::{Vec2, Vec3};

pub struct Scene {
    primitives: Vec<Primitive>,
    pub(crate) lights: Vec<Arc<Light>>,
    // Liste des primitives de la scène
    pub ambiant_light: Color,
    // Couleur ambiante
    pub ambiant_power: f32,
}

impl Scene {
    pub fn new(ambiant_light: Color, ambiant_power: f32) -> Scene {
        Scene {
            primitives: vec![],
            lights: vec![],
            ambiant_light,
            ambiant_power,
        }
    }

    pub fn add_light(&mut self, light: Arc<dyn Light>) {
        self.lights.push(light);
    }

    pub fn add_primitive(
        &mut self,
        hittable: Arc<dyn Hittable>,
        material: Arc<Material>,
        texture: Arc<dyn Texture>,
    ) {
        let primitive: Primitive = Primitive {
            hittable,
            material,
            texture,
        };
        self.primitives.push(primitive);
    }

    pub fn background_color(&self, rayon: &Ray) -> Color {
        let r = rayon.normalized();
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }

    pub fn launch_ray_min_dist(&self, rayon: &Ray, distance: f32) -> bool {
        for primitive in self.primitives.iter() {
            if let Some(hitinfo) = primitive.hittable.compute_hit(&rayon) {
                if hitinfo.distance < distance {
                    return true;
                }
            }
        }
        false
    }

    pub fn launch_ray(&self, rayon: &Ray) -> (HitInfo, Option<&Primitive>) {
        let mut closest_primitive: Option<&Primitive> = None;
        let mut closest_hitinfo: HitInfo = HitInfo {
            distance: std::f32::INFINITY,
            normal: Vec3::zero(),
            point: Vec3::zero(),
            rayon: *rayon,
            position: Vec3::zero(),
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
            let material_color = object.material.get_color(&closest_hitinfo, self, max_iter, rng);
            // Get Texture color
            let texture_color = object.texture.get_color(&closest_hitinfo);
            return texture_color *  material_color;
        }

        // Get texture color
        self.background_color(rayon)
    }
}
