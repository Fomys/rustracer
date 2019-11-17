use crate::raytracer::vec3::Vec3;
use crate::raytracer::hittables::hittable::Hittable;

pub struct Plane {
    pub origin: Vec3,
    pub dir1: Vec3,
    pub dir2: Vec3,
    pub material: Material,
}

impl Hittable for Plane {


    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        unimplemented!()
    }

    fn get_intersect(&self, rayon: &Ray) -> Option<Vec3> {
        unimplemented!()
    }

}