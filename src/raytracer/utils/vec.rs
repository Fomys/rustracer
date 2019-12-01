use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn min(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 { x: a.x.min(a.y), y: a.y.min(b.y), z: a.z.min(b.z) }
    }

    pub fn max(a: Vec3, b: Vec3) -> Vec3 {
        Vec3 { x: a.x.max(a.y), y: a.y.max(b.y), z: a.z.max(b.z) }
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

impl<T: Add<Output=T>> Add for Vec2<T>
    where T: Add<T, Output=T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output=T>> Sub for Vec2<T>
    where T: Sub<T, Output=T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<Vec2<f32>> for f32 {
    type Output = Vec2<f32>;

    fn mul(self, other: Vec2<f32>) -> Vec2<f32> {
        Vec2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl Mul<Vec2<usize>> for usize {
    type Output = Vec2<usize>;

    fn mul(self, other: Vec2<usize>) -> Vec2<usize> {
        Vec2 {
            x: self * other.x,
            y: self * other.y,
        }
    }
}

impl Div<usize> for Vec2<usize> {
    type Output = Vec2<usize>;

    fn div(self, other: usize) -> Vec2<usize> {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl Div<Vec2<usize>> for Vec2<usize> {
    type Output = Vec2<usize>;

    fn div(self, other: Vec2<usize>) -> Vec2<usize> {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl From<Vec2<f32>> for Vec2<usize> {
    fn from(item: Vec2<f32>) -> Self {
        Vec2 {
            x: item.x as usize,
            y: item.y as usize,
        }
    }
}

impl From<Vec2<usize>> for Vec2<f32> {
    fn from(item: Vec2<usize>) -> Self {
        Vec2 {
            x: item.x as f32,
            y: item.y as f32,
        }
    }
}

impl Mul<Vec2<usize>> for f32 {
    type Output = Vec2<f32>;

    fn mul(self, other: Vec2<usize>) -> Vec2<f32> {
        Vec2 {
            x: self * other.x as f32,
            y: self * other.y as f32,
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


