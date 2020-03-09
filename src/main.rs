use crate::raytracer::camera::Camera;
use crate::raytracer::color::Color;
use crate::raytracer::integrators::{Integrator, ParallelIntegrator};
use crate::raytracer::lights::Omnidirectional;
use crate::raytracer::loaders::loader::Loader;
use crate::raytracer::loaders::obj::ObjLoader;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::{Vec2, Vec3};
use obj::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::Arc;

mod raytracer;
fn main() {
    let center = Vec3 {
        x: -70.0,
        y: 90.0,
        z: 100.0,
    } - 20.0
        * Vec3 {
            x: 3.0,
            y: 0.0,
            z: 1.0,
        };

    let camera = Camera::new(
        center,
        Vec3 {
            x: 3.0,
            y: -0.7,
            z: -1.0,
        },
        Vec2 { x: 4.0, y: 2.0 },
        Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        Vec2 { x: 800, y: 600 },
        //Vec2 { x: 1920, y: 1080 },
        "out.png".to_string(),
        1,
    );
    let mut scene = ObjLoader::load_as_scene(Path::new("low-poly-fox-by-pixelmannen.obj")).unwrap();

    let light = Omnidirectional::new(Color::WHITE, center, 1000.0);

    scene.add_light(Arc::new(light));

    println!("{:?}", scene.primitives.len());

    let mut integrator = ParallelIntegrator::new(camera, scene);

    integrator.render(5);
}
