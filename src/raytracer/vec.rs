use std::ops::{Add, Sub, Mul, Neg, Div};
use std::cmp::min;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn min(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {x: a.x.min(a.y), y: a.y.min(b.y), z: a.z.min(b.z)}
    }

    pub fn max(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 {x: a.x.max(a.y), y: a.y.max(b.y), z: a.z.max(b.z)}
    }

    pub fn zero() -> Vec3 { Vec3 { x: 0.0, y: 0.0, z: 0.0 } }

    pub fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }

    pub fn squared_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&mut self) {
        let length = self.length();
        self.x /= length;
        self.y /= length;
        self.z /= length;
    }

    pub fn normalized(&self) -> Vec3 {
        let length = self.length();
        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }

    pub fn dot(left: &Vec3, right: &Vec3) -> f32 {
        left.x * right.x + left.y * right.y + left.z * right.z
    }

    pub fn cross_product(left: &Vec3, right: &Vec3) -> Vec3 {
        Vec3 {
            x: left.y * right.z - left.z * right.y,
            y: left.z * right.x - left.x * right.z,
            z: left.x * right.y - left.y * right.x,
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vec3> for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<&Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: Self) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add<Vec3> for &Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        return self.x == other.x && self.y == other.y && self.z == other.z;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::raytracer::vec::Vec3;

    #[test]
    fn mul_scalar_f32() {
        assert_eq!(
            Vec3 { x: 1f32, y: 1f32, z: 1f32 } * 2f32,
            Vec3 { x: 2f32, y: 2f32, z: 2f32 },
        )
    }

    #[test]
    fn mul_f32_scalar() {
        assert_eq!(
            2f32 * Vec3 { x: 1f32, y: 1f32, z: 1f32 },
            Vec3 { x: 2f32, y: 2f32, z: 2f32 },
        )
    }

    #[test]
    fn eq() {
        assert_eq!(
            Vec3 { x: 0.0, y: 0.0, z: 0.0 },
            Vec3 { x: 0.0, y: 0.0, z: 0.0 }
        )
    }

    #[test]
    fn add() {
        assert_eq!(
            Vec3 { x: 1f32, y: 1f32, z: 1f32 },
            Vec3 { x: 1f32, y: 2f32, z: 3f32 } + Vec3 { x: 0f32, y: -1f32, z: -2f32 });
    }


    #[test]
    fn sub() {
        assert_eq!(
            Vec3 { x: 1f32, y: 1f32, z: 1f32 },
            Vec3 { x: 1f32, y: 2f32, z: 3f32 } - &Vec3 { x: 0f32, y: 1f32, z: 2f32 });
    }

    #[test]
    fn squared_lenght() {
        assert_eq!((Vec3 { x: 1f32, y: 1f32, z: 1f32 }).squared_length(), 3f32);
    }

    #[test]
    fn lenght() {
        assert_eq!((Vec3 { x: 1f32, y: 0f32, z: 0f32 }).length(), 1f32);
        assert_eq!((Vec3 { x: 2f32, y: 0f32, z: 0f32 }).length(), 2f32);
        assert_eq!((Vec3 { x: 0f32, y: 1f32, z: 0f32 }).length(), 1f32);
        assert_eq!((Vec3 { x: 0f32, y: 0f32, z: 1f32 }).length(), 1f32);
    }

    #[test]
    fn normalize() {
        let vec1 = Vec3 { x: 1f32, y: 0f32, z: 0f32 };
        let mut vec1b = Vec3 { x: 2f32, y: 0f32, z: 0f32 };
        let vec2 = Vec3 { x: 1f32, y: 0f32, z: 0f32 };
        let mut vec2b = Vec3 { x: 2f32, y: 0f32, z: 0f32 };
        let vec3 = Vec3 { x: 1f32, y: 0f32, z: 0f32 };
        let mut vec3b = Vec3 { x: 2f32, y: 0f32, z: 0f32 };
        vec1b.normalize();
        vec2b.normalize();
        vec3b.normalize();
        assert_eq!(
            vec1,
            vec1b
        );
        assert_eq!(
            vec2,
            vec2b
        );
        assert_eq!(
            vec3,
            vec3b
        );
    }
}