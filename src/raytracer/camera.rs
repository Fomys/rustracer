use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::raytracer::color::{Color, BLACK, RED};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::consts::{RAY_PER_PIXELS, TILE_SIZE};
use crate::raytracer::utils::vec::{Vec2, Vec3};
use minifb::{Window, WindowOptions};

pub struct Tile {
    // Taille du buffer
    pub(crate) size: Vec2<usize>,
    // Position sur le buffer
    upper_left_corner: Vec2<usize>,
    // Buffer interne du tile
    pub(crate) buffer: Vec<Color>,
    // Liste des rayons pour itérer
    pub(crate) rays: Vec<Vec<Ray>>,
}

impl Tile {
    pub fn free_mem(&mut self) {
        self.rays = vec![];
    }

    fn new(size: Vec2<usize>, upper_left_corner: Vec2<usize>) -> Tile {
        Tile {
            size,
            upper_left_corner,
            buffer: vec![BLACK; size.x * size.y],
            rays: vec![],
        }
    }

    pub fn set_color(&mut self, index: usize, color: Color) {
        self.buffer[index] = color;
    }

    fn preprocess(&mut self, camera: &Camera) {
        for j in 0..self.size.y {
            for i in 0..self.size.x {
                let mut temp_rays: Vec<Ray> = vec![];
                for sub_i in 0..RAY_PER_PIXELS {
                    for sub_j in 0..RAY_PER_PIXELS {
                        temp_rays.push(camera.get_ray(Vec2 {
                            x: i as f32
                                + self.upper_left_corner.x as f32
                                + sub_i as f32 / RAY_PER_PIXELS as f32,
                            y: j as f32
                                + self.upper_left_corner.y as f32
                                + sub_j as f32 / RAY_PER_PIXELS as f32,
                        }));
                    }
                }
                self.rays.push(temp_rays);
            }
        }
    }
}

pub struct Camera {
    // Taille du buffer
    pub size: Vec2<usize>,
    // Position de la caméra
    pub position: Vec3,
    // NEXT: Remplacer Color par un spectre
    // Buffer
    buffer: Vec<Color>,
    // Vecteur générateur dans l'écran de la scène
    vertical_vector: Vec3,
    horizontal_vector: Vec3,
    // Point bas gauche
    lower_left_corner: Vec3,

    // For iterator
    current_tile: usize,
    pub tile_count: Vec2<usize>,
}

impl Camera {
    pub fn new(
        position: Vec3, direction: Vec3, fov: Vec2<f32>, up: Vec3, size: Vec2<usize>,
    ) -> Camera {
        let vertical_vector =
            (up - direction.normalized() * Vec3::dot(&direction, &up)).normalized() * fov.y;
        let horizontal_vector =
            Vec3::cross_product(&vertical_vector, &direction).normalized() * fov.x;
        let lower_left_corner =
            position + direction - 0.5 * horizontal_vector - 0.5 * vertical_vector;

        // Pregenerate tiles
        let tile_count = (size / TILE_SIZE) + Vec2 { x: 1, y: 1 };

        Camera {
            position,
            size,
            buffer: vec![BLACK; size.y * size.x],
            lower_left_corner,
            horizontal_vector,
            vertical_vector,
            current_tile: 0,
            tile_count,
        }
    }

    pub fn get_ray(&self, position: Vec2<f32>) -> Ray {
        Ray {
            origin: self.position,
            direction: (self.lower_left_corner
                + self.horizontal_vector * position.x / self.size.x as f32
                + self.vertical_vector * position.y / self.size.y as f32)
                - self.position,
        }
    }

    // NEXT: Remplacer Color par un spectre
    pub fn merge_tile(&mut self, tile: &Tile) {
        let mut index = 0;
        let mut x = 0;
        let mut y = 0;
        let start = tile.upper_left_corner.x + tile.upper_left_corner.y * self.size.x;
        for color in tile.buffer.iter() {
            self.buffer[start + x + y * self.size.x] = *color;
            x += 1;
            if x >= tile.size.x {
                x = 0;
                y += 1;
            }
        }
    }

    pub fn reset_tiles(&mut self) {
        self.current_tile = 0;
    }

    pub fn next_tile(&mut self) -> Option<Tile> {
        // Check if all tiles has been passed
        if self.current_tile > self.tile_count.x * self.tile_count.y - 1 {
            return None;
        }
        let current = Vec2 {
            x: self.current_tile % self.tile_count.x,
            y: self.current_tile / self.tile_count.x,
        };
        let tile_size = Vec2 {
            x: TILE_SIZE.x.min(self.size.x - TILE_SIZE.x * current.x),
            y: TILE_SIZE.y.min(self.size.y - TILE_SIZE.y * current.y),
        };

        let mut new_tile = Tile::new(
            tile_size,
            Vec2 {
                x: current.x * TILE_SIZE.x,
                y: self.size.y.min(current.y * TILE_SIZE.y),
            },
        );

        new_tile.preprocess(&self);

        self.current_tile += 1;
        Some(new_tile)
    }

    pub fn save(&self) {
        let path = Path::new("save.png");
        let file = File::create(path).unwrap();
        let w = &mut BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, self.size.x as u32, self.size.y as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::RGB);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();
        let mut png_data = vec![0u8; 0];
        png_data.reserve_exact((self.size.x * self.size.y * 3) as usize);

        for color in self.buffer.iter() {
            let (r, g, b) = color.to_rgb();
            png_data.push(r);
            png_data.push(g);
            png_data.push(b);
        }
        writer.write_image_data(&png_data).unwrap(); // Save
    }
}
