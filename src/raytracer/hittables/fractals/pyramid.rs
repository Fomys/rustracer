use crate::raytracer::hittables::{HitInfo, Hittable, Triangle};
use crate::raytracer::movements::movement::Movement;
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::{Vec3, ZERO};
use core::num::FpCategory::Zero;

pub struct Pyramid {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    d: Vec3,
    max_depth: usize,
}

impl Pyramid {
    pub fn new(a: Vec3, b: Vec3, c: Vec3, d: Vec3, max_depth: usize) -> Self {
        Self {
            a,
            b,
            c,
            d,
            max_depth,
        }
    }

    fn intersect_pyramid(
        a: &Vec3, b: &Vec3, c: &Vec3, d: &Vec3, rayon: &Ray, max_depth: usize,
    ) -> HitInfo {
        //http://webserver2.tecgraf.puc-rio.br/~mgattass/cg/trbRR/Fast%20MinimumStorage%20RayTriangle%20Intersection.pdf
        let edge_ab = b - a; // edge1
        let edge_ac = c - a; // edge2

        let edge_ad = d - a;
        let edge_bd = d - b;
        let edge_dc = c - d;
        let edge_bc = c - b;

        let mut hit_1 = HitInfo::NONE;
        let mut hit_2 = HitInfo::NONE;
        let mut hit_3 = HitInfo::NONE;
        let mut hit_4 = HitInfo::NONE;

        let pvec_ac = rayon.direction ^ edge_ac;
        let tvec_ao = rayon.origin - *a;

        // a-b-c
        let tri1_det = edge_ab | pvec_ac;
        if tri1_det.abs() >= ZERO {
            let tri1_inv_det = 1.0 / tri1_det;
            let u = tri1_inv_det * (tvec_ao | pvec_ac);
            if 0.0 < u && u < 1.0 {
                let tri1_qvec = tvec_ao ^ edge_ab;
                let v = tri1_inv_det * (rayon.direction | tri1_qvec);
                if 0.0 < v && u + v < 1.0 {
                    let t = tri1_inv_det * (edge_ac | tri1_qvec);
                    hit_1 = HitInfo {
                        distance: t,
                        normal: edge_ab ^ edge_ac,
                        point: rayon.point_at(t),
                        rayon: *rayon,
                        position: Vec3::ZERO,
                    }
                }
            }
        }

        // a-d-c

        let tri2_det = edge_ad | pvec_ac;
        if tri2_det.abs() >= ZERO {
            let tri2_inv_det = 1.0 / tri2_det;
            let u = tri2_inv_det * (tvec_ao | pvec_ac);
            if 0.0 < u && u < 1.0 {
                let tri2_qvec = tvec_ao ^ edge_ad;
                let v = tri2_inv_det * (rayon.direction | tri2_qvec);
                if 0.0 < v && u + v < 1.0 {
                    let t = tri2_inv_det * (edge_ac | tri2_qvec);
                    hit_2 = HitInfo {
                        distance: t,
                        normal: edge_ac ^ edge_ad, // TODO: c'est la meme normale quel que soit la profondeur
                        point: rayon.point_at(t),
                        rayon: *rayon,
                        position: Vec3::ZERO,
                    }
                }
            }
        }

        let pvec_bd = rayon.direction ^ edge_bd;
        let tvec_bo = rayon.origin - *b;

        // b-a-d

        let tri3_det = -edge_ab | pvec_bd;
        if tri3_det.abs() >= ZERO {
            let tri3_inv_det = 1.0 / tri3_det;
            let u = tri3_inv_det * (tvec_bo | pvec_bd);
            if 0.0 < u && u < 1.0 {
                let tri3_qvec = tvec_bo ^ (-edge_ab);
                let v = tri3_inv_det * (rayon.direction | tri3_qvec);
                if 0.0 < v && u + v < 1.0 {
                    let t = tri3_inv_det * (edge_bd | tri3_qvec);
                    hit_3 = HitInfo {
                        distance: t,
                        normal: edge_ab ^ edge_bd,
                        point: rayon.point_at(t),
                        rayon: *rayon,
                        position: Vec3::ZERO,
                    }
                }
            }
        }

        // b-c-d

        let tri4_det = edge_bc | pvec_bd;
        if tri4_det.abs() >= ZERO {
            let tri4_inv_det = 1.0 / tri4_det;
            let u = tri4_inv_det * (tvec_bo | pvec_bd);
            if 0.0 < u && u < 1.0 {
                let tri4_qvec = tvec_bo ^ edge_bc;
                let v = tri4_inv_det * (rayon.direction | tri4_qvec);
                if 0.0 < v && u + v < 1.0 {
                    let t = tri4_inv_det * (edge_bd | tri4_qvec);
                    hit_4 = HitInfo {
                        distance: t,
                        normal: edge_bc ^ edge_bd,
                        point: rayon.point_at(t),
                        rayon: *rayon,
                        position: Vec3::ZERO,
                    }
                }
            }
        }

        if (hit_1 == HitInfo::NONE
            && hit_2 == HitInfo::NONE
            && hit_3 == HitInfo::NONE
            && hit_4 == HitInfo::NONE)
            || max_depth == 0
        {
            return hit_1.min(hit_2).min(hit_3).min(hit_4);
        }

        let mid_1 = 0.5 * (a + b);
        let mid_2 = 0.5 * (b + c);
        let mid_3 = 0.5 * (a + c);
        let mid_4 = 0.5 * (a + d);
        let mid_5 = 0.5 * (b + d);
        let mid_6 = 0.5 * (c + d);

        let hit_1 = Self::intersect_pyramid(b, &mid_2, &mid_1, &mid_5, rayon, max_depth - 1);
        let hit_2 = Self::intersect_pyramid(a, &mid_1, &mid_3, &mid_4, rayon, max_depth - 1);
        let hit_3 = Self::intersect_pyramid(c, &mid_2, &mid_3, &mid_6, rayon, max_depth - 1);
        let hit_4 = Self::intersect_pyramid(d, &mid_6, &mid_4, &mid_5, rayon, max_depth - 1);

        hit_1.min(hit_2).min(hit_3).min(hit_4)
    }
}

impl Hittable for Pyramid {
    fn compute_hit(&self, rayon: &Ray) -> HitInfo {
        Self::intersect_pyramid(&self.a, &self.b, &self.c, &self.d, rayon, self.max_depth)
    }
}
