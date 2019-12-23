use std::sync::Arc;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::{Color, BLACK, WHITE};
use crate::raytracer::hittables::cylinder::Cylinder;
use crate::raytracer::hittables::plane::Plane;
use crate::raytracer::hittables::sphere::Sphere;
use crate::raytracer::integrator::integrator::Integrator;
use crate::raytracer::integrator::parallel_integrator::ParallelIntegrator;
use crate::raytracer::lights::diffuse_spot::DiffuseSpot;
use crate::raytracer::lights::omnidirectional::Omnidirectional;
use crate::raytracer::lights::rectangle::Rectangle;
use crate::raytracer::lights::spot::Spot;
use crate::raytracer::materials::material::Material;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::vec::{Vec2, Vec3};
use crate::raytracer::{materials, textures};

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
    let sphere_4 = Sphere::new(
        Vec3 {
            x: 1.0,
            y: 0.0,
            z: 7.0,
        },
        1.0,
    );

    let plan_1 = Plane::new(
        Vec3 {
            x: 0.0,
            y: 1.5,
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
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        0.5,
    );

    let metal = Arc::new(materials::metal::Metal { fuzziness: 0.01 });
    let diffuse = Arc::new(materials::diffuse::Diffuse {});
    let transparent = Arc::new(materials::transparent::Transparent {
        refractive_index_div: 2.0,
    });
    let plain_material = Arc::new(materials::plain::Plain {});

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
    let plain_color_3 = textures::plain::Plain {
        color: Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        },
    };
    let plain_squares = textures::squares::Squares {};
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
            z: 10.0,
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
            z: -5.0,
        },
        Vec3 {
            x: 2.0,
            y: -1.0,
            z: -2.0,
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

    /*scene.add_primitive(
        Arc::new(sphere),
        Arc::new(Material {
            materials: vec![(0.05, diffuse.clone()), (1.0, metal.clone())],
        }),
        Arc::new(plain_color_2),
    );
    scene.add_primitive(
        Arc::new(sphere_2),
        Arc::new(Material {
            materials: vec![(0.05, diffuse.clone()), (1.0, metal.clone())],
        }),
        Arc::new(plain_color),
    );

    scene.add_primitive(
        Arc::new(sphere_3),
        Arc::new(Material {
            materials: vec![(0.05, diffuse.clone()), (1.0, metal.clone())],
        }),
        Arc::new(plain_color),
    );*/

    scene.add_primitive(
        Arc::new(plan_1),
        Arc::new(Material {
            materials: vec![(0.1, diffuse.clone()), (1.0, metal.clone())],
        }),
        Arc::new(plain_squares),
    );

    scene.add_primitive(
        Arc::new(cyl_1),
        Arc::new(Material {
            materials: vec![(0.5, diffuse.clone()), (1.0, metal.clone())],
        }),
        Arc::new(plain_color),
    );

    //    scene.add_light(light_omnidirectional);
    scene.add_light(light_spot);

    let mut integrator = ParallelIntegrator::new(camera, scene);

    integrator.render();
}
