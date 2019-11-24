#![feature(in_band_lifetimes)]

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Instant;

use image::DynamicImage;
// Librement inspiré du raytracer de Lynix
use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use threadpool::ThreadPool;

use crate::raytracer::camera::Camera;
use crate::raytracer::color::Color;
use crate::raytracer::hittables::hittable::Hittable;
use crate::raytracer::hittables::triangle::Triangle;
use crate::raytracer::materials::material::Material;
use crate::raytracer::materials::metal::Metal;
use crate::raytracer::materials::plain::Plain;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::texture_maps::png_texture_map::PngTextureMap;
use crate::raytracer::textures;
use crate::raytracer::textures::texture::Texture;
use crate::raytracer::utils::vec::Vec3;

mod raytracer;



//1366;
//768;
const MAX_RECURSIONS: usize = 5;

fn main() {
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("settings")).unwrap();
    let width: usize = settings.get_int("width")
        .expect("width must be a integer") as usize;
    let height: usize = settings.get_int("height")
        .expect("height must be a integer") as usize;
    let rays_per_pixel: usize = settings.get_int("rays_per_pixel")
        .expect("height must be a integer") as usize;
    let max_recursions: usize = settings.get_int("max_recursions")
        .expect("height must be a integer") as usize;

    let mut window = Window::new(
        "Raytracer",
        width,
        height,
        WindowOptions::default(),
    )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    let mut rng = rand::thread_rng();
    let mut rng1 = vec![0.0; rays_per_pixel];
    let mut rng2 = vec![0.0; rays_per_pixel];
    for i in 0..rays_per_pixel {
        rng1[i] = rng.next_f32();
        rng2[i] = rng.next_f32();
    }

    let mut buffer: Vec<u32> = vec![0; width * height];

    let origin = Vec3 { x: 0.0, y: 20.0, z: 100.0 };
    let mut scene: Scene = Scene::new(Color { r: 1.0, g: 1.0, b: 1.0 }, 1.0);
    let camera: Camera = Camera::new(origin,
                                     Vec3 { x: 0.0, y: 0.0, z: 1.0 },
                                     8.0,
                                     4.0);
    //let metal_yellow: &dyn Material = &Metal {reflection_factor: 0.3,};

    let img: DynamicImage = image::open("texture.png").unwrap();

    let img_texture_map = PngTextureMap { image: img };

    let plain = Plain {};
    let texture_red = textures::plain::Plain { color: Color { r: 0.9, g: 0.2, b: 0.1 } };
    //let metal_black: &dyn Material = &Metal {reflection_factor: 0.7,};
    let metal_green = Metal { reflection_factor: 0.2 };

    //let sphere1: Box<dyn Hittable> = Box::new(Sphere::new(Vec3 { x: -1.5, y: 0.5, z: -1.0 }, 0.5, ));
    //let sphere2: Box<dyn Hittable> = Box::new(Sphere::new(Vec3 { x: 0.0, y: 0.75, z: -1.5 }, 0.75, ));
    //let sphere3: Box<dyn Hittable> = Box::new(Sphere::new(Vec3 { x: 1.5, y: 0.5, z: -1.0 }, 0.5, ));
    /*let sol_sphere: &dyn Hittable = &Sphere::new(
        Vec3 { x: 0.0, y: -1000.0, z: -1.0 },
        1000.0,
    );*/
    //let sol_plane: Box<dyn Hittable> = Box::new(Plane::new(Vec3 { x: 0.0, y: -5.0, z: 0.0 }, Vec3 { x: 0.0, y: 1.0, z: 0.0 },));
    let triangle = Triangle::new(Vec3 { x: -1.5, y: 0.5, z: -1.0 }, Vec3 { x: 1.5, y: 0.5, z: -1.0 }, Vec3 { x: -1.5, y: 2.5, z: -1.0 });

    //scene.add_primitive(sphere1, metal_black);
    //scene.add_primitive(sphere2, metal_yellow);
    //scene.add_primitive(sphere3, plain_red);
    //scene.add_primitive(sol_plane, metal_green);
    //scene.add_primitive(Box::new(triangle), Box::new(plain), Box::new(texture_red));

    scene.load_obj("test2.obj".to_string(), Box::new(plain), Arc::new(img_texture_map));


    // Pool
    let pool: ThreadPool = threadpool::Builder::new()
        .thread_name("Un pti raytracer presque fonctionnel".into())
        //.num_threads(8*4)
        .build();

    // Tx/Rx channel to send buffer
    let (tx, rx) = mpsc::channel();
    // Arc creation
    let rng1_arc = Arc::new(rng1);
    let rng2_arc = Arc::new(rng2);
    let scene_arc = Arc::new(scene);
    let camera_arc = Arc::new(camera);

    for i in 0..height {
        let tx_thread = tx.clone();
        let camera_thread = camera_arc.clone();
        let scene_thread = scene_arc.clone();
        let rng1_thread = rng1_arc.clone();
        let rng2_thread = rng2_arc.clone();
        pool.execute(move || {
            let mut thread_buffer = vec![0; width];
            for j in 0..width {
                let mut color: Color = Color::black();
                for k in 0..rays_per_pixel {
                    for l in 0..rays_per_pixel {
                        let u: f32 = (i as f32 + rng1_thread[k]) / height as f32;
                        let v: f32 = (j as f32 + rng2_thread[l]) / width as f32;
                        let rayon: Ray = camera_thread.get_ray(v, u);
                        color = color + scene_thread.trace(&rayon, MAX_RECURSIONS);
                    }
                }
                color = color / (rays_per_pixel * rays_per_pixel) as f32;
                thread_buffer[j as usize] = color.to_pixel();
            }

            tx_thread.send((i, thread_buffer))
                .expect("Failed to send temp buffer");
        });
    }
    let start: Instant = Instant::now();

    let mut finished = 0;
    while window.is_open() {
        window.update();
        if finished < height {
            for (y, buffer_thread) in rx.try_iter() {
                //println!("Line {}: {}/{}", y, finished, height);
                for i in 0..width {
                    buffer[i + (height - y - 1) * width] = buffer_thread[i];
                }
                window.update_with_buffer(&buffer)
                    .expect("Failed to update with buffer");
                finished += 1;
                if finished == height {
                    let duration = start.elapsed();
                    println!("Rendering took {}s", duration.as_secs_f32());
                    save_as_png("final.png", width as u32, height as u32, &buffer);
                }
            }
        }
    }
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