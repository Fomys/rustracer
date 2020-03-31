use crate::raytracer::lights::Light;
use crate::raytracer::primitive::Primitive;
use crate::raytracer::scene::Scene;
use std::error::Error;
use std::io;
use std::io::BufRead;
use std::path::Path;
use std::sync::Arc;

pub trait Loader {
    fn load_as_scene(input: &Path) -> Result<Scene, std::io::Error> {
        let (primitives, lights) = Self::load(input)?;
        let mut scene = Scene::new();
        scene.add_primitives(primitives);
        scene.add_lights(lights);
        Ok(scene)
    }
    fn load(input: &Path) -> io::Result<(Vec<Arc<Primitive>>, Vec<Arc<dyn Light>>)>;
}
