use rand::Rng;

use crate::raytracer::color::{Color, BLACK};
use crate::raytracer::hittables::HitInfo;
use crate::raytracer::materials::MaterialPrimitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::Vec3;
use crate::raytracer::utils::RAY_PER_REFLECT;

#[derive(Clone)]
pub struct Metal {
    pub fuzziness: f32,
}

impl MaterialPrimitive for Metal {
    fn get_color(
        &self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize, rng: &mut rand::XorShiftRng,
    ) -> Color {
        if max_iter > 0 {
            let mut new_color = BLACK;
            for _ in 0..RAY_PER_REFLECT {
                for _ in 0..RAY_PER_REFLECT {
                    let new_direction =
                        hitinfo.rayon.direction.reflect(hitinfo.normal).normalized()
                            + self.fuzziness
                                * Vec3 {
                                    x: rng.next_f32() * 2.0 - 1.0,
                                    y: rng.next_f32() * 2.0 - 1.0,
                                    z: rng.next_f32() * 2.0 - 1.0,
                                };
                    new_color += scene.trace(
                        &Ray {
                            origin: hitinfo.point + 0.1 * new_direction,
                            direction: new_direction,
                        },
                        max_iter - 1,
                        rng,
                    );
                }
            }
            return new_color / (RAY_PER_REFLECT as f32 * RAY_PER_REFLECT as f32);
        }
        BLACK
    }
}
