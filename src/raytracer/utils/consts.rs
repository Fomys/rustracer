use crate::raytracer::utils::vec::Vec2;

pub const ZERO: f32 = 0.0001;
pub const TILE_SIZE: Vec2<usize> = Vec2 { x: 16, y: 16 };
pub const MAX_ITERATION: usize = 5;