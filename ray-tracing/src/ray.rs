use serde::{Deserialize, Serialize};

use crate::{
    log,
    vec::{Color3, Point3, Vec3},
};

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + self.direction * t
    }
}

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
}

/**
 * Casted rays can have any number of collisions with the material
 * but it must at least have 1 entry for it to be qualified as a hit
 */
pub struct CastHit {
    pub shape: Shape,
    pub entry: Vec3,
    pub collisions: Vec<Vec3>,
    pub normal: Vec3,
    pub time: f32,
}

pub enum CastResult {
    Miss,
    Hit(CastHit),
}

impl CastResult {
    pub fn hit_count(&self) -> usize {
        match self {
            CastResult::Miss => 0,
            // a hit is guaranteed 1 hit + the next collisions
            CastResult::Hit(CastHit { collisions, .. }) => 1 + collisions.len(),
        }
    }

    pub fn hit(self) -> Option<CastHit> {
        match self {
            CastResult::Hit(hit) => Some(hit),
            CastResult::Miss => None,
        }
    }
}

pub trait Hittable {
    fn cast(&self, ray: &Ray) -> CastResult;
}

impl Hittable for Shape {
    fn cast(&self, ray: &Ray) -> CastResult {
        match self {
            Shape::Sphere { center, radius, .. } => {
                let oc = ray.origin - *center;
                let a = ray.direction.length_squared();
                let half_b = oc.dot(ray.direction);
                let c = oc.length_squared() - radius * radius;
                let discriminant = half_b * half_b - a * c;

                if discriminant < 0.0 {
                    return CastResult::Miss;
                }

                let sqrtd = f32::sqrt(discriminant);

                let root = (-half_b - sqrtd) / a;
                let second_root = (-half_b + sqrtd) / a;

                let t_min = 0.0;
                let t_max = 3.0;

                let out_of_bounds = |root: f32| root < t_min || t_max < root;

                let is_root_oob = out_of_bounds(root);

                if is_root_oob && out_of_bounds(second_root) {
                    return CastResult::Miss;
                }

                let t = if is_root_oob { second_root } else { root };

                let p = ray.at(t);
                let outward_normal = (p - *center) / *radius;

                let is_front_face = ray.direction.dot(outward_normal) < 0.0;

                let normal = if is_front_face {
                    outward_normal
                } else {
                    -outward_normal
                };

                CastResult::Hit(CastHit {
                    // TODO: Find out wtf is going on here
                    entry: p,
                    normal,
                    time: t,
                    collisions: vec![],
                    shape: *self,
                })
            }
        }
    }
}

impl ToString for Ray {
    fn to_string(&self) -> String {
        format!(
            "origin: {}\ndirection: {}",
            self.origin.to_string(),
            self.direction.to_string()
        )
    }
}
