use crate::raytracer::camera::Camera;
use crate::raytracer::color::{Color, BLACK, RED};
use crate::raytracer::integrator::integrator::Integrator;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::consts::{MAX_ITERATION, ZERO};
use crate::raytracer::utils::vec::Vec2;
use std::sync::{mpsc, Arc};
use threadpool::ThreadPool;

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
        let (tx, rx) = mpsc::channel();
        let pool: ThreadPool = threadpool::Builder::new()
            .thread_name("Un pti raytracer".into())
            .build();
        let total = self.camera.tile_count.x * self.camera.tile_count.y;
        let mut launched = 0;
        while let Some(mut tile) = self.camera.next_tile() {
            launched += 1;
            println!("Launch {}/{}", launched, total);
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
                tx_thread.send(tile);
            });
        }
        let mut merged = 0;
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
