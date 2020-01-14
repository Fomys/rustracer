use std::sync::Arc;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::{Color, WHITE};
use crate::raytracer::hittables::{Circle, Cylinder, Plane, Sphere};
use crate::raytracer::integrators::{Integrator, ParallelIntegrator};
use crate::raytracer::lights::{DiffuseSpot, Omnidirectional, Rectangle};
use crate::raytracer::materials::Material;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::{Vec2, Vec3};
use crate::raytracer::{materials, textures};

mod raytracer;

#[allow(unused_variables)]
fn main() {
    let camera = Camera::new(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Vec2 { x: 4.0, y: 2.0 },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vec2 { x: 1920, y: 1080 },
    );
    let mut scene = Scene::new();

    let sphere = Sphere::new(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        1.0,
    );
    let sphere_2 = Sphere::new(
        Vec3 {
            x: 3.0,
            y: 4.0,
            z: 10.0,
        },
        1.0,
    );
    let sphere_3 = Sphere::new(
        Vec3 {
            x: 0.0,
            y: 4.0,
            z: 16.0,
        },
        2.0,
    );
    let sphere_4 = Sphere::new(
        Vec3 {
            x: 0.5,
            y: 4.0,
            z: 10.5,
        },
        0.2,
    );

    let circle_1 = Circle::new(
        Vec3 {
            x: 2.0,
            y: -3.0,
            z: 10.0,
        },
        2.0,
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 1.0,
        },
    );

    let plan_1 = Plane::new(
        Vec3 {
            x: 0.0,
            y: 4.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
        },
    );

    let cyl_1 = Cylinder::new(
        Vec3 {
            x: 0.5,
            y: 4.0,
            z: 10.5,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        1.0,
    );

    let metal = Arc::new(materials::Metal { fuzziness: 0.01 });
    let diffuse = Arc::new(materials::Diffuse {});
    let transparent = Arc::new(materials::Transparent {
        refractive_index_div: 2.0,
    });
    let plain_material = Arc::new(materials::Plain {});

    let plain_color = Arc::new(textures::Plain {
        color: Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        },
    });
    let plain_color_2 = Arc::new(textures::Plain {
        color: Color {
            r: 1.0,
            g: 1.0,
            b: 0.0,
        },
    });
    let plain_color_3 = Arc::new(textures::Plain {
        color: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        },
    });
    let plain_squares = Arc::new(textures::Squares {});
    let light_square = Arc::new(Rectangle {
        color: WHITE,
        origin: Vec3 {
            x: 10.0,
            y: -5.0,
            z: 8.0,
        },
        dir1: Vec3 {
            x: -20.0,
            y: 0.0,
            z: 0.0,
        },
        dir2: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.1,
        },
        power: 100.0,
    });

    let light_omnidirectional = Arc::new(Omnidirectional {
        color: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        },
        position: Vec3 {
            x: 0.0,
            y: -10.0,
            z: 15.0,
        },
        power: 100.0,
    });
    let light_spot = Arc::new(DiffuseSpot::new(
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        },
        Vec3 {
            x: 10.0,
            y: -12.0,
            z: 15.0,
        },
        Vec3 {
            x: 2.0,
            y: -2.0,
            z: 2.0,
        },
        20.0,
        30.0,
        100.0,
    ));
    let light_omnidirectional_2 = Arc::new(Omnidirectional {
        color: Color {
            r: 0.5,
            g: 0.0,
            b: 1.0,
        },
        position: Vec3 {
            x: 10.0,
            y: 10.0,
            z: 0.0,
        },
        power: 10.0,
    });

    scene.add_primitive(
        Arc::new(sphere),
        Arc::new(Material::new(vec![(1.0, transparent.clone())])),
        plain_color_2.clone(),
    );

    scene.add_primitive(
        Arc::new(sphere_2),
        Arc::new(Material::new(vec![
            (0.5, diffuse.clone()),
            (1.0, metal.clone()),
        ])),
        plain_color.clone(),
    );
    scene.add_primitive(
        Arc::new(sphere_4),
        Arc::new(Material::new(vec![
            (0.5, diffuse.clone()),
            (1.0, metal.clone()),
        ])),
        plain_color.clone(),
    );

    scene.add_primitive(
        Arc::new(sphere_3),
        Arc::new(Material::new(vec![(1.0, transparent.clone())])),
        plain_color_3.clone(),
    );

    scene.add_primitive(
        Arc::new(plan_1),
        Arc::new(Material::new(vec![
            (0.5, diffuse.clone()),
            (1.0, metal.clone()),
        ])),
        plain_squares.clone(),
    );

    scene.add_primitive(
        Arc::new(circle_1),
        Arc::new(Material::new(vec![
            (0.5, diffuse.clone()),
            (1.0, metal.clone()),
        ])),
        plain_color_2.clone(),
    );

    scene.add_primitive(
        Arc::new(cyl_1),
        Arc::new(Material::new(vec![
            (0.5, diffuse.clone()),
            (1.0, metal.clone()),
        ])),
        plain_color_3.clone(),
    );

    scene.add_light(light_omnidirectional);
    scene.add_light(light_spot);

    let mut integrator = ParallelIntegrator::new(camera, scene);

    integrator.render();
}
