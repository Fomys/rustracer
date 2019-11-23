use crate::raytracer::primitive::Primitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::vec::Vec3;

pub struct Object<'a> {
    primitives: Vec<Primitive<'a>>,
    corner1: Vec3,
    corner2: Vec3,
}

impl Object<'_> {
    pub fn new(primitives: Vec<Primitive<'a>>) -> Object {
        let mut min: Vec3 = Vec3 { x: std::f32::INFINITY, y: std::f32::INFINITY, z: std::f32::INFINITY };
        let mut max: Vec3 = Vec3 { x: std::f32::NEG_INFINITY, y: std::f32::NEG_INFINITY, z: std::f32::NEG_INFINITY };
        let mut object = Object { primitives, corner1: min, corner2: max };
        object.compute_border();
        object
    }

    fn compute_border(&mut self) {
        for object in self.primitives.iter() {
            let (temp_min, temp_max) = object.hittable.extremums();
            self.min = Vec3::min(self.min, temp_min);
            self.max = Vec3::max(self.max, temp_max);
        }
    }

    pub fn add_primitive(&mut self, primitive: Primitive) {
        self.primitives.push(primitive);
        self.compute_border();
    }

    pub fn cut(&self, rayon: &Ray) -> bool {
        let mut tmin: f32;
        let mut tmax: f32;
        let tymin: f32;
        let tymax: f32;
        let tzmin: f32;
        let tzmax: f32;

        // compute minimum and maximum t projected on x
        if rayon.direction.x >= 0.0 {
            tmin = (self.corner1.x - rayon.origin.x) / rayon.direction.x;
            tmax = (self.corner2.x - rayon.origin.x) / rayon.direction.x;
        } else {
            tmin = (self.corner2.x - rayon.origin.x) / rayon.direction.x;
            tmax = (self.corner2.x - rayon.origin.x) / rayon.direction.x;
        }

        // compute minimum and maximum t projected on y
        if rayon.direction.y >= 0.0 {
            tymin = (self.corner1.y - rayon.origin.y) / rayon.direction.y;
            tymax = (self.corner2.y - rayon.origin.y) / rayon.direction.y;
        } else {
            tymin = (self.corner2.y - rayon.origin.y) / rayon.direction.y;
            tymax = (self.corner1.y - rayon.origin.y) / rayon.direction.y;
        }

        // If values are incompatible there isn't hit
        if tmin > tymax || tymin > tmax { return false; };
        // Keep true minimum and maximum in tmin and tmax
        if tymin > tmin { tmin = tymin; }
        if tymax < tmax { tmax = tymax; }

        // compute minimum and maximum t projected on z
        if rayon.direction.z >= 0.0 {
            tzmin = (self.corner1.z - rayon.origin.z) / rayon.direction.z;
            tzmax = (self.corner2.z - rayon.origin.z) / rayon.direction.z;
        } else {
            tzmin = (self.corner2.z - rayon.origin.z) / rayon.direction.z;
            tzmax = (self.corner1.z - rayon.origin.z) / rayon.direction.z;
        }

        // If values are incompatible there isn't hit
        if tmin > tzmax || tzmin > tmax { return false; };
        // Keep true minimum and maximum in tmin and tmax
        if tzmin > tmin { tmin = tzmin; }
        if tzmax < tmax { tmax = tzmax; }

        return (tmin > 0.0);
    }
}