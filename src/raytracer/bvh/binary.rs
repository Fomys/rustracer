use std::sync::Arc;

use crate::raytracer::bvh::bvh::Final;
use crate::raytracer::bvh::BVH;
use crate::raytracer::hittables::{HitInfo, Hittable};
use crate::raytracer::primitive::Primitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::Vec3;

pub struct Binary {
    bvh1: Arc<dyn BVH>,
    bvh2: Arc<dyn BVH>,
    extremums: (Vec3, Vec3),
}

impl Binary {
    pub fn new(bvh1: Arc<dyn BVH>, bvh2: Arc<dyn BVH>) -> Self {
        let a = bvh1.get_extremums();
        let b = bvh2.get_extremums();
        Self {
            bvh1,
            bvh2,
            extremums: (
                a.0.min(&a.1).min(&b.0).min(&b.1),
                a.0.max(&a.1).max(&b.0).max(&b.1),
            ),
        }
    }
}

impl BVH for Binary {
    fn prt(&self, i: usize) {
        for _ in 0..i {
            print!(" ");
        }
        println!("- Binary");
        self.bvh1.prt(i + 1);
        self.bvh2.prt(i + 1);
    }

    fn is_inside(&self, bb: (Vec3, Vec3)) -> bool {
        if self.extremums.0.max(&bb.0) != bb.0 {
            return false;
        }
        if self.extremums.1.min(&bb.1) != bb.1 {
            return false;
        }
        true
    }

    fn insert_primitive(&self, primitive: Arc<Primitive>) -> Arc<dyn BVH> {
        if self.bvh1.is_inside(primitive.hittable.get_extremums()) {
            return Arc::new(Binary::new(
                self.bvh1.insert_primitive(primitive),
                self.bvh2.clone(),
            ));
        }
        if self.bvh2.is_inside(primitive.hittable.get_extremums()) {
            return Arc::new(Binary::new(
                self.bvh1.clone(),
                self.bvh2.insert_primitive(primitive),
            ));
        }
        Arc::new(Binary::new(
            Arc::new(Binary::new(self.bvh1.clone(), self.bvh2.clone())),
            Arc::new(Final {
                object: primitive.clone(),
            }),
        ))
    }

    fn compute_hit(&self, rayon: &Ray) -> (HitInfo, Option<Arc<Primitive>>) {
        let (hit1, prim1) = if self.bvh1.box_intersect(rayon) {
            self.bvh1.compute_hit(rayon)
        } else {
            (HitInfo::NONE, None)
        };
        let (hit2, prim2) = if self.bvh2.box_intersect(rayon) {
            self.bvh2.compute_hit(rayon)
        } else {
            (HitInfo::NONE, None)
        };
        if hit1 < hit2 {
            return (hit1, prim1);
        }
        return (hit2, prim2);
    }

    fn get_extremums(&self) -> (Vec3, Vec3) {
        self.extremums
    }
}
