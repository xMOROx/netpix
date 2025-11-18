#![allow(dead_code)]
#![allow(unused_imports)]

use dioxus::prelude::*;

mod app;
mod filter_system;
mod streams;
mod utils;

fn main() {
    // Set up logging
    console_error_panic_hook::set_once();
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));

    // Launch the Dioxus app
    dioxus::launch(app::App);
}
