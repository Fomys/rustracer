use crate::raytracer::color::{Color, BLACK};
use crate::raytracer::hittables::HitInfo;
use crate::raytracer::materials::material::MaterialPrimitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;

#[derive(Clone)]
pub struct Diffuse {}

impl MaterialPrimitive for Diffuse {
    fn get_color(
        &self, hitinfo: &HitInfo, scene: &Scene, _max_iter: usize, rng: &mut rand::XorShiftRng,
    ) -> Color {
        let mut new_color = BLACK;
        let mut i = 0;
        for light in scene.lights.iter() {
            for position in light.get_positions(rng).iter() {
                i += 1;
                let direction = *position - hitinfo.point;
                if scene.launch_ray_min_dist(
                    &Ray {
                        origin: hitinfo.point + 0.01 * direction,
                        direction,
                    },
                    direction.length(),
                ) == HitInfo::NONE
                {
                    new_color += light.get_color(direction) * (direction | hitinfo.normal).abs();
                }
            }
        }
        new_color / i as f32
    }
}
