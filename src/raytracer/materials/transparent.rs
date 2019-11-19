use crate::raytracer::color::Color;
use crate::raytracer::vec::Vec3;
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::materials::material::Material;
use crate::raytracer::scene::Scene;
use crate::raytracer::ray::Ray;

pub struct Transparent {
    pub color: Color,
    pub refractive_index_div: f32,
}

impl Transparent {
    fn refract(&self, direction: &Vec3, normal: &Vec3) -> Option<Vec3> {
        let uv = &direction.normalized();
        let dt = Vec3::dot(uv, normal);
        let discriminant = 1.0 - self.refractive_index_div * self.refractive_index_div * (1.0 - dt*dt);
        if discriminant > 0.0 {
            return Some(self.refractive_index_div * (uv - dt * normal) - &(discriminant.sqrt() * normal))
        }
        None
    }
}

impl Material for Transparent {

    fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize) -> Color {
        if max_iter > 0 {
            // Reflected ray
            let new_direction = hitinfo.rayon.reflect(&hitinfo.normal).normalized();
            let reflected_color = scene.trace(
                &Ray {
                    origin: hitinfo.point + 0.1 * new_direction,
                    direction: new_direction,},
                max_iter - 1,
                );
            // Refracted ray
            let mut refracted_color: Option<Color> = None;
            match self.refract(&hitinfo.rayon.direction, &hitinfo.normal) {
                Some(tt) => {
                    refracted_color = Some(scene.trace(
                        &Ray {
                            origin: hitinfo.point + 0.1*tt,
                            direction: tt,
                        },
                        max_iter - 1,
                    ));
                }
                _ => {}
            }

            // Compute color
            match refracted_color {
                Some(toto) => {
                    return toto
                }
                _ => {
                    return reflected_color
                }
            }
        }

        self.color
    }
}