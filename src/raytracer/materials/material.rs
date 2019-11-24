use crate::raytracer::color::ColorInfo;
use crate::raytracer::hittables::hittable::HitInfo;
use crate::raytracer::scene::Scene;

pub trait Material: MaterialClone + Sync + Send {
    fn get_color(&self, hitinfo: &HitInfo, scene: &Scene, max_iter: usize) -> ColorInfo;
}


pub trait MaterialClone {
    fn clone_box(&self) -> Box<dyn Material>;
}

impl<T> MaterialClone for T
    where T: 'static + Material + Clone,
{
    fn clone_box(&self) -> Box<dyn Material> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn Material> {
    fn clone(&self) -> Box<dyn Material> {
        self.clone_box()
    }
}