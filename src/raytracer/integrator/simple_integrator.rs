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
    fn compute_ray(&self, ray: Ray, rng: &mut rand::XorShiftRng) -> Color {
        self.scene.trace(&ray, MAX_ITERATION, rng)
    }

    fn render(&mut self) {
        // NEXT: Intégrer le parallélisme ici
        let mut rng = rand::XorShiftRng::new_unseeded();
        for tile_index in 0..self.camera.tiles.len() {
            for pixel_index in 0..self.camera.tiles[tile_index].rays.len() {
                let mut new_color = BLACK;
                for ray_index in 0..self.camera.tiles[tile_index].rays[pixel_index].len() {
                    new_color += self.compute_ray(self.camera.tiles[tile_index].rays[pixel_index][ray_index], &mut rng);
                }
                self.camera.tiles[tile_index].buffer[pixel_index] = new_color / self.camera.tiles[tile_index].rays[pixel_index].len() as f32;
            }
            self.camera.merge_tile(tile_index);
            println!("{:.2}%", (tile_index as f32 * 100.0 /self.camera.tiles.len() as f32));
        }
        self.camera.save();
    }
}