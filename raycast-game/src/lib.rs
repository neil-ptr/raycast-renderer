mod utils;

use wasm_bindgen::prelude::*;
use web_sys::{console, js_sys::Math::sqrt};

static MAP: &'static [[char; 8]; 8] = &[
    ['w', 'w', 'w', 'w', 'w', 'w', 'w', 'w'],
    ['w', '0', '1', '0', '0', '0', '0', 'w'],
    ['w', '0', '1', '0', '0', '3', '0', 'w'],
    ['w', '0', '1', '0', '0', '0', '0', 'w'],
    ['w', '0', '0', '0', '2', '2', '0', 'w'],
    ['w', '0', '0', '0', '0', '2', '0', 'w'],
    ['w', '0', '0', '0', '0', '2', '0', 'w'],
    ['w', 'w', 'w', 'w', 'w', 'w', 'w', 'w'],
];

static MAP_GRID_CELL_SIZE_PX: f64 = 30.0;

static FOV: f64 = 100.0;
static CANVAS_WIDTH: f64 = 600.0;
static RAYS: f64 = 100.0;
static DEG_RAY: f64 = FOV / RAYS;
static PX_RAY: f64 = CANVAS_WIDTH / RAYS;

static SCREEN_WIDTH: u8 = 64;

struct Vec2D {
    x: f64,
    y: f64,
}

struct Player {
    pos_x: f64,
    pos_y: f64,

    dir_x: f64,
    dir_y: f64,

    camera_x: f64,
    camera_y: f64,
}

enum Side {
    Vertical,
    Horizontal,
}

pub fn cast(x: f64, y: f64, heading: f64) {}

#[wasm_bindgen(start)]
pub fn main() {
    utils::set_panic_hook();

    let document = web_sys::window().unwrap().document().unwrap();

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

    // ### map ###
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
            let fill_style = match *cell {
                'w' => "black",
                '1' => "red",
                '2' => "green",
                '3' => "blue",
                _ => "white",
            };

            map_canvas_2d_context.set_fill_style_str(fill_style);

            let x = col_index as f64 * MAP_GRID_CELL_SIZE_PX;
            let y = row_index as f64 * MAP_GRID_CELL_SIZE_PX;
            map_canvas_2d_context.fill_rect(x, y, MAP_GRID_CELL_SIZE_PX, MAP_GRID_CELL_SIZE_PX);
        }
    }

    // ### player ###
    let player_x: u8 = 4;
    let player_y: u8 = 2;
    let player_dir_x: i8 = -1;
    let player_dir_y: i8 = 0;
    map_canvas_2d_context.set_fill_style_str("red");
    map_canvas_2d_context.fill_rect(
        ((player_x as f64) * MAP_GRID_CELL_SIZE_PX),
        ((player_y as f64) * MAP_GRID_CELL_SIZE_PX),
        6.0,
        6.0,
    );

    let mut time = 0.0;
    let mut prev_time = 0.0;

    let mut player = Player {
        pos_x: 4.0,
        pos_y: 2.0,
        dir_x: 1.0,
        dir_y: 0.0,
        camera_x: 0.0,
        camera_y: 0.66,
    };
    map_canvas_2d_context.begin_path();
    /*     loop { */
    for x in 0..SCREEN_WIDTH {
        // note for me: 2 extends range from [0, 1] to [0, 2],
        // -1 shifts left to [-1, 1]
        let camera_x = 2.0 * (x as f64 / SCREEN_WIDTH as f64) - 1.0;

        let ray_x = player.dir_x + (player.camera_x * camera_x);
        let ray_y = player.dir_y + (player.camera_y * camera_x);

        console::log_2(&ray_x.into(), &ray_y.into());

        let mut map_x = player.pos_x as i8;
        let mut map_y = player.pos_y as i8;

        let mut dist_x: f64;
        let mut dist_y: f64;

        // TODO: using unoptimized verison of this function but only having it for now in this
        // exploratory code
        let delta_dist_x = if ray_x == 0.0 {
            f64::INFINITY
        } else {
            sqrt(f64::powi(1.0, 2) + f64::powi(ray_y / ray_x, 2)).abs()
        };
        let delta_dist_y = if ray_y == 0.0 {
            f64::INFINITY
        } else {
            sqrt(f64::powi(1.0, 2) + f64::powi(ray_x / ray_y, 2)).abs()
        };

        let step_x: i8;
        let step_y: i8;

        let mut wall_was_hit: bool = false;
        let mut side = Side::Vertical;

        if ray_x < 0.0 {
            step_x = -1;
            dist_x = (player.pos_x - map_x as f64) * delta_dist_x;
        } else {
            step_x = 1;
            dist_x = (map_x as f64 + 1.0 - player.pos_x) * delta_dist_x;
        }

        if ray_y < 0.0 {
            step_y = -1;
            dist_y = (player.pos_y - map_y as f64) * delta_dist_y;
        } else {
            step_y = 1;
            dist_y = (map_y as f64 + 1.0 - player.pos_y) * delta_dist_y;
        }

        // DDA alg
        while !wall_was_hit {
            if dist_x < dist_y {
                dist_x += delta_dist_x;
                map_x += step_x;
                side = Side::Vertical;
            } else {
                dist_y += delta_dist_y;
                map_y += step_y;
                side = Side::Vertical;
            }

            if map_x >= 0 && map_y >= 0 && MAP[map_x as usize][map_y as usize] != '0' {
                wall_was_hit = true;
            }
        }

        let perpindicular_wall_dist = match side {
            Side::Vertical => dist_x - delta_dist_x,
            Side::Horizontal => dist_y - delta_dist_y,
        };

        let line_height = game_canvas.height() as f64 / perpindicular_wall_dist;

        let draw_start = (-line_height / 2.0 + game_canvas.height() as f64 / 2.0).max(0.0);
        let draw_end = (line_height / 2.0 + game_canvas.height() as f64 / 2.0)
            .min(game_canvas.height() as f64 - 1.0);

        // Choose color
        let color = match MAP[map_x as usize][map_y as usize] {
            'w' => "black",
            '1' => "red",
            '2' => "green",
            '3' => "blue",
            _ => "white",
        };
        game_canvas_2d_context.set_fill_style_str(color);

        // Draw vertical line
        game_canvas_2d_context.fill_rect(
            x as f64,
            draw_start as f64,
            1.0,
            (draw_end - draw_start) as f64,
        );

        if x % 4 == 0 {
            map_canvas_2d_context.move_to(
                player.pos_x * MAP_GRID_CELL_SIZE_PX,
                player.pos_y * MAP_GRID_CELL_SIZE_PX,
            );
            map_canvas_2d_context.line_to(
                player.pos_x * MAP_GRID_CELL_SIZE_PX + (ray_x * 40.0),
                player.pos_y * MAP_GRID_CELL_SIZE_PX + (ray_y * 40.0),
            );
        }
        map_canvas_2d_context.set_stroke_style_str("blue");
    }
    // }

    map_canvas_2d_context.stroke();
    // let heading = 0.0; //

    // // game
    // let game_canvas_element = document.get_element_by_id("game-viewport").unwrap();
    //
    // let game_canvas: web_sys::HtmlCanvasElement = game_canvas_element
    //     .dyn_into::<web_sys::HtmlCanvasElement>()
    //     .map_err(|_| ())
    //     .unwrap();
    //
    // let game_canvas_width = game_canvas.width();
    // console::log_1(&game_canvas_width.into());
    //
    // let game_canvas_2d_context = game_canvas
    //     .get_context("2d")
    //     .unwrap()
    //     .unwrap()
    //     .dyn_into::<web_sys::CanvasRenderingContext2d>()
    //     .unwrap();
    //
    // game_canvas_2d_context.set_fill_style_str("red");
    //
    // game_canvas_2d_context.fill_rect(0.0, 0.0, 300.0, 300.0);
    //
    // loop {
    //     console::log_1(&"sdlkjf".into());
    // }
}
