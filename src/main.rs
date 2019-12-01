use crate::raytracer::camera::Camera;
use crate::raytracer::color::{BLACK, Color};
use crate::raytracer::integrator::integrator::Integrator;
use crate::raytracer::integrator::simple_integrator::SimpleIntegrator;
use crate::raytracer::scene::Scene;
use crate::raytracer::utils::vec::{Vec2, Vec3};
use crate::raytracer::hittables::sphere::Sphere;
use crate::raytracer::{textures, materials};

mod raytracer;

fn main() {
    let camera = Camera::new(
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        Vec2 { x: 4.0, y: 2.0 },
        Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        Vec2 { x: 1366, y: 768 },
    );
    let mut scene = Scene::new(BLACK, 0.1);

    let sphere = Sphere::new(Vec3 {x: 0.0, y: 0.0, z: 5.0}, 1.0);
    let plain = materials::plain::Plain {};
    let plain_color = textures::plain::Plain { color: Color {r: 1.0, g:0.0, b:0.0} };

    scene.add_primitive(Box::new(sphere), Box::new(plain), Box::new(plain_color));

    let mut integrator = SimpleIntegrator::new(camera, scene);

    integrator.render();
}
