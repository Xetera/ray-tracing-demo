use rayon::iter::IntoParallelIterator;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};

use crate::{
    camera::Camera,
    log,
    ray::{Hittable, Ray, Shape},
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

#[derive(Serialize, Deserialize)]
pub struct Canvas {
    width: usize,
    height: usize,
    aspect_ratio: f32,
    shapes: Vec<Shape>,
    camera: Camera,
}

impl Canvas {
    pub fn new(camera: Camera, width: usize, aspect_ratio: f32, shapes: Vec<Shape>) -> Self {
        let height = (width as f32 / aspect_ratio).floor() as usize;

        Self {
            camera,
            width,
            aspect_ratio,
            height,
            shapes,
        }
    }

    fn pixels(&self) -> usize {
        self.width * self.height
    }

    pub fn paint(&self) -> Vec<u8> {
        // let mut pixels: Vec<Vec3> = vec![];
        let pixels = (0..self.height)
            .flat_map(|j| (0..self.width).map(move |i| (i, j)))
            .rev()
            .collect::<Vec<(usize, usize)>>()
            .into_iter()
            .map(|(i, j)| {
                let u = i as f32 / (self.width - 1) as f32;
                let v = j as f32 / (self.height - 1) as f32;

                let ray = self.camera.beam(u, v);

                let color = self.color_at(&ray);
                color
                // pixels.push(color);
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
        let mut collisions = self.shapes.iter().find_map(|shape| shape.cast(ray).hit());

        // only concerned with the first collision for now
        match collisions {
            Some(hit) => 0.5 * (hit.normal + Color3::new(1.0, 1.0, 1.0)),
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
