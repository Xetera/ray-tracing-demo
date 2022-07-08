use serde::{Deserialize, Serialize};

use crate::{
    ray::Ray,
    vec::{Color3, Point3, Vec3},
};

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Shape {
    Sphere {
        center: Point3,
        radius: f32,
        color: Color3,
    },
}

impl Shape {
    pub fn color(&self, ray: &Ray) -> Color3 {
        match self {
            Shape::Sphere { color, center, .. } => {
                let coeff = 1.0 - ((ray.origin + ray.direction) - *center).length();
                *color * coeff
            }
        }
    }

    pub fn distance_squared(&self, origin: &Vec3) -> f32 {
        match self {
            Shape::Sphere { center, .. } => (*center - *origin).map(|x| x.powi(2)).sum(),
        }
    }
}
