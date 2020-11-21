use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;


#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

pub fn draw(canvas: &web_sys::HtmlCanvasElement) -> () {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    console_log!("Hello {}!", "world");

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
        .set_attribute("width", &body.client_width().to_string())
        .map_err(|err| console_log!("Error: {}", err.as_string().unwrap()))
        .ok();
    canvas
        .set_attribute("height", &body.client_height().to_string())
        .map_err(|err| console_log!("Error: {}", err.as_string().unwrap()))
        .ok();

    let canvas_onresize = canvas.clone();
    let onresize = Closure::wrap(Box::new(move || {
        canvas_onresize
            .set_attribute("width", &body.client_width().to_string())
            .map_err(|err| console_log!("Error: {}", err.as_string().unwrap()))
            .ok();
        canvas_onresize
            .set_attribute("height", &body.client_height().to_string())
            .map_err(|err| console_log!("Error: {}", err.as_string().unwrap()))
            .ok();
        draw(&canvas_onresize);
    }) as Box<dyn FnMut()>);

    window.set_onresize(Some(onresize.as_ref().unchecked_ref()));
    onresize.forget();

    draw(&canvas);

    Ok(())
}
