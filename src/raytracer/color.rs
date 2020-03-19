use std::ops::{Add, AddAssign, Div, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub const BLACK: Color = Color::from_rgb(0.0, 0.0, 0.0);
    pub const RED: Color = Color::from_rgb(1.0, 0.0, 0.0);
    pub const GREEN: Color = Color::from_rgb(0.0, 1.0, 0.0);
    pub const WHITE: Color = Color::from_rgb(1.0, 1.0, 1.0);
    pub const YELLOW: Color = Color::from_rgb(1.0, 1.0, 0.0);
    pub const ORANGE: Color = Color::from_rgb(1.0, 0.5, 0.0);
    pub const PURPLE: Color = Color::from_rgb(1.0, 0.0, 1.0);
    pub const BLUE: Color = Color::from_rgb(0.0, 0.0, 1.0);

    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color { r, g, b }
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            (self.r * 255.0).min(255.0) as u8,
            (self.g * 255.0).min(255.0) as u8,
            (self.b * 255.0).min(255.0) as u8,
        )
    }

    pub fn to_pixel(&self) -> u32 {
        ((self.r * 255.0).min(255.0) as u32) << 16
            | ((self.g * 255.0).min(255.0) as u32) << 8
            | ((self.b * 255.0).min(255.0) as u32)
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Div<f32> for Color {
    type Output = Color;

    fn div(self, rhs: f32) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl Mul<f32> for Color {
    type Output = Color;

    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self * rhs.r,
            g: self * rhs.g,
            b: self * rhs.b,
        }
    }
}
