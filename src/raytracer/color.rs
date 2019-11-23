use std::ops::{Add, Div, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}


pub struct ColorInfo {
    pub color: Color,
    pub ratio: f32,
}

impl Color {
    pub fn black() -> Color {
        Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
        }
    }
    pub fn to_rgb(&self) -> (u8, u8, u8) {
        ((self.r * 255.0).min(255.0) as u8,
         (self.g * 255.0).min(255.0) as u8,
         (self.b * 255.0).min(255.0) as u8)
    }

    pub fn to_pixel(&self) -> u32 {
        ((self.r * 255.0).min(255.0) as u32) << 16 |
            ((self.g * 255.0).min(255.0) as u32) << 8 |
            ((self.b * 255.0).min(255.0) as u32)
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Self) -> Color {
        Color {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, other: f32) -> Color {
        Color {
            r: self.r / other,
            g: self.g / other,
            b: self.b / other,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, other: f32) -> Color {
        Color {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {
            r: other.r * self,
            g: other.g * self,
            b: other.b * self,
        }
    }
}