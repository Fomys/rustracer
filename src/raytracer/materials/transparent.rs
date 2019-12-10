use crate::raytracer::color::{Color, ColorInfo, BLACK, RED};
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::materials::material::{Material, MaterialPrimitive};
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::vec::Vec3;
use rand::XorShiftRng;

pub struct Transparent {
    pub refractive_index_div: f32,
}

impl Transparent {
    fn refract(&self, direction: &Vec3, normal: &Vec3) -> Option<Vec3> {
        let uv = direction.normalized();
        let dt = Vec3::dot(&uv, normal);
        let discriminant =
            1.0 - self.refractive_index_div * self.refractive_index_div * (1.0 - dt * dt);
        if discriminant > 0.0 {
            return Some(
                self.refractive_index_div * (uv - normal * dt) - normal * discriminant.sqrt(),
            );
        }
        None
    }
}

impl MaterialPrimitive for Transparent {
    fn get_color(
        &self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize, rng: &mut XorShiftRng,
    ) -> Color {
        if max_iter > 0 {
            if let Some(new_ray) = self.refract(&hitinfo.rayon.direction, &hitinfo.normal) {
                return scene.trace(
                    &Ray {
                        origin: hitinfo.point + hitinfo.rayon.direction * 0.01,
                        direction: new_ray,
                    },
                    max_iter - 1,
                    rng,
                );
            }
        }
        BLACK
    }
}
