use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use web_sys::{KeyboardEvent, MouseEvent};

use crate::Player;
use std::cell::RefCell;
use std::rc::Rc;

const W_KEY_IDX: usize = 0;
const A_KEY_IDX: usize = 1;
const S_KEY_IDX: usize = 2;
const D_KEY_IDX: usize = 3;
const ARROW_LEFT_KEY_IDX: usize = 4;
const ARROW_RIGHT_KEY_IDX: usize = 5;

pub fn attach_event_handlers(window: &Rc<RefCell<web_sys::Window>>, player: Rc<RefCell<Player>>) {
    // Mouse move handler
    let player_clone = player.clone();
    let movemove_handler = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
        let mut player = player_clone.borrow_mut();
        player.mouse_velocity.x = event.movement_x() as f64;
        player.mouse_velocity.y = event.movement_y() as f64;
    });

    // Key up handler
    let player_clone = player.clone();
    let keyup_handler = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        let mut player = player_clone.borrow_mut();
        match event.key().as_str() {
            "w" => player.keys_down[W_KEY_IDX] = false,
            "a" => player.keys_down[A_KEY_IDX] = false,
            "s" => player.keys_down[S_KEY_IDX] = false,
            "d" => player.keys_down[D_KEY_IDX] = false,
            "ArrowRight" => player.keys_down[ARROW_RIGHT_KEY_IDX] = false,
            "ArrowLeft" => player.keys_down[ARROW_LEFT_KEY_IDX] = false,
            _ => {}
        }
    });

    // Key down handler
    let player_clone = player.clone();
    let keydown_handler = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
        let mut player = player_clone.borrow_mut();
        match event.key().as_str() {
            "w" => player.keys_down[W_KEY_IDX] = true,
            "a" => player.keys_down[A_KEY_IDX] = true,
            "s" => player.keys_down[S_KEY_IDX] = true,
            "d" => player.keys_down[D_KEY_IDX] = true,
            "ArrowRight" => player.keys_down[ARROW_RIGHT_KEY_IDX] = true,
            "ArrowLeft" => player.keys_down[ARROW_LEFT_KEY_IDX] = true,
            _ => {}
        }
    });

    // Attach handlers to the window
    let _ = window
        .borrow()
        .add_event_listener_with_callback("mousemove", movemove_handler.as_ref().unchecked_ref());
    let _ = window
        .borrow()
        .add_event_listener_with_callback("keyup", keyup_handler.as_ref().unchecked_ref());
    let _ = window
        .borrow()
        .add_event_listener_with_callback("keydown", keydown_handler.as_ref().unchecked_ref());

    // Prevent closures from being dropped
    movemove_handler.forget();
    keyup_handler.forget();
    keydown_handler.forget();
}
