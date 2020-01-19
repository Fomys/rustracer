use crate::raytracer::camera::Camera;
use crate::raytracer::color::BLACK;
use crate::raytracer::integrators::integrator::Integrator;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::MAX_ITERATION;

pub struct SimpleIntegrator {
    camera: Camera,
    scene: Scene,
}

impl SimpleIntegrator {
    #[allow(dead_code)]
    pub fn new(camera: Camera, scene: Scene) -> SimpleIntegrator {
        SimpleIntegrator { camera, scene }
    }
}

impl Integrator for SimpleIntegrator {
    fn preprocess(&mut self) {}

    fn render(&mut self) {
        let mut rng = rand::XorShiftRng::new_unseeded();
        while let Some(mut tile) = self.camera.next_tile() {
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
