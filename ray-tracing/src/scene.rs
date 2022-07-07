use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    camera::{Camera, RelativeDirection},
    canvas::Canvas,
    shape::Shape,
    vec::{Point3, Vec3},
    PixelData,
};

#[wasm_bindgen]
pub struct Scene {
    canvas: Canvas,
    camera: Camera,
}

#[wasm_bindgen]
impl Scene {
    pub fn new(
        width: i32,
        viewport_height: i32,
        aspect_ratio: f32,
        focal_length: f32,
        origin: Vec<f32>,
        rotation: Vec<f32>,
    ) -> Self {
        console_error_panic_hook::set_once();

        let shapes = vec![
            Shape::Sphere {
                center: Vec3::new(0.0, 0.0, 0.0),
                radius: 0.5,
                color: Vec3::new(0.3, 1.0, 0.3),
            },
            Shape::Sphere {
                center: Vec3::new(2.0, 0.0, 0.0),
                radius: 0.2,
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
            if let [rx, ry, rz] = rotation[..] {
                let rotation = Vec3::new(rx, ry, rz);
                let camera = Camera::new(
                    aspect_ratio,
                    viewport_height,
                    focal_length,
                    origin,
                    rotation,
                );
                let canvas = Canvas::new(width as usize, aspect_ratio, shapes);
                return Self { camera, canvas };
                // let pixels = canvas.paint();

                // return PixelData::new(&pixels);
                // JsValue::from_serde(&pixels).unwrap()
            }
        }
        panic!("AA")
    }

    pub fn move_along(&mut self, direction: RelativeDirection) {
        self.camera.move_along(direction)
    }

    pub fn render(&self) -> PixelData {
        let pixels = &self.canvas.paint(&self.camera);
        PixelData::new(pixels)
    }

    #[wasm_bindgen(js_name = changeWidth)]
    pub fn change_width(&mut self, width: usize) {
        self.canvas.resize(width)
    }
}
