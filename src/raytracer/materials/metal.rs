use crate::raytracer::color::Color;
use crate::raytracer::materials::material::Material;
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::scene::Scene;
use crate::raytracer::ray::Ray;

pub struct Metal {
    pub color: Color,
    pub reflection_factor: f32,
}

impl Material for Metal {
    fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize) -> Color {
        if self.reflection_factor > 0.001 {
            if max_iter > 0 {
                let new_direction = hitinfo.rayon.reflect(&hitinfo.normal).normalized();
                let other_color = scene.trace(
                    &Ray {origin: hitinfo.point + 0.1 * new_direction, direction: new_direction},
                    max_iter - 1,
                );
                return (self.color * scene.ambiant_light * scene.ambiant_power * (1.0 - self.reflection_factor))
                    + self.reflection_factor * other_color
            } else {
                return self.color * scene.ambiant_light * scene.ambiant_power;
            }
        }
        Color::black()
    }
}