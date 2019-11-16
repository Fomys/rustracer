#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
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
