use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    log,
    shape::Shape,
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

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Theta(f32);

impl Theta {
    fn from_radians(radians: f32) -> Option<Self> {
        if radians < 0.0 || radians > std::f32::consts::TAU {
            None
        } else {
            Some(Self(radians))
        }
    }
    fn cos(&self) -> f32 {
        self.0.cos()
    }

    fn sin(&self) -> f32 {
        self.0.sin()
    }
}

pub trait RotationTransformer {
    fn transform(&self, theta: Theta, direction: &Vec3) -> Vec3;
}

#[derive(Copy, Clone)]
pub struct Rotation3D {
    rotation: Vec3,
    transformers: [AxisRotator; 3],
}

impl Rotation3D {
    pub fn new(rotation: Vec3) -> Self {
        let transformers = [ROTATION_X, ROTATION_Y, ROTATION_Z];

        Self {
            rotation,
            transformers,
        }
    }

    pub fn rotate(&self, direction: &Vec3) -> Vec3 {
        self.transformers
            .iter()
            .zip(self.rotation.parts())
            .fold(*direction, |dir, (transformer, theta)| {
                transformer.transform(Theta(theta), &dir)
            })
    }

    pub fn turn(&mut self, rotation: Vec3) {
        self.rotation = rotation
    }
}

#[derive(Copy, Clone)]
pub struct AxisRotator(fn(f32) -> [Vec3; 3]);

impl RotationTransformer for AxisRotator {
    fn transform(&self, theta: Theta, direction: &Vec3) -> Vec3 {
        let vecs = self.0(theta.0);
        Vec3::from_array(vecs.map(|vec| direction.dot(vec)))
    }
}

pub const ROTATION_X: AxisRotator = AxisRotator(|theta| {
    [
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, theta.cos(), -theta.sin()),
        Vec3::new(0.0, theta.sin(), theta.cos()),
    ]
});

pub const ROTATION_Y: AxisRotator = AxisRotator(|theta| {
    [
        Vec3::new(theta.cos(), 0.0, theta.sin()),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(-theta.sin(), 0.0, theta.cos()),
    ]
});

pub const ROTATION_Z: AxisRotator = AxisRotator(|theta| {
    [
        Vec3::new(theta.cos(), -theta.sin(), 0.0),
        Vec3::new(theta.sin(), theta.cos(), 0.0),
        Vec3::new(0.0, 0.0, 1.0),
    ]
});
