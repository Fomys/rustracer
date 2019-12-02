use crate::raytracer::color::{Color, BLACK, RED};
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::materials::material::MaterialPrimitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::vec::Vec3;

#[derive(Clone)]
pub struct Diffuse {}

impl MaterialPrimitive for Diffuse {
    fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize, rng: &mut rand::XorShiftRng) -> Color {
        let mut new_color = BLACK;
        let mut i = 0;
        for light in scene.lights.iter() {
            let mut direction = light.get_position() - hitinfo.point;
            if !scene.launch_ray_min_dist(
                &Ray {
                    origin: hitinfo.point + 0.01 * direction,
                    direction,
                },
                direction.length(),
            ) {
                new_color += light.get_color() * Vec3::dot(&direction, &hitinfo.normal).max(0.0);
            }
        }
        new_color
    }
}
