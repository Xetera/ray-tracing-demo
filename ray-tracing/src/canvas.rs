use serde::{Deserialize, Serialize};

use crate::shape::Shape;
use crate::{
    camera::Camera,
    ray::{Hittable, Ray},
    vec::{Color3, Vec3},
};
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[derive(Serialize, Deserialize)]
pub struct ImageData {
    pub data: Vec<u8>,
    pub height: i32,
    pub width: i32,
}

// #[derive(Serialize, Deserialize)]
pub struct Canvas {
    width: usize,
    height: usize,
    aspect_ratio: f32,
    shapes: Vec<Shape>,
    pixels: Vec<(usize, usize)>,
}

impl Canvas {
    pub fn new(width: usize, aspect_ratio: f32, shapes: Vec<Shape>) -> Self {
        let (width, height) = Canvas::dimensions(width, aspect_ratio);

        Self {
            width,
            aspect_ratio,
            height,
            shapes,
            pixels: Canvas::pixel_vec(width, height),
        }
    }

    pub fn dimensions(width: usize, aspect_ratio: f32) -> (usize, usize) {
        let height = (width as f32 / aspect_ratio).floor() as usize;
        (width, height)
    }

    pub fn resize(&mut self, width: usize) {
        (self.width, self.height) = Canvas::dimensions(width, self.aspect_ratio);
        self.pixels = Canvas::pixel_vec(self.width, self.height);
    }

    pub fn pixel_vec(width: usize, height: usize) -> Vec<(usize, usize)> {
        (0..height)
            .flat_map(|j| (0..width).map(move |i| (i, j)))
            .rev()
            .collect::<Vec<(usize, usize)>>()
    }

    pub fn paint(&self, camera: &Camera) -> Vec<u8> {
        let pixels = self
            .pixels
            .iter()
            .map(|(i, j)| {
                let u = *i as f32 / (self.width - 1) as f32;
                let v = *j as f32 / (self.height - 1) as f32;

                let ray = camera.beam(u, v);

                self.color_at(&ray)
            })
            .collect::<Vec<Vec3>>();

        Canvas::to_pixel_data(pixels)
    }

    pub fn to_pixel_data<T: Drawable>(drawables: Vec<T>) -> Vec<u8> {
        let mut data = Vec::<u8>::with_capacity(drawables.len() * 4);

        for drawable in drawables {
            let pixel = drawable.pixels();
            data.append(&mut vec![pixel.r, pixel.g, pixel.b, pixel.a]);
        }

        data
    }

    pub fn color_at(&self, ray: &Ray) -> Color3 {
        let first_collision = self
            .shapes
            .iter()
            .filter_map(|shape| shape.cast(ray).hit())
            .min_by(|a, b| {
                a.time
                    .partial_cmp(&b.time)
                    .unwrap_or(std::cmp::Ordering::Less)
            });

        match first_collision {
            Some(hit) => (hit.normal + Color3::new(1.0, 1.0, 1.0)) * 0.5,
            None => self.background_color(ray),
        }
    }

    fn background_color(&self, ray: &Ray) -> Color3 {
        let unit_direction = ray.direction.unit_vector();
        let one = Color3::new(1.0, 1.0, 1.0);
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * one + (t * Color3::new(0.5, 0.7, 1.0))
    }
}

pub trait Drawable {
    fn pixels(&self) -> Pixel;
}
