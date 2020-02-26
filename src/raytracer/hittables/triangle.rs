use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::movements::movement::MovementPrimitive;
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::{Vec3, ZERO, ZERO_VEC3};

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    normal: Vec3,
    edge0: Vec3,
    edge1: Vec3,
    edge2: Vec3,
}

impl Triangle {
    #[allow(dead_code)]
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        let edge0 = b - a;
        let edge1 = c - a;
        let edge2 = c - b;
        let normal = edge0 ^ edge1;

        Triangle {
            a,
            b,
            c,
            normal,
            edge0,
            edge1,
            edge2,
        }
    }

    fn update(&mut self) {
        self.edge0 = self.b - self.a;
        self.edge1 = self.c - self.a;
        self.edge2 = self.c - self.b;
        self.normal = self.edge0 ^ self.edge1;
    }
}

impl Hittable for Triangle {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let denom = self.normal | rayon.direction;
        // Check if ray intersect triangle plane
        if denom.abs() >= ZERO {
            // Find intersection with plane
            let t = ((self.a - rayon.origin) | self.normal) / denom;
            if t >= 0.0 {
                let intersection = rayon.point_at(t);
                // Check if interestion is in triangle
                let vp0 = intersection - self.a;
                let c = self.edge0 ^ vp0;
                if (self.normal | c) < 0.0 {
                    return None;
                }
                let vp1 = intersection - self.c;
                let c = -self.edge1 ^ vp1;
                if (self.normal | c) < 0.0 {
                    return None;
                }
                let vp2 = intersection - self.b;
                let c = self.edge2 ^ vp2;
                if (self.normal | c) < 0.0 {
                    return None;
                }
                return Some(HitInfo {
                    distance: t,
                    normal: self.normal,
                    point: intersection,
                    rayon: *rayon,
                    position: ZERO_VEC3,
                });
            } else {
            }
        }

        None
    }

    fn next_pos(&mut self) {
        let movements = self.movements.next_movements();
        for movement in movements {
            match movement {
                MovementPrimitive::Translation(distance) => {
                    self.a += distance;
                    self.b += distance;
                    self.c += distance;
                    self.update();
                }
                MovementPrimitive::Scale(scale) => {
                    // 3/2 * 1/2 * (AB + AC) = AG
                    let center = self.a + 3.0 / 4.0 * (self.edge0 + self.edge1);
                    let ga = -scale * 3.0 / 2.0 * 0.5 * (self.edge0 + self.edge1);
                    let gb = -scale * 3.0 / 2.0 * 0.5 * (-self.edge0 + self.edge2);
                    let gc = scale * 3.0 / 2.0 * 0.5 * (self.edge1 + self.edge2);
                    self.a = center + ga;
                    self.b = center + gb;
                    self.c = center + gc;
                }
                MovementPrimitive::Cycle(_) => {
                    // Nothing here
                }
            }
        }
    }
}
