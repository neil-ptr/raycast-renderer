mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document.get_element_by_id("game-viewport").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context.set_fill_style_str("red");

    context.fill_rect(50.0, 50.0, 100.0, 100.0);

    let map: [[char; 8]; 8] = [
        ['w', 'w', 'w', 'w', 'w', 'w', 'w', 'w'],
        ['w', '0', '0', '0', '0', '0', '0', 'w'],
        ['w', '0', '0', '0', '0', '0', '0', 'w'],
        ['w', '0', '0', '0', '0', '0', '0', 'w'],
        ['w', '0', '0', '0', '0', '0', '0', 'w'],
        ['w', '0', '0', '0', '0', '0', '0', 'w'],
        ['w', '0', '0', '0', '0', '0', '0', 'w'],
        ['w', 'w', 'w', 'w', 'w', 'w', 'w', 'w'],
    ];
}
