use wasm_bindgen::{prelude::wasm_bindgen, Clamped, JsCast};
mod wasm_modules;

#[wasm_bindgen]
pub fn draw_mandelbrot_set() {
    let document = web_sys::window().unwrap().document().unwrap();
    web_sys::console::log_1(&"drawing start".into());
    let canvas = document
        .get_element_by_id("wasm_canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let canvas_ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let mut result = wasm_modules::generate_mandelbrot_set(
        canvas.width() as usize,
        canvas.height() as usize,
        -1.5,
        0.5,
        -1.0,
        1.0,
        1 << 7 - 1,
    );
    let data = web_sys::ImageData::new_with_u8_clamped_array_and_sh(
        Clamped(&mut result),
        canvas.width(),
        canvas.height(),
    );
    if let Ok(data) = data {
        let _ = canvas_ctx.put_image_data(&data, 0.0, 0.0);
    }
}
