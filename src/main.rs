mod raytracer;

use minifb::{Window, WindowOptions, Key};
use crate::raytracer::vec3::Vec3;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::hittables::sphere::Sphere;
use crate::raytracer::color::Color;
use rand::Rng;
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;
use crate::raytracer::materials;
use crate::raytracer::hittables::plane::Plane;
use crate::raytracer::camera::Camera;


const WIDTH: usize = 800;//1366;
const HEIGHT: usize = 600;//768;
const RAY_PER_PIXELS: usize = 5;
const MAX_RECURSIONS: usize = 5;

fn main() {
    let mut window = Window::new(
        "Raytracer - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut rng = rand::thread_rng();
    let mut rng1: [f32; RAY_PER_PIXELS] = [0.0; RAY_PER_PIXELS];
    let mut rng2: [f32; RAY_PER_PIXELS] = [0.0; RAY_PER_PIXELS];
    for i in 0..RAY_PER_PIXELS {
        rng1[i] = rng.next_f32();
        rng2[i] = rng.next_f32();
    }


    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut scene = Scene {
        objects: vec![],
        ambiant_light: Color {r:1.0, g:1.0, b:1.0},
        ambiant_power: 1.0,
    };

    let transparent = Box::new(materials::transparent::Transparent {
        color: Color {
            r: 1.0,
            g: 0.0,
            b: 0.0
        },
        refractive_index_div: 0.5
    });
    let metal_yellow = Box::new(materials::metal::Metal {
        color: Color {
            r: 1.0,
            g: 1.0,
            b: 0.0
        },
        reflection_factor: 0.3,
    });
    let plain_green = Box::new( materials::plain::Plain {
        color: Color {
            r: 0.0,
            g: 1.0,
            b: 0.0
        }
    });
    let plain_red = Box::new(materials::plain::Plain {
        color: Color {
            r: 1.0,
            g: 0.0,
            b: 0.0
        }
    });
    let metal_black = Box::new(materials::metal::Metal {
        color: Color {
            r: 0.0,
            g: 0.0,
            b: 0.0
        },
        reflection_factor: 0.7,
    });
    let metal_green = Box::new(materials::metal::Metal {
        color: Color {r: 0.0, g: 1.0, b: 0.0},
        reflection_factor: 0.2
    });

    let sphere1 = Box::new(Sphere {
        center: Vec3 {x:-1.5, y:0.5, z:-1.0},
        radius: 0.5,
    });
    let sphere2 = Box::new(Sphere {
        center: Vec3 {x: 0.0, y: 0.75, z: -1.5},
        radius: 0.75,
    });
    let sphere3 = Box::new(Sphere {
        center: Vec3 {x:1.5, y:0.5, z:-1.0},
        radius: 0.5,
    });
    let sol_sphere = Box::new(Sphere {
        center: Vec3 {x: 0.0, y: -1000.0, z: -1.0},
        radius: 1000.0,
    });
    let sol_plane = Box::new(Plane {
        origin: Vec3 {
            x: 0.0,
            y: -5.0,
            z: 0.0
        },
        normal: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0
        },
    });

    scene.add_object(sphere1, metal_black);
    scene.add_object(sphere2, metal_yellow);
    scene.add_object(sphere3, plain_red);
    scene.add_object(sol_sphere, metal_green);

    let lower_left_corner = Vec3 { x: -2.0, y: -0.5, z: -1.0 };
    let horizontal = Vec3 { x: 4.0, y: 0.0, z: 0.0 };
    let vertical = Vec3 { x: 0.0, y: 2.0, z: 0.0 };
    let origin = Vec3 { x: 0.0, y: 0.0, z: 5.0 };

    let camera = Camera::new(origin,
                             Vec3 {x:0.0, y:0.0, z:1.0},
                             4.0,
                             2.0);

    let start = Instant::now();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let mut color: Color = Color::black();
            for k in 0..RAY_PER_PIXELS {
                for l in 0..RAY_PER_PIXELS {
                    let u: f32 = (i as f32 + rng1[k]) / HEIGHT as f32;
                    let v: f32 = (j as f32 + rng2[l]) / WIDTH as f32;
                    let rayon: Ray = camera.get_ray(v, u);
                    color = color + scene.trace(&rayon, MAX_RECURSIONS);
                }
            }
            color = color / (RAY_PER_PIXELS*RAY_PER_PIXELS) as f32;
            buffer[((HEIGHT - i - 1) * WIDTH + j) as usize] = color.to_pixel();
        }
        window.update_with_buffer(&buffer).unwrap();
    }
    let duration = start.elapsed();
    println!("Rendering took {}s", duration.as_secs_f32());
    save_as_png("final.png", WIDTH as u32, HEIGHT as u32, &buffer);
    while window.is_open() && !window.is_key_down(Key::Escape) { window.update(); }
}

fn save_as_png(file_name: &str, width: u32, height: u32, buffer: &Vec<u32>) {
    let path = Path::new(file_name);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGB);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();

    let mut png_data = vec![0u8; 0];
    png_data.reserve_exact((width * height * 3) as usize);

    for value in buffer.iter() {
        let r = ((value & 0x00FF0000) >> 16) as u8;
        let g = ((value & 0x0000FF00) >> 8) as u8;
        let b = ((value & 0x000000FF) >> 0) as u8;

        png_data.push(r);
        png_data.push(g);
        png_data.push(b);
    }

    writer.write_image_data(&png_data).unwrap(); // Save
}