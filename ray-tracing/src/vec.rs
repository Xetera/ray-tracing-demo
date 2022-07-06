use std::ops;

use serde::{Deserialize, Serialize};

use crate::canvas::{Drawable, Pixel};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Vec3 {
    data: [f32; 3],
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { data: [x, y, z] }
    }

    pub fn from_array(arr: [f32; 3]) -> Self {
        Self { data: arr }
    }

    pub fn null() -> Self {
        Self {
            data: [0f32, 0f32, 0f32],
        }
    }

    pub fn x(&self) -> f32 {
        self.data[0]
    }

    pub fn y(&self) -> f32 {
        self.data[1]
    }

    pub fn z(&self) -> f32 {
        self.data[2]
    }

    #[inline]
    pub fn dot(&self, vec: Vec3) -> f32 {
        self.data.iter().zip(vec.data).map(|(a, b)| a * b).sum()
    }

    #[inline]
    pub fn cross(&self, vec: Vec3) -> Self {
        Self::new(
            self.data[1] * vec.data[2] - self.data[2] * vec.data[1],
            self.data[2] * vec.data[0] - self.data[0] * vec.data[0],
            self.data[0] * vec.data[1] - self.data[1] * vec.data[0],
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }

    pub fn length_squared(&self) -> f32 {
        self.data.iter().map(|a| a * a).sum()
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.length_squared())
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.data[0] + rhs.data[0],
            self.data[1] + rhs.data[1],
            self.data[2] + rhs.data[2],
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.data[0] - rhs.data[0],
            self.data[1] - rhs.data[1],
            self.data[2] - rhs.data[2],
        )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(
            self.data[0] * rhs.data[0],
            self.data[1] * rhs.data[1],
            self.data[2] * rhs.data[2],
        )
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::from_array(self.data.map(|a| a * rhs))
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        self * (1f32 / rhs)
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            data: self.data.map(|i| -i),
        }
    }
}

impl Drawable for Vec3 {
    fn pixels(&self) -> crate::canvas::Pixel {
        Pixel {
            r: u8::try_from((self.x() * 0xff as f32) as i32).expect("Number is not between 0-255"),
            g: u8::try_from((self.y() * 0xff as f32) as i32).expect("Number is not between 0-255"),
            b: u8::try_from((self.z() * 0xff as f32) as i32).expect("Number is not between 0-255"),
            a: u8::max_value(),
        }
    }
}

impl ToString for Vec3 {
    fn to_string(&self) -> String {
        format!("[{}, {}, {}]", self.x(), self.y(), self.z())
    }
}

pub type Point3 = Vec3;
pub type Color3 = Vec3;
