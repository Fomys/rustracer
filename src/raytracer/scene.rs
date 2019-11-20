use crate::raytracer::hittables::hittable::{Hittable, HitInfo};
use crate::raytracer::materials::material::Material;
use crate::raytracer::color::Color;
use crate::raytracer::vec::Vec3;
use crate::raytracer::ray::Ray;
use crate::raytracer::primitive::Primitive;
use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::raytracer::hittables::triangle::Triangle;

pub struct Scene<'a> {
    primitives: Vec<Primitive<'a>>,
    pub ambiant_light: Color,
    pub ambiant_power: f32,
}

impl<'a> Scene<'a> {
    pub fn load_obj(&mut self, filepath: String, material: &'a dyn Material) {
        let file = match File::open(filepath) {
            Ok(f) => f,
            _ => { return },
        };
        let mut reader = BufReader::new(file);

        let mut points: Vec<Vec3> = vec![];
        let mut faces: Vec<(usize, usize, usize)> = vec![];
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
                        z: words.next().unwrap().parse::<f32>().unwrap() });
                }
                Some("f") => {
                    // Faces
                    faces.push((
                                   words.next().unwrap().split("/").next().unwrap().parse::<usize>().unwrap(),
                                   words.next().unwrap().split("/").next().unwrap().parse::<usize>().unwrap(),
                                   words.next().unwrap().split("/").next().unwrap().parse::<usize>().unwrap()));
                }
                _ => {}
            }
        }
        println!("{:?}", points);
        println!("{:?}", faces);

        for (p1, p2, p3) in faces {
            if p1 <= points.len() || p2 <= points.len() || p3 <= points.len() {
                let new_triangle: Box<dyn Hittable> = Box::new(Triangle::new(points[p1-1],
                                                                             points[p2-1], points[p3-1]));
                self.add_primitive(new_triangle, material);
            }
        }

    }

    pub fn new(ambiant_light: Color, ambiant_power: f32) -> Scene<'a> {
        Scene {
            primitives: vec![],
            ambiant_light,
            ambiant_power,
        }
    }

    pub fn add_primitive(&mut self, hittable: Box<dyn Hittable>, material: &'a dyn Material) {
        let primitive: Primitive = Primitive { hittable, material };
        self.primitives.push(primitive);
    }

    /*pub fn load_obj(&'a mut self, filename: String) {
        let material: Box<dyn Material> = Box::new( Plain { color: Color { r: 0.0, g: 0.0, b: 0.0 }});
        let matref: &'static Box<dyn Material> = &material;
        let input = File::open(filename).unwrap();
        let buffered = BufReader::new(input);
        let mut points: Vec<Vec3> = vec![];
        for line in buffered.lines() {
            let line = line.unwrap();
            if line.starts_with("v") {
                let v: Vec<&str> = line.split(" ").collect();
                points.push(Vec3 {
                    x: v[1].parse::<f32>().unwrap(),
                    y: v[2].parse::<f32>().unwrap(),
                    z: v[3].parse::<f32>().unwrap(),
                });
            } else if line.starts_with("f") {
                let v: Vec<&str> = line.split(" ").collect();
                let a: Vec<&str> = v[1].split("/").collect();
                let b: Vec<&str> = v[2].split("/").collect();
                let c: Vec<&str> = v[3].split("/").collect();
                let ia: usize = a[0].parse::<usize>().unwrap();
                let ib: usize = b[0].parse::<usize>().unwrap();
                let ic: usize = c[0].parse::<usize>().unwrap();
                let new_triangle: Box<dyn Hittable> = Box::new(Triangle::new(points[ia],
                                                 points[ib],
                                                 points[ic]));
                self.add_object(&new_triangle, matref);
            }
        }
    }*/

    pub fn background_color(&self, rayon: &Ray) -> Color {
        Color { r: 0.0, g: 0.0, b: 1.0 }
    }

    pub fn trace(&self, rayon: &Ray, max_iter: usize) -> Color {
        let mut closest_primitive: Option<&Primitive> = None;
        let mut closest_hitinfo: HitInfo = HitInfo {
            distance: std::f32::INFINITY,
            normal: Vec3::zero(),
            point: Vec3::zero(),
            rayon: *rayon
        };
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
                return object.material.get_color(&closest_hitinfo, self, max_iter);
            }
            _ => {}
        }
        self.background_color(rayon)
    }
}