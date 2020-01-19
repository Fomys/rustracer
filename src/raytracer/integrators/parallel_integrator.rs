use std::sync::{mpsc, Arc};

use threadpool::ThreadPool;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::BLACK;
use crate::raytracer::integrators::Integrator;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::MAX_ITERATION;

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

    fn render(&mut self) {
        let mut merged = 0;
        let (tx, rx) = mpsc::channel();
        let pool: ThreadPool = threadpool::Builder::new()
            .thread_name("Un pti raytracer".into())
            .build();
        let total = self.camera.tile_count.x * self.camera.tile_count.y;
        let mut launched = 0;
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
                let mut rng = rand::XorShiftRng::new_unseeded();
                for pixel_index in 0..tile.rays.len() {
                    let mut new_color = BLACK;
                    for ray_index in 0..tile.rays[pixel_index].len() {
                        new_color += scene_thread.trace(
                            &tile.rays[pixel_index][ray_index],
                            MAX_ITERATION,
                            &mut rng,
                        );
                    }
                    tile.buffer[pixel_index] = new_color / tile.rays[pixel_index].len() as f32;
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
