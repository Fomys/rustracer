#![allow(clippy::eq_op)]
#![allow(clippy::redundant_clone)]

use std::sync::Arc;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::{Color, WHITE};
use crate::raytracer::hittables::{Circle, Cylinder, Plane, Sphere};
use crate::raytracer::integrators::{Integrator, ParallelIntegrator, SimpleIntegrator};
use crate::raytracer::lights::{DiffuseSpot, Omnidirectional, Rectangle};
use crate::raytracer::materials::Material;
use crate::raytracer::movements::movement::{Movement, MovementPart, MovementPrimitive};
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::{Vec2, Vec3};
use crate::raytracer::{materials, textures};

mod raytracer;

#[allow(unused_variables)]
fn main() {
    let matches = clap::App::new("Rustracer")
        .version("0.0.1")
        .author("Louis Chauvet <louis.chauver@free.fr>")
        .arg(
            clap::Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Set custom output, default \"out.png\"")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("max_iteration")
                .short("m")
                .long("maxiter")
                .value_name("MAX ITERATION")
                .help("Set max iteration, default 4")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("ray_per_pixels")
                .short("r")
                .long("rayperpixels")
                .value_name("RAYS PER PIXELS")
                .help("Set ray per pixels value, default 5")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("number_of_frames")
                .short("n")
                .long("numberofframes")
                .value_name("NUMBER OF FRAMES")
                .help("Set number of frames")
                .takes_value(true),
        )
        .get_matches();

    let file = matches.value_of("output").unwrap_or("out.png");
    let max_iteration = matches
        .value_of("max_iteration")
        .unwrap_or("4")
        .to_string()
        .parse::<usize>()
        .unwrap_or(4);
    let ray_per_pixels = matches
        .value_of("ray_per_pixels")
        .unwrap_or("1")
        .to_string()
        .parse::<usize>()
        .unwrap_or(1);
    let number_of_frames = matches
        .value_of("number_of_frames")
        .unwrap_or("10")
        .to_string()
        .parse::<usize>()
        .unwrap_or(10);

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
        Vec2 { x: 800, y: 600 },
        //Vec2 { x: 1920, y: 1080 },
        file.to_string(),
        5,
    );
    let mut scene = Scene::new();

    let sphere = Sphere::new(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 5.0,
        },
        1.0,
        Movement::new(vec![
            MovementPart {
                start_frame: 0,
                end_frame: 50,
                movement: MovementPrimitive::Scale(0.99),
            },
            MovementPart {
                start_frame: 50,
                end_frame: 100,
                movement: MovementPrimitive::Scale(1.01),
            },
            MovementPart {
                start_frame: 30,
                end_frame: 70,
                movement: MovementPrimitive::Translation(Vec3 {
                    x: 0.01,
                    y: 0.0,
                    z: 0.01,
                }),
            },
            MovementPart {
                start_frame: 100,
                end_frame: 101,
                movement: MovementPrimitive::Cycle(0),
            },
        ]),
    );
    let sphere_2 = Sphere::new(
        Vec3 {
            x: 3.0,
            y: 4.0,
            z: 4.0,
        },
        1.0,
        Movement::new(vec![
            MovementPart {
                start_frame: 0,
                end_frame: 50,
                movement: MovementPrimitive::Scale(0.99),
            },
            MovementPart {
                start_frame: 50,
                end_frame: 100,
                movement: MovementPrimitive::Scale(1.01),
            },
            MovementPart {
                start_frame: 30,
                end_frame: 70,
                movement: MovementPrimitive::Translation(Vec3 {
                    x: 0.01,
                    y: 0.0,
                    z: 0.01,
                }),
            },
            MovementPart {
                start_frame: 100,
                end_frame: 101,
                movement: MovementPrimitive::Cycle(0),
            },
        ]),
    );
    let sphere_5 = Sphere::new(
        Vec3 {
            x: -5.0,
            y: 4.0,
            z: 9.0,
        },
        1.0,
        Movement::new(vec![
            MovementPart {
                start_frame: 0,
                end_frame: 50,
                movement: MovementPrimitive::Scale(0.99),
            },
            MovementPart {
                start_frame: 50,
                end_frame: 100,
                movement: MovementPrimitive::Scale(1.01),
            },
            MovementPart {
                start_frame: 30,
                end_frame: 70,
                movement: MovementPrimitive::Translation(Vec3 {
                    x: 0.01,
                    y: 0.0,
                    z: 0.01,
                }),
            },
            MovementPart {
                start_frame: 100,
                end_frame: 101,
                movement: MovementPrimitive::Cycle(0),
            },
        ]),
    );
    let sphere_6 = Sphere::new(
        Vec3 {
            x: 5.0,
            y: 4.0,
            z: 7.0,
        },
        1.0,
        Movement::new(vec![
            MovementPart {
                start_frame: 0,
                end_frame: 50,
                movement: MovementPrimitive::Scale(0.99),
            },
            MovementPart {
                start_frame: 50,
                end_frame: 100,
                movement: MovementPrimitive::Scale(1.01),
            },
            MovementPart {
                start_frame: 30,
                end_frame: 70,
                movement: MovementPrimitive::Translation(Vec3 {
                    x: 0.01,
                    y: 0.0,
                    z: 0.01,
                }),
            },
            MovementPart {
                start_frame: 100,
                end_frame: 101,
                movement: MovementPrimitive::Cycle(0),
            },
        ]),
    );
    let sphere_3 = Sphere::new(
        Vec3 {
            x: 0.0,
            y: 4.0,
            z: 7.0,
        },
        2.0,
        Movement::new(vec![
            MovementPart {
                start_frame: 0,
                end_frame: 50,
                movement: MovementPrimitive::Scale(0.99),
            },
            MovementPart {
                start_frame: 50,
                end_frame: 100,
                movement: MovementPrimitive::Scale(1.01),
            },
            MovementPart {
                start_frame: 30,
                end_frame: 70,
                movement: MovementPrimitive::Translation(Vec3 {
                    x: 0.01,
                    y: 0.0,
                    z: 0.01,
                }),
            },
            MovementPart {
                start_frame: 100,
                end_frame: 101,
                movement: MovementPrimitive::Cycle(0),
            },
        ]),
    );
    let sphere_4 = Sphere::new(
        Vec3 {
            x: 0.5,
            y: 4.0,
            z: 10.5,
        },
        1.2,
        Movement::new(vec![
            MovementPart {
                start_frame: 0,
                end_frame: 50,
                movement: MovementPrimitive::Scale(0.99),
            },
            MovementPart {
                start_frame: 50,
                end_frame: 100,
                movement: MovementPrimitive::Scale(1.01),
            },
            MovementPart {
                start_frame: 20,
                end_frame: 21,
                movement: MovementPrimitive::Cycle(0),
            },
        ]),
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
        Movement::new(vec![
            MovementPart {
                start_frame: 0,
                end_frame: 50,
                movement: MovementPrimitive::Scale(0.99),
            },
            MovementPart {
                start_frame: 50,
                end_frame: 100,
                movement: MovementPrimitive::Scale(1.01),
            },
            MovementPart {
                start_frame: 30,
                end_frame: 70,
                movement: MovementPrimitive::Translation(Vec3 {
                    x: 0.01,
                    y: 0.0,
                    z: 0.01,
                }),
            },
            MovementPart {
                start_frame: 100,
                end_frame: 101,
                movement: MovementPrimitive::Cycle(0),
            },
        ]),
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
        Movement::new(vec![
            MovementPart {
                start_frame: 0,
                end_frame: 50,
                movement: MovementPrimitive::Scale(0.99),
            },
            MovementPart {
                start_frame: 50,
                end_frame: 100,
                movement: MovementPrimitive::Scale(1.01),
            },
            MovementPart {
                start_frame: 30,
                end_frame: 70,
                movement: MovementPrimitive::Translation(Vec3 {
                    x: 0.01,
                    y: 0.0,
                    z: 0.01,
                }),
            },
            MovementPart {
                start_frame: 100,
                end_frame: 101,
                movement: MovementPrimitive::Cycle(0),
            },
        ]),
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
        Movement::new(vec![
            MovementPart {
                start_frame: 0,
                end_frame: 50,
                movement: MovementPrimitive::Scale(0.99),
            },
            MovementPart {
                start_frame: 50,
                end_frame: 100,
                movement: MovementPrimitive::Scale(1.01),
            },
            MovementPart {
                start_frame: 30,
                end_frame: 70,
                movement: MovementPrimitive::Translation(Vec3 {
                    x: 0.01,
                    y: 0.0,
                    z: 0.01,
                }),
            },
            MovementPart {
                start_frame: 100,
                end_frame: 101,
                movement: MovementPrimitive::Cycle(0),
            },
        ]),
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
    let texture_perlin = Arc::new(textures::Perlin::new(Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    }));
    let light_square = Arc::new(Rectangle::new(
        WHITE,
        Vec3 {
            x: 10.0,
            y: -5.0,
            z: 8.0,
        },
        Vec3 {
            x: -20.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.1,
        },
        100.0,
    ));

    let light_omnidirectional = Arc::new(Omnidirectional::new(
        Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
        },
        Vec3 {
            x: 0.0,
            y: -10.0,
            z: 1.0,
        },
        100.0,
    ));
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
    let light_omnidirectional_2 = Arc::new(Omnidirectional::new(
        Color {
            r: 0.5,
            g: 0.0,
            b: 1.0,
        },
        Vec3 {
            x: 10.0,
            y: 10.0,
            z: 0.0,
        },
        10.0,
    ));

    scene.add_primitive(
        Arc::new(sphere),
        Arc::new(Material::new(vec![
            (0.8, diffuse.clone()),
            (0.2, metal.clone()),
        ])),
        plain_color_3.clone(),
    );

    scene.add_primitive(
        Arc::new(sphere_2),
        Arc::new(Material::new(vec![
            (0.8, diffuse.clone()),
            (0.2, metal.clone()),
        ])),
        texture_perlin.clone(),
    );
    scene.add_primitive(
        Arc::new(sphere_5),
        Arc::new(Material::new(vec![
            (0.2, diffuse.clone()),
            (0.8, metal.clone()),
        ])),
        plain_color_2.clone(),
    );
    scene.add_primitive(
        Arc::new(sphere_6),
        Arc::new(Material::new(vec![
            (0.2, diffuse.clone()),
            (0.8, metal.clone()),
        ])),
        plain_color_3.clone(),
    );
    scene.add_primitive(
        Arc::new(sphere_4),
        Arc::new(Material::new(vec![
            (0.2, diffuse.clone()),
            (0.8, metal.clone()),
        ])),
        plain_color_2.clone(),
    );

    scene.add_primitive(
        Arc::new(sphere_3),
        Arc::new(Material::new(vec![
            (0.8, diffuse.clone()),
            (0.2, metal.clone()),
        ])),
        plain_color_3.clone(),
    );
    /*
           scene.add_primitive(
               Arc::new((plan_1)),
               Arc::new(Material::new(vec![
                   (0.8, diffuse.clone()),
                   (0.2, metal.clone()),
               ])),
               plain_squares.clone(),
           );

           scene.add_primitive(
               Arc::new((circle_1)),
               Arc::new(Material::new(vec![
                   (0.8, diffuse.clone()),
                   (0.2, metal.clone()),
               ])),
               plain_color.clone(),
           );
    */
    scene.add_primitive(
        Arc::new(cyl_1),
        Arc::new(Material::new(vec![
            (0.8, diffuse.clone()),
            (0.2, metal.clone()),
        ])),
        plain_color.clone(),
    );

    scene.add_light(light_omnidirectional);
    scene.add_light(light_spot);
    let mut integrator = ParallelIntegrator::new(camera, scene);

    for i in 0..10 {
        integrator.render(5);
        integrator.next_frame();
    }
}
