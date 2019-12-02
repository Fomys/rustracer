use crate::raytracer::camera::Camera;
use crate::raytracer::color::{Color, BLACK};
use crate::raytracer::hittables::sphere::Sphere;
use crate::raytracer::integrator::integrator::Integrator;
use crate::raytracer::integrator::simple_integrator::SimpleIntegrator;
use crate::raytracer::lights::omnidirectional::Omnidirectional;
use crate::raytracer::materials::material::Material;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::vec::{Vec2, Vec3};
use crate::raytracer::{materials, textures};
use std::sync::Arc;

mod raytracer;

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
        Vec2 { x: 1366, y: 768 },
    );
    let mut scene = Scene::new(BLACK, 0.0);

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
            x: 2.1,
            y: 0.0,
            z: 5.0,
        },
        1.0,
    );
    let sphere_3 = Sphere::new(
        Vec3 {
            x: -2.1,
            y: 0.0,
            z: 5.0,
        },
        1.0,
    );

    let metal = Arc::new(materials::metal::Metal {});
    let diffuse = Arc::new(materials::diffuse::Diffuse {});

    let plain_color = textures::plain::Plain {
        color: Color {
            r: 1.0,
            g: 0.0,
            b: 0.0,
        },
    };
    let plain_color_2 = textures::plain::Plain {
        color: Color {
            r: 1.0,
            g: 1.0,
            b: 0.0,
        },
    };

    let light_omnidirectional = Arc::new(Omnidirectional {
        color: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        },
        position: Vec3 {
            x: 0.0,
            y: -10.0,
            z: 0.0,
        },
    });
    let light_omnidirectional_2 = Arc::new(Omnidirectional {
        color: Color {
            r: 1.0,
            g: 0.0,
            b: 1.0,
        },
        position: Vec3 {
            x: 0.0,
            y: 10.0,
            z: 0.0,
        },
    });

    scene.add_primitive(
        Arc::new(sphere),
        Arc::new(Material {
            materials: vec![(0.1, diffuse.clone()), (0.9, metal.clone())],
            texture_ratio: 0.1,
        }),
        Arc::new(plain_color_2),
    );
    scene.add_primitive(
        Arc::new(sphere_2),
        Arc::new(Material {
            materials: vec![(0.95, diffuse.clone())],
            texture_ratio: 0.1,
        }),
        Arc::new(plain_color),
    );
    scene.add_primitive(
        Arc::new(sphere_3),
        Arc::new(Material {
            materials: vec![(0.95, diffuse.clone()), (0.05, metal.clone())],
            texture_ratio: 0.1,
        }),
        Arc::new(plain_color),
    );

    scene.add_light(light_omnidirectional);
    scene.add_light(light_omnidirectional_2);

    let mut integrator = SimpleIntegrator::new(camera, scene);

    integrator.render();
}
