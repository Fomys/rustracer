use crate::raytracer::color::Color;

pub trait TextureMap: TextureMapClone {
    fn get_pixel(&self, x: f32, y: f32) -> Color;
}

pub trait TextureMapClone {
    fn clone_box(&self) -> Box<dyn TextureMap>;
}

impl<T> TextureMapClone for T
    where T: 'static + TextureMap + Clone,
{
    fn clone_box(&self) -> Box<dyn TextureMap> {
        Box::new(self.clone())
    }
}

// We can now implement Clone manually by forwarding to clone_box.
impl Clone for Box<dyn TextureMap> {
    fn clone(&self) -> Box<dyn TextureMap> {
        self.clone_box()
    }
}