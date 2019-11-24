use crate::raytracer::hittables::hittable::{HitInfo, Hittable};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::vec::Vec3;
use crate::raytracer::utils::consts;

pub struct Triangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
    normal: Vec3,
    edge0: Vec3,
    edge1: Vec3,
    edge2: Vec3,
    edge0lenght_square: f32,
    edge1lenght_square: f32,
    mincoord: Vec3,
    maxcoord: Vec3,
    matrice_passage: [f32; 9],
    matrice_passage_inverse: [f32; 9],
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        let edge0 = b - a;
        let edge1 = c - a;
        let edge2 = c - b;
        let normal = Vec3::cross_product(&edge0, &edge1);

        // Calcul de la matrice de passage
        let matrice_passage = [
            edge0.x, edge1.x, normal.x,
            edge0.y, edge1.y, normal.y,
            edge0.z, edge1.z, normal.z,
        ];
        let det = matrice_passage[0] * matrice_passage[4] * matrice_passage[8] +
            matrice_passage[1] * matrice_passage[5] * matrice_passage[6] +
            matrice_passage[2] * matrice_passage[3] * matrice_passage[7] -
            matrice_passage[2] * matrice_passage[4] * matrice_passage[6] -
            matrice_passage[5] * matrice_passage[7] * matrice_passage[0] -
            matrice_passage[8] * matrice_passage[1] * matrice_passage[3];
        let inv_det = 1.0 / det;
        // Inverse de la matrice de passage
        let matrice_passage_inverse = [
            inv_det * (matrice_passage[4] * matrice_passage[8] - matrice_passage[5] * matrice_passage[7]),
            inv_det * (matrice_passage[2] * matrice_passage[7] - matrice_passage[1] * matrice_passage[8]),
            inv_det * (matrice_passage[1] * matrice_passage[5] - matrice_passage[2] * matrice_passage[4]),
            inv_det * (matrice_passage[5] * matrice_passage[6] - matrice_passage[3] * matrice_passage[8]),
            inv_det * (matrice_passage[0] * matrice_passage[8] - matrice_passage[2] * matrice_passage[6]),
            inv_det * (matrice_passage[2] * matrice_passage[3] - matrice_passage[0] * matrice_passage[5]),
            inv_det * (matrice_passage[3] * matrice_passage[7] - matrice_passage[4] * matrice_passage[6]),
            inv_det * (matrice_passage[1] * matrice_passage[6] - matrice_passage[0] * matrice_passage[7]),
            inv_det * (matrice_passage[0] * matrice_passage[4] - matrice_passage[1] * matrice_passage[3]),
        ];
        Triangle {
            a,
            b,
            c,
            normal,
            edge0,
            edge1,
            edge2,
            edge0lenght_square: edge0.length().powf(2.0),
            edge1lenght_square: edge1.length().powf(2.0),
            mincoord: Vec3::min(a, Vec3::min(b, c)),
            maxcoord: Vec3::max(a, Vec3::max(b, c)),
            matrice_passage,
            matrice_passage_inverse,
        }
    }
}

impl Hittable for Triangle {
    fn compute_hit(&self, rayon: &Ray) -> Option<HitInfo> {
        let denom = Vec3::dot(&self.normal, &rayon.direction);
        if denom.abs() >= consts::ZERO {
            // Find intersection with plane
            let t = Vec3::dot(&(self.a - &rayon.origin), &self.normal) / denom;
            if t >= 0.0 {
                let intersection = rayon.point_at(t);
                // Check if interestion is in triangle
                let vp0 = intersection - self.a;
                let c = Vec3::cross_product(&self.edge0, &vp0);
                if Vec3::dot(&self.normal, &c) < 0.0 {
                    return None;
                }
                let vp1 = intersection - self.c;
                let c = Vec3::cross_product(&(-self.edge1), &vp1);
                if Vec3::dot(&self.normal, &c) < 0.0 {
                    return None;
                }
                let vp2 = intersection - self.b;
                let c = Vec3::cross_product(&self.edge2, &vp2);
                if Vec3::dot(&self.normal, &c) < 0.0 {
                    return None;
                }
                let offset_intersect = intersection - self.a;
                return Some(HitInfo {
                    distance: t,
                    normal: self.normal,
                    point: intersection,
                    rayon: *rayon,
                    position: Vec3 {
                        x: self.matrice_passage_inverse[0] * offset_intersect.x +
                            self.matrice_passage_inverse[1] * offset_intersect.y +
                            self.matrice_passage_inverse[2] * offset_intersect.z,
                        y: self.matrice_passage_inverse[3] * offset_intersect.x +
                            self.matrice_passage_inverse[4] * offset_intersect.y +
                            self.matrice_passage_inverse[5] * offset_intersect.z,
                        z: self.matrice_passage_inverse[6] * offset_intersect.x +
                            self.matrice_passage_inverse[7] * offset_intersect.y +
                            self.matrice_passage_inverse[8] * offset_intersect.z,
                    },
                });
            }
        }

        None
    }

    fn extremums(&self) -> (Vec3, Vec3) {
        (self.mincoord, self.maxcoord)
    }
}