use std::sync::Arc;

use crate::raytracer::color::Color;
use crate::raytracer::hittables::HitInfo;
use crate::raytracer::scene::Scene;
use rand::prelude::ThreadRng;

pub trait MaterialPrimitive: Sync + Send {
    fn get_color(
        &self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize, rng: &mut ThreadRng,
    ) -> Color;
}

pub struct Material {
    materials: Vec<(f32, Arc<dyn MaterialPrimitive>)>,
    sum_weight: f32,
}

impl Material {
    pub fn new(materials: Vec<(f32, Arc<dyn MaterialPrimitive>)>) -> Material {
        let mut sum_weight = 0.0;
        for material in materials.clone() {
            sum_weight += material.0;
        }
        Material {
            materials,
            sum_weight,
        }
    }
    pub fn get_color(
        &self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize, rng: &mut ThreadRng,
    ) -> Color {
        let mut new_color = Color::BLACK;
        for (weight, material) in self.materials.iter() {
            new_color += material.get_color(hitinfo, scene, max_iter, rng) * *weight;
        }
        new_color / self.sum_weight
    }
}
