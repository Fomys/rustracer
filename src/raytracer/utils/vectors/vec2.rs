use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T: Add<Output = T>> Add for Vec2<T>
where
    T: Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Sub<Output = T>> Sub for Vec2<T>
where
    T: Sub<T, Output = T>,
{
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
