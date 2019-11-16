mod raytracer;

use minifb::{Window, WindowOptions, Key};
use crate::raytracer::vec3::Vec3;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::sphere::Sphere;
use crate::raytracer::color::Color;


const WIDTH: usize = 860;
const HEIGHT: usize = 540;

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

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut scene = Scene {
        objects: vec![]
    };
    
    let sphere1 = Sphere {
        center: Vec3 {x: 0.0, y: 0.0, z: -10.0},
        color: Color {r:0.0, g:1.0, b:0.0},
        radius: 2.0
    };

    let sphere2 = Sphere {
        center: Vec3 {x: -3.0, y: 0.0, z: -11.0},
        color: Color {r:1.0, g:0.0, b:0.0},
        radius: 3.0
    };

    let sol = Sphere {
        center: Vec3 {x: 0.0, y: 100.0, z: -10.0},
        color: Color {r: 0.5, g: 0.2, b: 0.8},
        radius: 100.0,
    };

    scene.objects.push(Box::new(sphere1));
    scene.objects.push(Box::new(sphere2));
    scene.objects.push(Box::new(sol));

    let lower_left_corner = Vec3 { x: -1.0, y: -0.25, z: -1.0 };
    let horizontal = Vec3 { x: 1.0, y: 0.0, z: 0.0 };
    let vertical = Vec3 { x: 0.0, y: 0.5, z: 0.0 };
    let origin = Vec3 { x: 0.0, y: 0.0, z: 0.0 };

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let u: f32 = i as f32 / HEIGHT as f32;
            let v: f32 = j as f32 / WIDTH as f32;
            let rayon: Ray = Ray { origin, direction: lower_left_corner + v * horizontal + u * vertical };

            let col: Color = scene.trace(rayon);


            //println!("{:?}", rayon);
            buffer[(i * WIDTH + j) as usize] = col.to_pixel();
        }
        window.update_with_buffer(&buffer).unwrap();
    }

    while window.is_open() && !window.is_key_down(Key::Escape) { window.update(); }
}
