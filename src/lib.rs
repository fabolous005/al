use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen]
pub struct App {
    scale: f64,
    width: u32,
    height: u32,
    context: CanvasRenderingContext2d,
}

#[wasm_bindgen]
pub fn new_app() -> App {
    // Get the canvas element from the DOM
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document.get_element_by_id("myCanvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    canvas.set_width(window.inner_width().unwrap().as_f64().unwrap() as u32);
    canvas.set_height(window.inner_height().unwrap().as_f64().unwrap() as u32);

    App {
        scale: 0.5,
        width: window.inner_width().unwrap().as_f64().unwrap() as u32,
        height: window.inner_height().unwrap().as_f64().unwrap() as u32,
        context,
    }
}

#[wasm_bindgen]
pub fn draw_grid(app: App) {
    let mut x_pos: f64 = 0.0;
    let mut y_pos: f64 = 0.0;
    app.context.move_to(0 as f64, 0 as f64);
    for _ in 1..app.width {
        y_pos += app.scale;
        app.context.move_to(x_pos, y_pos);
        app.context.line_to(app.width as f64, y_pos);
    }
    y_pos = 0.0;
    for _ in 1..app.height {
        x_pos += app.scale;
        app.context.move_to(x_pos, y_pos);
        app.context.line_to(app.width as f64, y_pos);
    }
    app.context.set_stroke_style(&JsValue::from_str("black"));
    app.context.stroke();
}
