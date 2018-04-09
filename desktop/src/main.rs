extern crate minifb;
extern crate rand;
#[macro_use]
extern crate itertools;

extern crate pong_core;

mod display;
mod controller;

use minifb::{Key, WindowOptions, Window};
use rand::{Rng, thread_rng};
use std::time::{SystemTime, UNIX_EPOCH};

use pong_core::pong::GameState;
use pong_core::framebuffer::FrameBuffer;
use pong_core::display::Display;
use pong_core::controller::{Controller, Direction};
use pong_core::renderer::Renderer;
use display::DefaultDisplay;
use controller::DefaultController;

fn main() {
    println!("Hello, world!");

    let mut frame_buffer = FrameBuffer::new(640, 360);
    let mut renderer = Renderer::new();
    let mut display = DefaultDisplay::new("Game", 640, 360, frame_buffer);

    let mut controller_a = DefaultController::new(display.window.clone(), Key::Up, Key::Down);
    let mut controller_b = DefaultController::new(display.window.clone(), Key::W, Key::S);

    let mut game_state = GameState::new();

    let mut start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    while display.window.borrow().is_open() && !display.window.borrow().is_key_down(Key::Escape) {
        let dir_a = controller_a.get_direction();
        let dir_b = controller_b.get_direction();

        let stop = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let t_delta = stop - start;
        start = stop;
        game_state.update(dir_a, dir_b, 1.0);
        renderer.render(&game_state, &mut display);
        display.show();

    }
}
