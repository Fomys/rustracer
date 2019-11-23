use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::raytracer::color::Color;
use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::hittables::triangle::Triangle;
use crate::raytracer::materials::material::Material;
use crate::raytracer::primitive::Primitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::textures::texture::Texture;
use crate::raytracer::vec::{Vec3, Vec2};
use crate::raytracer::texture_maps::texture_map::TextureMap;
use crate::raytracer::textures;


pub struct Scene {
    primitives: Vec<Primitive>,
    //textures: Vec<Box<dyn Texture>>,
    pub ambiant_light: Color,
    pub ambiant_power: f32,
}

impl Scene {
    pub fn load_obj(&mut self, filepath: String, material: Box<dyn Material>, img_texture_map: Box<dyn TextureMap>) {
        let file = match File::open(filepath) {
            Ok(f) => f,
            _ => { return; }
        };
        let reader = BufReader::new(file);

        let mut points: Vec<Vec3> = vec![];
        // f (v, v, v), (vt, vt, vt)
        let mut faces: Vec<((usize, usize, usize), (usize, usize, usize))> = vec![];
        let mut texture_points: Vec<Vec2<f32>> = vec![];
        // f v/vt/vn v/vt/vn v/vt/vn v/vt/vn
        // v x y z w
        for line in reader.lines() {
            let text = line.unwrap();
            let mut words = text.split_whitespace();
            let carac = words.next();
            match carac {
                Some("v") => {
                    // Coordonnées simples
                    points.push(Vec3 {
                        x: words.next().unwrap().parse::<f32>().unwrap(),
                        y: words.next().unwrap().parse::<f32>().unwrap(),
                        z: words.next().unwrap().parse::<f32>().unwrap(),
                    });
                }
                Some("f") => {
                    // Faces
                    let mut p1 = words.next().unwrap().split("/");
                    let mut p2 = words.next().unwrap().split("/");
                    let mut p3 = words.next().unwrap().split("/");
                    faces.push((
                        (p1.next().unwrap().parse::<usize>().unwrap(),
                         p2.next().unwrap().parse::<usize>().unwrap(),
                         p3.next().unwrap().parse::<usize>().unwrap()),
                        (p1.next().unwrap().parse::<usize>().unwrap(),
                         p2.next().unwrap().parse::<usize>().unwrap(),
                         p3.next().unwrap().parse::<usize>().unwrap())));
                }
                Some("vt") => {
                    texture_points.push(Vec2 {
                        x: words.next().unwrap().parse::<f32>().unwrap(),
                        y: words.next().unwrap().parse::<f32>().unwrap()
                    });
                }
                _ => {}
            }
        }
        println!("{:?}", points);
        println!("{:?}", faces);

        for ((p1, p2, p3), (t1, t2, t3)) in faces {
            if p1 <= points.len() || p2 <= points.len() || p3 <= points.len()
                || t1 <= texture_points.len() || t2 <= texture_points.len() || t2 <= texture_points.len(){
                let img_texture = textures::image::Image::new(img_texture_map.clone(),
                                                              texture_points[t1-1],
                                                              texture_points[t2-1],
                                                              texture_points[t3-1]);
                let new_triangle = Triangle::new(points[p1 - 1], points[p2 - 1], points[p3 - 1]);
                self.add_primitive(Box::new(new_triangle), material.clone(), Box::new(img_texture));
            }
        }
    }

    pub fn new(ambiant_light: Color, ambiant_power: f32) -> Scene {
        Scene {
            primitives: vec![],
            ambiant_light,
            ambiant_power,
        }
    }

    pub fn add_primitive(&mut self, hittable: Box<dyn Hittable>, material: Box<dyn Material>, texture: Box<dyn Texture>) {
        let primitive: Primitive = Primitive { hittable, material, texture };
        self.primitives.push(primitive);
    }


    pub fn background_color(&self, rayon: &Ray) -> Color {
        let r = rayon.normalized();
        Color { r: r.direction.x, g: r.direction.y, b: r.direction.z }
    }

    pub fn trace(&self, rayon: &Ray, max_iter: usize) -> Color {
        let mut closest_primitive: Option<&Primitive> = None;
        let mut closest_hitinfo: HitInfo = HitInfo {
            distance: std::f32::INFINITY,
            normal: Vec3::zero(),
            point: Vec3::zero(),
            rayon: *rayon,
            position: (0.0, 0.0),
        };

        // Search visible object
        for primitive in self.primitives.iter() {
            match primitive.hittable.compute_hit(&rayon) {
                Some(hitinfo) => {
                    if hitinfo.distance < closest_hitinfo.distance {
                        closest_primitive = Some(primitive);
                        closest_hitinfo = hitinfo;
                    }
                }
                _ => {}
            }
        }


        match closest_primitive {
            Some(object) => {
                // Get material color (color due to reflect, refract...)
                let color_info = object.material.get_color(&closest_hitinfo,    self, max_iter);
                // Get Texture color
                let texture_color = object.texture.get_color(&closest_hitinfo);
                return texture_color * (1.0 - color_info.ratio) + color_info.ratio * color_info.color;
            }
            _ => {}
        }


        // Get texture color
        self.background_color(rayon)
    }
}