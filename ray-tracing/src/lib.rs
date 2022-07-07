mod camera;
mod canvas;
mod ray;
mod scene;
mod shape;
mod vec;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(js_name = sharedMemory)]
pub fn shared_memory() -> JsValue {
    wasm_bindgen::memory()
}

#[wasm_bindgen]
pub struct PixelData {
    pub offset: *const u8,
    pub size: usize,
}

#[wasm_bindgen]
impl PixelData {
    pub fn new(bytes: &[u8]) -> PixelData {
        PixelData {
            offset: bytes.as_ptr(),
            size: bytes.len(),
        }
    }

    pub fn offset(&self) -> *const u8 {
        self.offset
    }

    pub fn size(&self) -> usize {
        self.size
    }
}
#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}
