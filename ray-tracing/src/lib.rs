mod camera;
mod canvas;
mod ray;
mod vec;

use std::panic;

use camera::Camera;
use canvas::Canvas;
use ray::Shape;
use vec::{Point3, Vec3};
use wasm_bindgen::prelude::*;
pub use wasm_bindgen_rayon::init_thread_pool;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn paint(
    width: i32,
    viewport_height: i32,
    aspect_ratio: f32,
    focal_length: f32,
    origin: Vec<f32>,
) -> JsValue {
    console_error_panic_hook::set_once();

    let shapes = vec![
        Shape::Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
            color: Vec3::new(0.3, 1.0, 0.3),
        },
        Shape::Sphere {
            center: Vec3::new(0.2, 0.0, -1.0),
            radius: 0.6,
            color: Vec3::new(0.8, 0.0, 0.3),
        },
        Shape::Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
            color: Vec3::null(),
        },
    ];

    if let [x, y, z] = origin[..] {
        let origin = Point3::new(x, y, z);
        let camera = Camera::new(aspect_ratio, viewport_height, focal_length, origin);

        let canvas = Canvas::new(camera, width as usize, aspect_ratio, shapes);
        let pixels = canvas.paint();

        JsValue::from_serde(&pixels).unwrap()
    } else {
        JsValue::from_serde(&0).unwrap()
        //
    }
}
