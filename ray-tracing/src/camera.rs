use serde::{Deserialize, Serialize};

use crate::{
    ray::Ray,
    vec::{Point3, Vec3},
};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Camera {
    pub origin: Point3,
    lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f32, viewport_height: i32, focal_length: f32, origin: Vec3) -> Self {
        let viewport_width = aspect_ratio * viewport_height as f32;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height as f32, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);
        Self {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn beam(&self, u: f32, v: f32) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }
}
