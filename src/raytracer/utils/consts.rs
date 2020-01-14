use crate::raytracer::utils::vec::Vec2;

pub const ZERO: f32 = 0.0001;
pub const TILE_SIZE: Vec2<usize> = Vec2 { x: 16, y: 16 };
pub const MAX_ITERATION: usize = 3;
pub const RAY_PER_PIXELS: usize = 3;
// Launch 10*10 rays per pixels
pub const RAY_PER_REFLECT: usize = 2;
pub const SOURCE_PER_SURFACE: usize = 5;
