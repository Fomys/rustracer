use crate::raytracer::color::Color;

// Le truc que j'aimerais partager entre les différentes textures, c'est un trait car je sais pas
// si c'est possible d'avoir une texturemap sous un autre format
pub trait TextureMap: TextureMapClone + Sync + Send {
    fn get_pixel(&self, x: f32, y: f32) -> Color;
}


// Truc moche pour cloner une box
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