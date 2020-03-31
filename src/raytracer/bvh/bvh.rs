use std::mem::swap;
use std::sync::Arc;

use crate::raytracer::bvh::Binary;
use crate::raytracer::hittables::{HitInfo, Hittable};
use crate::raytracer::primitive::Primitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::Vec3;

pub trait BVH: Sync + Send {
    fn prt(&self, i: usize);
    fn box_intersect(&self, rayon: &Ray) -> bool {
        //println!("{:p}", self);
        let (min, max) = self.get_extremums();
        let mut tmin = std::f32::NEG_INFINITY;
        let mut tmax = std::f32::NEG_INFINITY;
        let mut txmin = (min.x - rayon.origin.x) / rayon.direction.x;
        let mut txmax = (max.x - rayon.origin.x) / rayon.direction.x;

        if txmin > txmax {
            swap(&mut txmin, &mut txmax);
        }

        let mut tymin = (min.y - rayon.origin.y) / rayon.direction.y;
        let mut tymax = (max.y - rayon.origin.y) / rayon.direction.y;

        if tymin > tymax {
            swap(&mut tymin, &mut tymax);
        }

        let mut tzmin = (min.z - rayon.origin.z) / rayon.direction.z;
        let mut tzmax = (max.z - rayon.origin.z) / rayon.direction.z;

        if tzmin > tzmax {
            swap(&mut tzmin, &mut tzmax);
        }

        /*println!(
            "{:?}, {:?} \n {}, {} - {}, {} - {}, {}",
            min, max, txmin, txmax, tymin, tymax, tzmin, tzmax
        );*/
        tmin = txmin;
        tmax = txmax;

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return false;
        }

        if (tmin > tymax) || (tymin > tmax) {
            return false;
        }

        true
    }
    fn is_inside(&self, bb: (Vec3, Vec3)) -> bool;
    fn insert_primitive(&self, primitive: Arc<Primitive>) -> Arc<dyn BVH>;
    fn compute_hit(&self, rayon: &Ray) -> (HitInfo, Option<Arc<Primitive>>);
    fn get_extremums(&self) -> (Vec3, Vec3);
}

pub struct Final {
    pub object: Arc<Primitive>,
}

impl BVH for Final {
    fn prt(&self, i: usize) {
        for _ in 0..i {
            print!(" ");
        }
        println!("- Final");
    }
    fn box_intersect(&self, rayon: &Ray) -> bool {
        true
    }
    fn is_inside(&self, bb: (Vec3, Vec3)) -> bool {
        true
    }

    fn insert_primitive(&self, primitive: Arc<Primitive>) -> Arc<dyn BVH> {
        Arc::new(Binary::new(
            Arc::new(Self {
                object: self.object.clone(),
            }),
            Arc::new(Self {
                object: primitive.clone(),
            }),
        ))
    }

    fn compute_hit(&self, rayon: &Ray) -> (HitInfo, Option<Arc<Primitive>>) {
        (
            self.object.hittable.compute_hit(rayon),
            Some(self.object.clone()),
        )
    }

    fn get_extremums(&self) -> (Vec3, Vec3) {
        let e = self.object.hittable.get_extremums();
        (e.0.min(&e.1), e.0.max(&e.1))
    }
}

pub struct Empty {}

impl BVH for Empty {
    fn prt(&self, i: usize) {
        for _ in 0..i {
            print!(" ");
        }
        println!("- Empty");
    }

    fn is_inside(&self, bb: (Vec3, Vec3)) -> bool {
        true
    }

    fn insert_primitive(&self, primitive: Arc<Primitive>) -> Arc<BVH> {
        Arc::new(Binary::new(
            Arc::new(Empty {}),
            Arc::new(Final {
                object: primitive.clone(),
            }),
        ))
    }

    fn compute_hit(&self, rayon: &Ray) -> (HitInfo, Option<Arc<Primitive>>) {
        (HitInfo::NONE, None)
    }

    fn get_extremums(&self) -> (Vec3, Vec3) {
        (Vec3::ZERO, Vec3::ZERO)
    }
}
