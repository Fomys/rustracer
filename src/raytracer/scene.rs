use crate::raytracer::hittables::hittable::{Hittable, HitInfo};
use crate::raytracer::ray::Ray;
use crate::raytracer::color::Color;
use crate::raytracer::vec3::Vec3;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
    pub ambiant_light: Color,
    pub ambiant_power: f32,
}

impl Scene {
    pub fn background_color(&self, rayon: &Ray) -> Color {
        Color { r: 0.0, g: 0.0, b: 1.0 }
    }

    pub fn trace(&self, rayon: &Ray, max_iter: usize) -> Color {
        let mut closest_object: Option<&Box<dyn Hittable>> = None;
        let mut closest_hitinfo: HitInfo = HitInfo {
            distance: std::f32::INFINITY,
            normal: Vec3::zero(),
            point: Vec3::zero(),
            rayon: *rayon
        };
        for object in self.objects.iter() {
            match object.compute_hit(&rayon) {
                Some(hitinfo) => {
                    if hitinfo.distance < closest_hitinfo.distance {
                        closest_object = Some(object);
                        closest_hitinfo = hitinfo;
                    }
                }
                _ => {}
            }
        }

        match closest_object {
            Some(object) => {
                return object.get_material().get_color(&closest_hitinfo, self, max_iter);
            }
            _ => {}
        }
        self.background_color(rayon)
    }
}