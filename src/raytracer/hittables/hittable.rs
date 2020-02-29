use std::cmp::Ordering;

use crate::raytracer::ray::Ray;
use crate::raytracer::utils::Vec3;

#[derive(Debug)]
pub struct HitInfo {
    pub distance: f32,
    pub normal: Vec3,
    pub point: Vec3,
    pub rayon: Ray,
    pub position: Vec3,
}

impl HitInfo {
    pub const NONE: Self = Self {
        distance: std::f32::INFINITY,
        normal: Vec3::ZERO,
        point: Vec3::ZERO,
        rayon: Ray::NONE,
        position: Vec3::ZERO,
    };
}

impl PartialOrd for HitInfo {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for HitInfo {
    fn eq(&self, other: &Self) -> bool {
        self.distance.eq(&other.distance)
    }
}

impl Ord for HitInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.distance.is_infinite() && other.distance.is_infinite() {
            return Ordering::Equal;
        }
        if self.distance.is_infinite() {
            return Ordering::Greater;
        }
        if other.distance.is_infinite() {
            return Ordering::Less;
        }
        if self.distance < other.distance {
            return Ordering::Less;
        }
        if self.distance > other.distance {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl Eq for HitInfo {}

pub trait Hittable: Sync + Send {
    fn compute_hit(&self, rayon: &Ray) -> HitInfo;
    fn next_pos(&mut self) {}
}
