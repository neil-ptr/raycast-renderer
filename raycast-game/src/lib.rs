mod assets;
mod input;
mod utils;

use input::{ARROW_LEFT_KEY_IDX, ARROW_RIGHT_KEY_IDX, A_KEY_IDX, D_KEY_IDX, S_KEY_IDX, W_KEY_IDX};
use std::cell::RefCell;
use std::f64::consts::PI;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{js_sys::Uint8Array, Blob};

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}
extern crate console_error_panic_hook;

const MAP: [[char; 24]; 24] = [
    [
        'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w',
        'w', 'w', 'w', 'w', 'w', 'w',
    ],
    [
        'w', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '3', ' ',
        ' ', ' ', ' ', ' ', ' ', 'w',
    ],
    [
        'w', ' ', '3', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', 'w',
    ],
    [
        'w', ' ', '1', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', 'w',
    ],
    [
        'w', ' ', '1', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '3', ' ',
        ' ', ' ', ' ', ' ', ' ', 'w',
    ],
    [
        'w', ' ', '2', ' ', ' ', ' ', ' ', '1', '1', '2', '3', '1', '2', '3', '3', '2', '2', '2',
        ' ', '2', '1', '1', '3', 'w',
    ],
    [
        'w', ' ', '1', ' ', ' ', ' ', ' ', '2', ' ', '2', ' ', '2', ' ', '1', ' ', '1', '3', ' ',
        ' ', ' ', '2', '1', '3', 'w',
    ],
    [
        'w', ' ', '1', ' ', ' ', ' ', ' ', '2', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '2', '2', ' ',
        ' ', ' ', ' ', ' ', ' ', 'w',
    ],
    [
        'w', ' ', '1', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', '3', '1', '3', 'w',
    ],
    [
        'w', ' ', '2', ' ', ' ', ' ', ' ', '2', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '3', '3', ' ',
        ' ', ' ', ' ', ' ', ' ', 'w',
    ],
    [
        'w', ' ', ' ', ' ', ' ', ' ', ' ', '2', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '1', '1', ' ',
        ' ', ' ', '1', '2', '2', 'w',
    ],
    [
        'w', ' ', ' ', ' ', ' ', ' ', ' ', '1', '2', '2', '1', ' ', '1', '2', '3', '1', '3', '2',
        '1', '2', '2', '1', '2', 'w',
    ],
    [
        'w', '3', '3', '1', '1', '3', '2', '3', '3', '3', '3', ' ', '2', '2', '3', '2', '1', '2',
        '1', '2', '3', '1', '1', 'w',
    ],
    [
        'w', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ',
        ' ', ' ', ' ', ' ', ' ', 'w',
    ],
    [
        'w', '1', '1', '1', '3', '1', ' ', '3', '2', '3', '2', ' ', '1', '3', '2', '3', '3', '1',
        '3', '3', '2', '2', '1', 'w',
    ],
    [
        'w', '1', '1', '1', '1', '1', ' ', '3', '1', '2', '3', ' ', '2', '2', '2', '3', '2', '2',
        '3', '2', '2', '2', '3', 'w',
    ],
    [
        'w', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '3', '2', ' ', '1', '2', ' ', ' ', ' ', ' ',
        ' ', '2', ' ', ' ', ' ', 'w',
    ],
    [
        'w', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '2', '1', ' ', ' ', '1', ' ',
        ' ', '2', ' ', ' ', ' ', 'w',
    ],
    [
        'w', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '2', '2', ' ', '2', '1', ' ', ' ', ' ', ' ',
        ' ', '3', '2', ' ', '3', 'w',
    ],
    [
        'w', ' ', '3', ' ', '2', ' ', ' ', ' ', ' ', '1', '3', ' ', ' ', ' ', ' ', ' ', '2', ' ',
        ' ', ' ', ' ', ' ', ' ', 'w',
    ],
    [
        'w', ' ', ' ', '2', ' ', ' ', ' ', ' ', ' ', '3', '3', ' ', '3', '1', ' ', ' ', ' ', ' ',
        ' ', '1', '2', ' ', '1', 'w',
    ],
    [
        'w', ' ', '2', ' ', '2', ' ', ' ', ' ', ' ', '2', '1', ' ', '3', '3', ' ', ' ', '1', ' ',
        ' ', '3', ' ', ' ', ' ', 'w',
    ],
    [
        'w', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', '1', '1', ' ', '3', '2', ' ', ' ', ' ', ' ',
        ' ', '3', ' ', ' ', ' ', 'w',
    ],
    [
        'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w', 'w',
        'w', 'w', 'w', 'w', 'w', 'w',
    ],
];

const MAP_GRID_CELL_SIZE_PX: f64 = 10.0;

struct Vec2D<T> {
    x: T,
    y: T,
}

impl<T> Vec2D<T>
where
    T: MulAssign + AddAssign + Copy,
{
    pub fn scale(&mut self, factor: T) {
        self.x *= factor;
        self.y *= factor;
    }

    pub fn add(&mut self, other: Vec2D<T>) {
        self.x += other.x;
        self.y += other.y;
    }
}

struct Game {
    entities: Vec<Entity>,
    players: Vec<Player>,
}

pub struct Player {
    position: Vec2D<f64>,
    direction: Vec2D<f64>,
    camera: Vec2D<f64>,
    keys_down: [bool; 6], // [w,a,s,d,left,right]
    mouse_velocity: Vec2D<f64>,
}

struct Entity {
    position: Vec2D<f64>,
}

enum Axis {
    Vertical,
    Horizontal,
}

const WALKING_SPEED: f64 = 0.1;
const ROTATION_SPEED: f64 = 2.0 * (PI / 180.0); // 10 degrees converted to radians

#[wasm_bindgen]
pub async fn main() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    // initialize canvases
    let window = Rc::new(RefCell::new(web_sys::window().unwrap()));
    let document = window.borrow().document().unwrap();

    let result = assets::get_asset("backrooms_wallpaper.png".into(), &window.borrow()).await;
    match result {
        Ok(blob_js_val) => {
            let blob = Blob::from(blob_js_val);
            let uint8_array = Uint8Array::new(&blob);
            let u8_vector = uint8_array.to_vec();
            let length = u8_vector.len();
            console_log!("{:?}", length);
        }
        Err(err) => {
            console_log!("{:?}", err);
        }
    }
    // console::log_2(
    //     &String::from_utf8(u8_vector).unwrap().into(),
    //     &length.into(),
    // );

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

    // initialize game state
    let player = Rc::new(RefCell::new(Player {
        position: Vec2D { x: 4.0, y: 4.0 },
        direction: Vec2D { x: 1.0, y: 0.0 },
        camera: Vec2D { x: 0.0, y: 0.66 },
        keys_down: [false, false, false, false, false, false],
        mouse_velocity: Vec2D { x: 0.0, y: 0.0 },
    }));

    input::attach_user_event_handlers(&window, player.clone());

    let f: Rc<RefCell<Option<Closure<dyn Fn()>>>> = Rc::new(RefCell::new(None));
    let g = f.clone();
    let player_clone = player.clone();
    let map_canvas_2d_context_clone = map_canvas_2d_context.clone();
    let window_clone = window.clone();
    // animation loop
    *g.borrow_mut() = Some(Closure::new(move || {
        let mut player = player_clone.borrow_mut();

        // Clear the map canvas
        map_canvas_2d_context_clone.clear_rect(
            0.0,
            0.0,
            map_canvas.width() as f64,
            map_canvas.height() as f64,
        );

        game_canvas_2d_context.clear_rect(
            0.0,
            0.0,
            game_canvas.width() as f64,
            game_canvas.height() as f64,
        );

        // update game state
        // move player
        for (idx, is_key_down) in player.keys_down.clone().iter().enumerate() {
            match (idx, is_key_down) {
                (W_KEY_IDX, true) => {
                    player.position.x += player.direction.x * WALKING_SPEED;
                    player.position.y += player.direction.y * WALKING_SPEED;
                }
                (A_KEY_IDX, true) => {
                    player.position.x += player.direction.y * WALKING_SPEED;
                    player.position.y -= player.direction.x * WALKING_SPEED;
                }
                (S_KEY_IDX, true) => {
                    player.position.x -= player.direction.x * WALKING_SPEED;
                    player.position.y -= player.direction.y * WALKING_SPEED;
                }
                (D_KEY_IDX, true) => {
                    player.position.x -= player.direction.y * WALKING_SPEED;
                    player.position.y += player.direction.x * WALKING_SPEED;
                }
                (ARROW_RIGHT_KEY_IDX, true) => {
                    // Rotate direction
                    let new_dir_x = player.direction.x * f64::cos(ROTATION_SPEED)
                        - player.direction.y * f64::sin(ROTATION_SPEED);
                    let new_dir_y = player.direction.x * f64::sin(ROTATION_SPEED)
                        + player.direction.y * f64::cos(ROTATION_SPEED);
                    player.direction.x = new_dir_x;
                    player.direction.y = new_dir_y;

                    // Rotate camera
                    let new_camera_x = player.camera.x * f64::cos(ROTATION_SPEED)
                        - player.camera.y * f64::sin(ROTATION_SPEED);
                    let new_camera_y = player.camera.x * f64::sin(ROTATION_SPEED)
                        + player.camera.y * f64::cos(ROTATION_SPEED);
                    player.camera.x = new_camera_x;
                    player.camera.y = new_camera_y;
                }
                (ARROW_LEFT_KEY_IDX, true) => {
                    // Rotate direction
                    let new_dir_x = player.direction.x * f64::cos(-ROTATION_SPEED)
                        - player.direction.y * f64::sin(-ROTATION_SPEED);
                    let new_dir_y = player.direction.x * f64::sin(-ROTATION_SPEED)
                        + player.direction.y * f64::cos(-ROTATION_SPEED);
                    player.direction.x = new_dir_x;
                    player.direction.y = new_dir_y;

                    // Rotate camera
                    let new_camera_x = player.camera.x * f64::cos(-ROTATION_SPEED)
                        - player.camera.y * f64::sin(-ROTATION_SPEED);
                    let new_camera_y = player.camera.x * f64::sin(-ROTATION_SPEED)
                        + player.camera.y * f64::cos(-ROTATION_SPEED);
                    player.camera.x = new_camera_x;
                    player.camera.y = new_camera_y;
                }
                _ => {}
            }
        }

        // Redraw the map grid
        for (row_index, row) in MAP.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                let fill_style = match *cell {
                    'w' => "black",
                    '1' => "red",
                    '2' => "green",
                    '3' => "blue",
                    _ => "grey",
                };

                map_canvas_2d_context_clone.set_fill_style_str(fill_style);

                let x = col_index as f64 * MAP_GRID_CELL_SIZE_PX;
                let y = row_index as f64 * MAP_GRID_CELL_SIZE_PX;
                map_canvas_2d_context_clone.fill_rect(
                    x,
                    y,
                    MAP_GRID_CELL_SIZE_PX,
                    MAP_GRID_CELL_SIZE_PX,
                );
            }
        }

        map_canvas_2d_context_clone.begin_path();
        map_canvas_2d_context_clone.move_to(
            player.position.x * MAP_GRID_CELL_SIZE_PX,
            player.position.y * MAP_GRID_CELL_SIZE_PX,
        );
        map_canvas_2d_context_clone.line_to(
            (player.position.x * MAP_GRID_CELL_SIZE_PX)
                + (player.direction.x * MAP_GRID_CELL_SIZE_PX),
            (player.position.y) * MAP_GRID_CELL_SIZE_PX
                + (player.direction.y) * MAP_GRID_CELL_SIZE_PX,
        );
        map_canvas_2d_context_clone.stroke();

        map_canvas_2d_context_clone.set_fill_style_str("rebeccapurple");

        map_canvas_2d_context_clone.fill_rect(
            player.position.x * MAP_GRID_CELL_SIZE_PX,
            player.position.y * MAP_GRID_CELL_SIZE_PX,
            6.0,
            6.0,
        );

        // ray casting
        let camera_width = game_canvas.width();
        for ray_idx in 0..camera_width {
            let camera_x = ((ray_idx as f64 / camera_width as f64) * 2.0) - 1.0;

            // TODO: should probably implement vector methods so intentinallity is obvious (ie the
            // scaling done to camera vector below)
            let ray_direction = Vec2D {
                x: (player.camera.x * camera_x) + player.direction.x,
                y: (player.camera.y * camera_x) + player.direction.y,
            };

            // distance between 2 x and y intersects
            let delta_x_intersect = if ray_direction.x == 0.0 {
                f64::INFINITY
            } else {
                f64::abs(1.0 / ray_direction.x)
            };
            let delta_y_intersect = if ray_direction.y == 0.0 {
                f64::INFINITY
            } else {
                f64::abs(1.0 / ray_direction.y)
            };

            let mut map_cell_position_x = player.position.x as i32;
            let mut map_cell_position_y = player.position.y as i32;

            let step_x: i32;
            let step_y: i32;

            let mut side_dist_x = if ray_direction.x >= 0.0 {
                step_x = 1;
                ((map_cell_position_x + 1) as f64 - player.position.x) * delta_x_intersect
            } else {
                step_x = -1;
                (player.position.x - map_cell_position_x as f64) * delta_x_intersect
            };
            let mut side_dist_y = if ray_direction.y >= 0.0 {
                step_y = 1;
                ((map_cell_position_y + 1) as f64 - player.position.y) * delta_y_intersect
            } else {
                step_y = -1;
                (player.position.y - map_cell_position_y as f64) * delta_y_intersect
            };

            let mut wall_was_hit = false;
            let mut axis_intersected = Axis::Vertical;
            while !wall_was_hit {
                if side_dist_x <= side_dist_y {
                    map_cell_position_x += step_x;
                    side_dist_x += delta_x_intersect;
                    axis_intersected = Axis::Vertical;
                } else {
                    map_cell_position_y += step_y;
                    side_dist_y += delta_y_intersect;
                    axis_intersected = Axis::Horizontal;
                }

                if MAP[map_cell_position_y as usize][map_cell_position_x as usize] != ' ' {
                    wall_was_hit = true;
                }
            }

            let perpendicular_dist = match axis_intersected {
                Axis::Vertical => side_dist_x - delta_x_intersect,
                Axis::Horizontal => side_dist_y - delta_y_intersect,
            };

            let line_height = game_canvas.height() as f64 / perpendicular_dist;
            let center_screen = game_canvas.height() as f64 / 2.0;
            let line_start = center_screen - (line_height / 2.0);
            let line_end = line_start + line_height;

            game_canvas_2d_context.begin_path();
            let base_color = match MAP[map_cell_position_y as usize][map_cell_position_x as usize] {
                '1' => "red",
                '2' => "green",
                '3' => "blue",
                _ => "black",
            };
            let color = match axis_intersected {
                Axis::Vertical => base_color,
                Axis::Horizontal => match base_color {
                    "red" => "#8B0000",
                    "green" => "#006400",
                    "blue" => "#00008B",
                    _ => "black",
                },
            };
            game_canvas_2d_context.set_stroke_style_str(color);
            game_canvas_2d_context.move_to(ray_idx as f64, line_start);
            game_canvas_2d_context.line_to(ray_idx as f64, line_end);
            game_canvas_2d_context.stroke();
        }

        window_clone
            .borrow()
            .request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }));

    window
        .borrow()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .unwrap();
}
