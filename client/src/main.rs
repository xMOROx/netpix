#![allow(dead_code)]
#![allow(unused_imports)]

#[cfg(target_arch = "wasm32")]
use eframe::wasm_bindgen::JsCast;
#[cfg(target_arch = "wasm32")]
use eframe::web_sys;
#[cfg(target_arch = "wasm32")]
use egui::TextBuffer;

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

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id(CANVAS_ID)
            .unwrap_or_else(|| panic!("Failed to find {CANVAS_ID}"))
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap_or_else(|_| panic!("{CANVAS_ID} , was not a HtmlCanvasElement"));
        canvas.set_height(document.body().unwrap().client_height() as u32);
        canvas.set_width(document.body().unwrap().client_width() as u32);

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}

// trick to be able to run tests in CI
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    panic!("Only wasm32 target supported");
}
