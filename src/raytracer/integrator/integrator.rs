use crate::raytracer::color::Color;
use crate::raytracer::ray::Ray;
use crate::raytracer::scene::Scene;

pub trait Integrator {
    fn preprocess(&mut self);
    fn render(&mut self);
}