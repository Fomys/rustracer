use crate::raytracer::camera::Camera;
use crate::raytracer::color::BLACK;
use crate::raytracer::integrators::integrator::Integrator;
use crate::raytracer::scene::Scene;

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

    fn render(&mut self, max_iteration: usize) {
        let mut rng = rand::XorShiftRng::new_unseeded();
        let ray_per_pixel_count = self.camera.ray_per_pixels_count;
        let total = self.camera.tile_count.x * self.camera.tile_count.y;
        let mut finished = 0;
        while let Some(mut tile) = self.camera.next_tile() {
            println!(
                "{} finished/{} remaining/{} total",
                finished,
                total - finished,
                total
            );
            for pixel_index in 0..tile.size.x * tile.size.y {
                let mut new_color = BLACK;
                for ray_index in 0..ray_per_pixel_count {
                    new_color += self.scene.trace(
                        &tile.rays[pixel_index * ray_per_pixel_count + ray_index],
                        max_iteration,
                        &mut rng,
                    );
                }
                tile.buffer[pixel_index] = new_color / self.camera.ray_per_pixels_count as f32;
            }
            self.camera.merge_tile(&tile);
            finished += 1;
        }
        self.camera.save();
    }
    fn next_frame(&mut self) {
        self.camera.next_frame();
        self.scene.next_pos();
    }
}
