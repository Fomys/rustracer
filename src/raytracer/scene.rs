use crate::raytracer::hittable::Hittable;
use crate::raytracer::ray::Ray;
use crate::raytracer::color::Color;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn background_color(&self, rayon: Ray) -> Color{
        Color {r: 0.0, g: 0.0, b: 1.0}
    }

    pub fn trace (&self, rayon: Ray) -> Color {
        let mut closest_object: Option<&Box<dyn Hittable>> = None;
        let mut closest_distance: f32 = std::f32::INFINITY;
        for object in self.objects.iter() {
            match object.compute_hit(&rayon) {
                Some(dist) => {
                    if(dist < closest_distance) {
                        closest_object = Some(object);
                        closest_distance = dist;
                    }
                }
                _ => {}
            }
        }
        match closest_object {
            Some(object) => {
                object.get_color(&rayon)
            }
            _ => {
                self.background_color(rayon)
            }
        }
    }
}