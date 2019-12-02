use crate::raytracer::color::{ColorInfo, Color, BLACK};
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::scene::Scene;
use std::sync::Arc;

pub trait MaterialPrimitive: Sync + Send {
    fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize) -> Color;
}

pub struct Material {
    pub materials: Vec<(f32, Arc<dyn MaterialPrimitive>)>,
    pub texture_ratio: f32,
}

impl Material {
    pub fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize) -> Color {
        let mut new_color = BLACK;
        for (weight, material) in self.materials.iter() {
            new_color += material.get_color(hitinfo, scene, max_iter) * *weight;
        }
        new_color
    }
}