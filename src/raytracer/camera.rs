use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::raytracer::color::{Color, BLACK};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::TILE_SIZE;
use crate::raytracer::utils::{Vec2, Vec3};

pub struct Tile {
    // Taille du buffer
    pub(crate) size: Vec2<usize>,
    // Position sur le buffer
    upper_left_corner: Vec2<usize>,
    // Buffer interne du tile
    pub(crate) buffer: Vec<Color>,
    // Liste des rayons pour itérer
    pub(crate) rays: Vec<Ray>,
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

    fn preprocess(&mut self, camera: &Camera) {
        for j in 0..self.size.y {
            for i in 0..self.size.x {
                for sub_i in 0..camera.ray_per_pixels {
                    for sub_j in 0..camera.ray_per_pixels {
                        self.rays.push(camera.get_ray(Vec2 {
                            x: i as f32
                                + self.upper_left_corner.x as f32
                                + sub_i as f32 / camera.ray_per_pixels as f32,
                            y: j as f32
                                + self.upper_left_corner.y as f32
                                + sub_j as f32 / camera.ray_per_pixels as f32,
                        }));
                    }
                }
            }
        }
    }
}

pub struct Camera {
    current_frame: usize,
    file: String,
    ray_per_pixels: usize,
    pub ray_per_pixels_count: usize,
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
        position: Vec3, direction: Vec3, fov: Vec2<f32>, up: Vec3, size: Vec2<usize>, file: String,
        ray_per_pixels: usize,
    ) -> Camera {
        let vertical_vector = fov.y * (up - (direction | up) * direction.normalized()).normalized();
        let horizontal_vector = fov.x * (vertical_vector ^ direction).normalized();
        let lower_left_corner =
            position + direction - 0.5 * horizontal_vector - 0.5 * vertical_vector;

        // Pregenerate tiles
        let tile_count = (size / TILE_SIZE) + Vec2 { x: 1, y: 1 };

        Camera {
            current_frame: 0,
            position,
            size,
            buffer: vec![BLACK; size.y * size.x],
            lower_left_corner,
            horizontal_vector,
            vertical_vector,
            current_tile: 0,
            tile_count,
            file,
            ray_per_pixels,
            ray_per_pixels_count: ray_per_pixels * ray_per_pixels,
        }
    }

    pub fn get_ray(&self, position: Vec2<f32>) -> Ray {
        Ray {
            origin: self.position,
            direction: (self.lower_left_corner
                + (position.x / self.size.x as f32) * self.horizontal_vector
                + (position.y / self.size.y as f32) * self.vertical_vector)
                - self.position,
        }
    }

    // NEXT: Remplacer Color par un spectre
    pub fn merge_tile(&mut self, tile: &Tile) {
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

    pub fn next_frame(&mut self) {
        self.current_frame += 1;
        self.current_tile = 0;
    }

    pub fn save(&self) {
        let mut str_file = self.current_frame.to_string();
        str_file.push_str(self.file.as_str());
        let path = Path::new(str_file.as_str());
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
