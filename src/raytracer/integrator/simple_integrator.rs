use crate::raytracer::camera::Camera;
use crate::raytracer::color::{Color, BLACK, RED};
use crate::raytracer::integrator::integrator::Integrator;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::consts::{MAX_ITERATION, ZERO};
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

    fn render(&mut self) {
        let mut rng = rand::XorShiftRng::new_unseeded();
        let mut total_pixel = 0;
        while let Some(mut tile) = self.camera.next_tile() {
            total_pixel += tile.rays.len();
            for pixel_index in 0..tile.rays.len() {
                let mut new_color = BLACK;
                for ray_index in 0..tile.rays[pixel_index].len() {
                    new_color += self.scene.trace(
                        &tile.rays[pixel_index][ray_index],
                        MAX_ITERATION,
                        &mut rng,
                    );
                }
                tile.buffer[pixel_index] = new_color / tile.rays[pixel_index].len() as f32;
            }
            self.camera.merge_tile(&tile);
        }
        self.camera.save();
    }
}
