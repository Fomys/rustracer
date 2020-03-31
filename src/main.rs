#![allow(clippy::eq_op)]
#![allow(clippy::redundant_clone)]

use std::sync::Arc;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::Color;
use crate::raytracer::hittables::{Circle, Cylinder, Parallelogram, Plane, Sphere};
use crate::raytracer::integrators::{Integrator, ParallelIntegrator, SimpleIntegrator};
use crate::raytracer::lights::{DiffuseSpot, Directional, Omnidirectional, Rectangle};
use crate::raytracer::loaders::loader::Loader;
use crate::raytracer::loaders::obj::ObjLoader;
use crate::raytracer::materials::{Material, Plain};
use crate::raytracer::movements::movement::{Movement, MovementPart, MovementPrimitive};
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::{Vec2, Vec3, FAR_FAR_AWAY};
use crate::raytracer::{materials, textures};
use std::path::Path;

mod raytracer;

#[allow(unused_variables)]
fn main() {
    /*let center = Vec3 {
        x: -70.0,
        y: 90.0,
        z: 100.0,
    } - 20.0
        * Vec3 {
            x: 3.0,
            y: 0.0,
            z: 1.0,
        };*/

    let center = Vec3 {
        x: 0.0,
        y: 5.0,
        z: -3.0,
    };

    let camera = Camera::new(
        center,
        Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.7,
        },
        Vec2 { x: 4.0, y: 2.0 },
        Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
        //Vec2 { x: 80, y: 60 },
        Vec2 { x: 1920, y: 1080 },
        "out.png".to_string(),
        1,
    );
    let mut scene = ObjLoader::load_as_scene(Path::new("donuts.obj")).unwrap();

    scene.add_primitive(
        Arc::new(Sphere::new(
            Vec3::ZERO,
            0.01,
            Movement::NONE,
            Arc::new(textures::Plain { color: Color::RED }),
        )),
        Arc::new(Material::new(vec![(1.0, Arc::new(Plain {}))])),
    );
    //let light = Omnidirectional::new(Color::WHITE, center, 10000.0);

    //scene.add_light(Arc::new(light));
    scene.preprocess();

    let mut integrator = ParallelIntegrator::new(camera, scene);

    integrator.render(5);
}
