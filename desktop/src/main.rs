extern crate minifb;
extern crate rand;
#[macro_use]
extern crate itertools;

extern crate pong_core;

mod display;
mod controller;

use minifb::{Key, WindowOptions, Window};
use rand::{Rng, thread_rng};

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
    let renderer = Renderer::new();
    let mut display = DefaultDisplay::new("Game", 640, 360);

    let mut controller_a = DefaultController::new(display.window.clone(), Key::Up, Key::Down);
    let mut controller_b = DefaultController::new(display.window.clone(), Key::W, Key::S);

    let mut game_state = GameState::new();

    while display.window.borrow().is_open() && !display.window.borrow().is_key_down(Key::Escape) {
        let dir = controller_a.get_direction();

        let string = match dir {
            Direction::None => "a:None",
            Direction::Up => "a:Up",
            Direction::Down => "a:Down",
        };

        println!("{}", string);

        let dir = controller_b.get_direction();

        let string = match dir {
            Direction::None => "b:None",
            Direction::Up => "b:Up",
            Direction::Down => "b:Down",
        };

        game_state.update(Direction::Up, Direction::Down, 1);
        renderer.render(&game_state, &mut frame_buffer);
        display.show(&mut frame_buffer);

    }
}
