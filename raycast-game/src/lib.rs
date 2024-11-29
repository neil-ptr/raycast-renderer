mod utils;

use wasm_bindgen::prelude::*;
use web_sys::console;

static MAP: &'static [[char; 8]; 8] = &[
    ['w', 'w', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', '0', 'w', '0', '0', '0', '0', 'w'],
    ['w', '0', 'w', '0', '0', 'w', '0', 'w'],
    ['w', '0', 'w', '0', '0', '0', '0', 'w'],
    ['w', '0', '0', '0', 'w', 'w', '0', 'w'],
    ['w', '0', '0', '0', '0', 'w', '0', 'w'],
    ['w', '0', '0', '0', '0', 'w', '0', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', 'w', 'w'],
];

static MAP_GRID_CELL_SIZE_PX: f64 = 30.0;

static FOV: f64 = 100.0;
static CANVAS_WIDTH: f64 = 600.0;
static RAYS: f64 = 100.0;
static DEG_RAY: f64 = FOV / RAYS;
static PX_RAY: f64 = CANVAS_WIDTH / RAYS;

static PLAYER_X: u8 = 4;
static PLAYER_Y: u8 = 2;

pub fn cast(x: f64, y: f64, heading: f64) {}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();

    let map_canvas_element = document.get_element_by_id("map").unwrap();
    let map_canvas: web_sys::HtmlCanvasElement = map_canvas_element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let map_canvas_2d_context = map_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    for (row_index, row) in MAP.iter().enumerate() {
        for (col_index, cell) in row.iter().enumerate() {
            if *cell == 'w' {
                map_canvas_2d_context.set_fill_style_str("black");
            } else {
                map_canvas_2d_context.set_fill_style_str("white");
            }

            let x = col_index as f64 * MAP_GRID_CELL_SIZE_PX;
            let y = row_index as f64 * MAP_GRID_CELL_SIZE_PX;
            map_canvas_2d_context.fill_rect(x, y, MAP_GRID_CELL_SIZE_PX, MAP_GRID_CELL_SIZE_PX);
        }
    }

    let game_canvas_element = document.get_element_by_id("game-viewport").unwrap();

    let game_canvas: web_sys::HtmlCanvasElement = game_canvas_element
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let game_canvas_2d_context = game_canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    game_canvas_2d_context.set_fill_style_str("red");

    game_canvas_2d_context.fill_rect(0.0, 0.0, 300.0, 300.0);

    // loop {
    //     console::log_1(&"sdlkjf".into());
    // }
}
