use wasm_bindgen::prelude::*;

mod enums;
mod exports;
mod grids;
mod nodes;
mod traits;
mod types;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = game)]
    pub static GAME: JsValue;
}

extern crate web_sys;
#[macro_export]
macro_rules! log {
    ($($t:tt)*) => {
        web_sys::console::log_1(&format!( $ ( $t )* ).into());
    };
}

#[wasm_bindgen(start)]
fn start() {
    console_error_panic_hook::set_once();
}
