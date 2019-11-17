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
    pub fn background_color(&self, rayon: &Ray) -> Color{
        Color {r: 0.0, g: 0.0, b: 1.0}
    }

    pub fn trace (&self, rayon: &Ray, max_iter: usize) -> Color {
        let mut closest_object: Option<&Box<dyn Hittable>> = None;
        let mut closest_hitinfo: HitInfo = HitInfo {distance: std::f32::INFINITY, normal: Vec3::zero(), point:Vec3::zero()};
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
                /*let reflection_factor = object.get_reflect_factor(closest_hitinfo.point);

                if reflection_factor > 0.001 {
                    if max_iter > 0 {
                        closest_hitinfo.normal.normalize();
                        let new_direction = rayon.reflect(&closest_hitinfo.normal);
                        rayon.reflect(&closest_hitinfo.normal).normalize();
                        let other_color = self.trace(
                            Ray {origin: closest_hitinfo.point + 0.1 * new_direction, direction: new_direction},
                            max_iter - 1,
                        );
                        return (object.get_color(&rayon) * self.ambiant_light * self.ambiant_power * (1.0-reflection_factor)) + reflection_factor * other_color
                        //return reflection_factor * other_color
                    } else {
                        return object.get_color(&rayon) * self.ambiant_light * self.ambiant_power;
                    }
                }*/
                return object.get_material().get_color(&closest_hitinfo, self)
            }
            _ => {

            }
        }
        self.background_color(rayon)
    }
}