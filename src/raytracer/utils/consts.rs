use crate::raytracer::utils::Vec2;

pub const ZERO: f32 = 0.0001;
pub const OFFSET: f32 = 0.01;
pub const TILE_SIZE: Vec2<usize> = Vec2 { x: 16, y: 16 };
pub const RAY_PER_REFLECT: usize = 1;
pub const SOURCE_PER_SURFACE: usize = 1;
pub const FAR_FAR_AWAY: f32 = 1_000.0;
