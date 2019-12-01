use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use crate::raytracer::color::{Color, BLACK, RED};
use crate::raytracer::ray::Ray;
use crate::raytracer::utils::consts::TILE_SIZE;
use crate::raytracer::utils::vec::{Vec2, Vec3};
use minifb::{Window, WindowOptions};

pub struct Tile {
    // Taille du buffer
    pub(crate) size: Vec2<usize>,
    // Position sur le buffer
    upper_left_corner: Vec2<usize>,
    // Buffer interne du tile
    pub buffer: Vec<Color>,
    // Liste des rayons pour itérer
    pub rays: Vec<Ray>,
}

impl Tile {
    fn new(size: Vec2<usize>, upper_left_corner: Vec2<usize>) -> Tile {
        let mut rays: Vec<Ray> = vec![];
        let mut buffer: Vec<Color> = vec![RED; size.x * size.y];
        rays.reserve_exact(size.x * size.y);
        Tile {
            size,
            upper_left_corner,
            buffer,
            rays,
        }
    }

    pub fn set_color(&mut self, index: usize, color: Color) {
        self.buffer[index] = color;
    }

    fn preprocess(
        &mut self,
        origin: Vec3,
        horizontal_vector: Vec3,
        vertical_vector: Vec3,
        lower_left_corner: Vec3,
        size: Vec2<usize>,
    ) {
        for j in 0..self.size.y {
            for i in 0..self.size.x {
                //println!(" i + self.lower_left_corner.x - self.size.x {} - {}",  i + self.lower_left_corner.x, self.size.x);
                self.rays.push(Camera::raw_get_ray(
                    origin,
                    horizontal_vector,
                    vertical_vector,
                    lower_left_corner,
                    Vec2 {
                        x: i + self.upper_left_corner.x,
                        y: j + self.upper_left_corner.y,
                    },
                    size,
                ));
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

    // TODO: Ya pas plus propre?
    // Tiles
    pub tiles: Vec<Tile>,
}

impl Camera {
    pub fn new(
        position: Vec3,
        direction: Vec3,
        fov: Vec2<f32>,
        up: Vec3,
        size: Vec2<usize>,
    ) -> Camera {
        let vertical_vector =
            (up - direction.normalized() * Vec3::dot(&direction, &up)).normalized() * fov.y;
        let horizontal_vector =
            Vec3::cross_product(&vertical_vector, &direction).normalized() * fov.x;
        let lower_left_corner =
            position + direction - 0.5 * horizontal_vector - 0.5 * vertical_vector;

        // Pregenerate tiles
        let mut tiles = vec![];
        let tile_count = (size / TILE_SIZE) + Vec2 { x: 1, y: 1 };

        for i_tile in 0..tile_count.x {
            for j_tile in 0..tile_count.y {
                let tile_size = Vec2 {
                    x: TILE_SIZE.x.min(size.x - TILE_SIZE.x * i_tile),
                    y: TILE_SIZE.y.min(size.y - TILE_SIZE.y * j_tile),
                };
                let mut new_tile = Tile::new(
                    tile_size,
                    Vec2 {
                        x: i_tile * TILE_SIZE.x,
                        y: size.y.min(j_tile * TILE_SIZE.y),
                    },
                );
                new_tile.preprocess(
                    position,
                    horizontal_vector,
                    vertical_vector,
                    lower_left_corner,
                    size,
                );
                tiles.push(new_tile);
            }
        }

        Camera {
            position,
            size,
            buffer: vec![BLACK; size.y * size.x],
            lower_left_corner,
            horizontal_vector,
            vertical_vector,
            tiles,
        }
    }

    pub fn raw_get_ray(
        origin: Vec3,
        horizontal_vector: Vec3,
        vertical_vector: Vec3,
        lower_left_corner: Vec3,
        position: Vec2<usize>,
        size: Vec2<usize>,
    ) -> Ray {
        Ray {
            origin,
            direction: (lower_left_corner
                + horizontal_vector * position.x as f32 / size.x as f32
                + vertical_vector * position.y as f32 / size.y as f32)
                - origin,
        }
    }

    // NEXT: Remplacer Color par un spectre
    pub fn merge_tile(&mut self, tile_index: usize) {
        let mut index = 0;
        let mut x = 0;
        let mut y = 0;
        let start = self.tiles[tile_index].upper_left_corner.x
            + self.tiles[tile_index].upper_left_corner.y * self.size.x;
        for color in self.tiles[tile_index].buffer.iter() {
            self.buffer[start + x + y * self.size.x] = *color;
            x += 1;
            if x >= self.tiles[tile_index].size.x {
                x = 0;
                y += 1;
            }
        }
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
