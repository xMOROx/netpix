use eframe::wasm_bindgen::JsCast;
use eframe::web_sys;
use web_sys::window;

#[cfg(target_arch = "wasm32")]
mod app;
mod streams;
mod utils;

const CANVAS_ID: &str = "the_canvas_id";

#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    let document = web_sys::window()
        .expect("No window")
        .document()
        .expect("No document");

    let canvas = document
        .get_element_by_id(CANVAS_ID)
        .expect("Failed to find the_canvas_id")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("the_canvas_id was not a HtmlCanvasElement");
    canvas.
        set_height(window().unwrap().inner_width().unwrap().as_f64().unwrap() as _);
    canvas.set_width(window().unwrap().inner_width().unwrap().as_f64().unwrap() as _);

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
            )
            .await
            .expect("Error: failed to start eframe");
    });
}

// trick to be able to run tests in CI
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("Only wasm32 target supported");
}
