use std::sync::{mpsc, Arc};

use threadpool::ThreadPool;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::Color;
use crate::raytracer::integrators::Integrator;
use crate::raytracer::scene::Scene;

pub struct ParallelIntegrator {
    camera: Camera,
    scene: Arc<Scene>,
}

impl ParallelIntegrator {
    pub fn new(camera: Camera, scene: Scene) -> ParallelIntegrator {
        ParallelIntegrator {
            camera,
            scene: Arc::new(scene),
        }
    }
}

impl Integrator for ParallelIntegrator {
    fn preprocess(&mut self) {}

    fn next_frame(&mut self) {
        self.camera.next_frame();
        Arc::get_mut(&mut self.scene).unwrap().next_pos();
    }

    fn render(&mut self, max_iteration: usize) {
        let mut merged = 0;
        let (tx, rx) = mpsc::channel();
        let pool: ThreadPool = threadpool::Builder::new()
            .thread_name("Un pti raytracer".into())
            .build();
        let total = self.camera.tile_count.x * self.camera.tile_count.y;
        let mut launched = 0;
        let ray_per_pixel_count = self.camera.ray_per_pixels_count;
        while let Some(mut tile) = self.camera.next_tile() {
            while pool.queued_count() > 5 {
                for tile in rx.try_iter() {
                    self.camera.merge_tile(&tile);
                    merged += 1;
                }
            }
            launched += 1;
            println!(
                "{} launched/{} waiting/{} merged/{} total",
                launched,
                pool.queued_count(),
                merged,
                total
            );
            let tx_thread = tx.clone();
            let scene_thread = self.scene.clone();
            pool.execute(move || {
                let mut rng = rand::thread_rng();
                for pixel_index in 0..tile.size.x * tile.size.y {
                    let mut new_color = Color::BLACK;
                    for ray_index in 0..ray_per_pixel_count {
                        new_color += scene_thread.trace(
                            &tile.rays[pixel_index * ray_per_pixel_count + ray_index],
                            max_iteration,
                            &mut rng,
                        );
                    }
                    tile.buffer[pixel_index] = new_color / ray_per_pixel_count as f32;
                }
                tile.free_mem();
                tx_thread.send(tile).unwrap();
            });
        }
        while pool.active_count() > 0 {
            for tile in rx.try_iter() {
                self.camera.merge_tile(&tile);
                merged += 1;
                println!("Merge {}/{}", merged, total);
            }
        }
        self.camera.save();
    }
}
