extern crate minifb;
extern crate rand;
extern crate itertools;

extern crate pong_core;

mod display;
mod controller;

use minifb::{Key, WindowOptions, Window};
use rand::{Rng, thread_rng};
use std::time::Instant;

use pong_core::math::{Vector, unit, InvSqrt32};
use pong_core::pong::{GameState, Game};
use pong_core::framebuffer::FrameBuffer;
use pong_core::display::Display;
use pong_core::controller::{Controller, Direction};
use pong_core::renderer::Renderer;
use display::DefaultDisplay;
use controller::DefaultController;

fn main() {
    let frame_buffer = FrameBuffer::new(640, 360);
    let mut renderer = Renderer::new();
    let mut display = DefaultDisplay::new("Game", 640, 360, frame_buffer);

    let mut controller_a = DefaultController::new(display.window.clone(), Key::W, Key::S);
    let mut controller_b = DefaultController::new(display.window.clone(), Key::Up, Key::Down);

    let mut game_state = GameState::new(640.0, 360.0);
    let game = Game::new(640.0, 360.0);

    let mut t = 0.0;
    let mut u = 0.0;

    let mut start = Instant::now();
    while display.window.borrow().is_open() && !display.window.borrow().is_key_down(Key::Escape) {
        let dir_a = controller_a.get_direction();
        let dir_b = controller_b.get_direction();

        let t_delta = start.elapsed();
        let t_delta = (t_delta.as_secs() * 1000) as f32 + (t_delta.subsec_nanos() / 1000000) as f32;
        start = Instant::now();
        game_state = game.update(game_state, dir_a, dir_b, t_delta / 5.0);
        //println!("{:?}", game_state);
        renderer.render(&game_state, &mut display);
        display.show();

    }
}
