use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    ray::{Ray, Rotation3D},
    vec::{Point3, Vec3},
};

#[wasm_bindgen]
pub enum RelativeDirection {
    Up,
    Down,
    Left,
    Right,
}

impl RelativeDirection {
    fn to_movement_vector(&self, speed: f32) -> Vec3 {
        match &self {
            RelativeDirection::Up => Vec3::new(0.0, 0.0, -speed),
            RelativeDirection::Down => Vec3::new(0.0, 0.0, speed),
            RelativeDirection::Left => Vec3::new(speed, 0.0, 0.0),
            RelativeDirection::Right => Vec3::new(-speed, 0.0, 0.0),
        }
    }

    fn to_rotation_vector(&self, speed: f32, rotation: Vec3) -> Vec3 {
        rotation
            * match &self {
                RelativeDirection::Up => Vec3::new(speed, 0.0, 0.0),
                RelativeDirection::Down => Vec3::new(-speed, 0.0, 0.0),
                RelativeDirection::Left => Vec3::new(0.0, -speed, 0.0),
                RelativeDirection::Right => Vec3::new(0.0, speed, 0.0),
            }
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub speed: f32,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub rotation: Rotation3D,
    focal_length: f32,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        viewport_height: i32,
        focal_length: f32,
        origin: Vec3,
        rotation_degrees: Vec3,
    ) -> Self {
        let rotation = Rotation3D::new(rotation_degrees);
        let viewport_width = aspect_ratio * viewport_height as f32;
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height as f32, 0.0);

        Self {
            origin,
            focal_length,
            horizontal,
            rotation,
            vertical,
            speed: 0.1,
        }
    }

    pub fn move_along(&mut self, movement: RelativeDirection) {
        self.origin = self.origin
            + self
                .rotation
                .rotate(&movement.to_movement_vector(self.speed));
    }

    pub fn turn(&mut self, rotation: Vec3) {
        self.rotation.turn(rotation)
    }

    pub fn beam(&self, u: f32, v: f32) -> Ray {
        let lower_left_corner = self.origin
            - self.horizontal / 2.0
            - self.vertical / 2.0
            - Vec3::new(0.0, 0.0, self.focal_length);
        let raw_direction =
            lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        let direction = self.rotation.rotate(&raw_direction);

        Ray {
            origin: self.origin,
            direction,
        }
    }
}
