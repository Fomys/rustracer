use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::sync::Arc;

use crate::raytracer::color::Color;
use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::hittables::triangle::Triangle;
use crate::raytracer::materials::material::Material;
use crate::raytracer::primitive::Primitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::texture_maps::texture_map::TextureMap;
use crate::raytracer::textures;
use crate::raytracer::textures::texture::Texture;
use crate::raytracer::utils::vec::{Vec2, Vec3};

pub struct Scene {
    primitives: Vec<Primitive>,
    // Liste des primitives de la scène
    pub ambiant_light: Color,
    // Couleur ambiante
    pub ambiant_power: f32,
}

impl Scene {
    pub fn new(ambiant_light: Color, ambiant_power: f32) -> Scene {
        Scene {
            primitives: vec![],
            ambiant_light,
            ambiant_power,
        }
    }

    pub fn add_primitive(&mut self, hittable: Box<dyn Hittable>, material: Box<dyn Material>, texture: Box<dyn Texture>) {
        let primitive: Primitive = Primitive { hittable, material, texture };
        self.primitives.push(primitive);
    }


    pub fn background_color(&self, rayon: &Ray) -> Color {
        let r = rayon.normalized();
        Color { r: 0.0, g: 0.0, b: 0.0}
    }

    // La deuxième fonction qui borrow self
    pub fn trace(&self, rayon: &Ray, max_iter: usize) -> Color {
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


        if let Some(object) = closest_primitive {
                // Get material color (color due to reflect, refract...)
                let color_info = object.material.get_color(&closest_hitinfo, self, max_iter);
                // Get Texture color
                let texture_color = object.texture.get_color(&closest_hitinfo);
                return texture_color * (1.0 - color_info.ratio) + color_info.ratio * color_info.color;
            }


        // Get texture color
        self.background_color(rayon)
    }
}