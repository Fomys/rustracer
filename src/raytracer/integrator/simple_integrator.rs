use std::sync::mpsc;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::{BLACK, Color, RED};
use crate::raytracer::integrator::integrator::Integrator;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::consts::{ZERO, MAX_ITERATION};
use crate::raytracer::utils::vec::Vec2;

pub struct SimpleIntegrator {
    camera: Camera,
    scene: Scene,
}

impl SimpleIntegrator {
    pub fn new(camera: Camera, scene: Scene) -> SimpleIntegrator {
        SimpleIntegrator { camera, scene }
    }
}

impl Integrator for SimpleIntegrator {
    fn preprocess(&mut self) {}

    // NEXT: Remplacer Color par un spectre
    fn compute_ray(&self, ray: Ray) -> Color {
        self.scene.trace(&ray, MAX_ITERATION)
    }

    fn render(&mut self) {
        // NEXT: Intégrer le parallélisme ici
        for tile_index in 0..self.camera.tiles.len() {
            for ray_index in 0..self.camera.tiles[tile_index].rays.len() {
                let new_color = self.compute_ray(self.camera.tiles[tile_index].rays[ray_index]);
                self.camera.tiles[tile_index].buffer[ray_index] = new_color;
            }
            self.camera.merge_tile(tile_index);
        }
        self.camera.save();
    }
}