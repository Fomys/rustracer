use rand::XorShiftRng;

use crate::raytracer::color::{Color, BLACK};
use crate::raytracer::hittables::HitInfo;
use crate::raytracer::materials::MaterialPrimitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::{Vec3, OFFSET};

pub struct Transparent {
    pub refractive_index_div: f32,
}

impl Transparent {
    fn refract(&self, direction: &Vec3, normal: Vec3) -> Option<Vec3> {
        let uv = direction.normalized();
        let dt = uv | normal;
        let discriminant =
            1.0 - self.refractive_index_div * self.refractive_index_div * (1.0 - dt * dt);
        if discriminant > 0.0 {
            return Some(
                self.refractive_index_div * (uv - dt * normal) - discriminant.sqrt() * normal,
            );
        }
        None
    }
}

impl MaterialPrimitive for Transparent {
    fn get_color(
        &self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize, rng: &mut XorShiftRng,
    ) -> Color {
        if max_iter > 0
            && self
                .refract(&hitinfo.rayon.direction, hitinfo.normal)
                .is_some()
        {
            return scene.trace(
                &Ray {
                    origin: hitinfo.point + OFFSET * hitinfo.rayon.direction,
                    direction: hitinfo.rayon.direction,
                },
                max_iter - 1,
                rng,
            );
        }
        BLACK
    }
}
