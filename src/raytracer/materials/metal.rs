use crate::raytracer::color::{Color, ColorInfo};
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::materials::material::Material;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;

#[derive(Clone)]
pub struct Metal {
    pub reflection_factor: f32,
}

impl Material for Metal {
    fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize) -> ColorInfo {
        if self.reflection_factor > 0.001 {
            if max_iter > 0 {
                let new_direction = hitinfo.rayon.reflect(&hitinfo.normal).normalized();
                let other_color = scene.trace(
                    &Ray {
                        origin: hitinfo.point + 0.1 * new_direction,
                        direction: new_direction,
                    },
                    max_iter - 1,
                );
                return ColorInfo { color: other_color, ratio: self.reflection_factor };
            }
        }
        return ColorInfo { color: Color::black(), ratio: 0.0 };
    }
}