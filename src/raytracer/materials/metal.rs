use crate::raytracer::color::{Color, ColorInfo, BLACK};
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::materials::material::MaterialPrimitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;

#[derive(Clone)]
pub struct Metal {}

impl MaterialPrimitive for Metal {
    fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize) -> Color {
        if max_iter > 0 {
            let new_direction = hitinfo.rayon.reflect(&hitinfo.normal).normalized();
            let other_color = scene.trace(
                &Ray {
                    origin: hitinfo.point + 0.1 * new_direction,
                    direction: new_direction,
                },
                max_iter - 1,
            );
            return other_color;
        }
        BLACK
    }
}
