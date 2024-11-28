use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;
use rand::prelude::*;

/* ====== delete =======
// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
*/

// This is like the `main` function, except for JavaScript.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let top = (300.0, 0.0);
    let left = (0.0, 600.0);
    let right = (600.0, 600.0);
    sierpinski(&context, 5, top, left, right);

    // Your code goes here!
    console::log_1(&JsValue::from_str("Hello world!"));

    Ok(())
}

fn sierpinski(context: &web_sys::CanvasRenderingContext2d, depth: usize, top: (f64, f64), left: (f64, f64), ritgh: (f64, f64)) {
    draw_triangle(context, depth, top, left, ritgh);
}

fn make_random_color() -> String {
    let mut rng = thread_rng();
    let r = rng.gen_range(0..255);
    let g = rng.gen_range(0..255);
    let b = rng.gen_range(0..255);
    format!("rgb({}, {}, {})", r, g, b)
}

fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, depth: usize, top: (f64, f64), left: (f64, f64), ritgh: (f64, f64)) {
    context.move_to(top.0, top.1);
    context.begin_path();
    context.line_to(ritgh.0, ritgh.1);
    context.line_to(left.0, left.1);
    context.line_to(top.0, top.1);
    context.close_path();
    context.stroke();

    let color = make_random_color();
    context.set_fill_style_str(&color);
    context.fill();

    if depth <= 0 {
        return;
    }

    draw_triangle(context,  depth - 1, top, middle_point(top, left), middle_point(top, ritgh));
    draw_triangle(context,  depth - 1, middle_point(top, left), left, middle_point(left, ritgh));
    draw_triangle(context,  depth - 1, middle_point(top, ritgh), middle_point(left, ritgh), ritgh);
}

fn middle_point(p1: (f64, f64), p2: (f64, f64)) -> (f64, f64) {
    ((p1.0 + p2.0) * 0.5, (p1.1 + p2.1) * 0.5)
}