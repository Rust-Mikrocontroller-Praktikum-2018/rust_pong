extern crate minifb;
extern crate rand;
#[macro_use]
extern crate itertools;

extern crate pong_core;

mod display;
mod controller;

use minifb::{Key, WindowOptions, Window};
use rand::{Rng, thread_rng};

use pong_core::framebuffer::FrameBuffer;
use pong_core::display::Display;
use pong_core::controller::Controller;
use display::DefaultDisplay;

fn main() {
    println!("Hello, world!");

    let mut frame_buffer = FrameBuffer::new(640, 360);
    let mut display = DefaultDisplay::new("Game", 640, 360);

    let mut rng = thread_rng();

    while display.window.is_open() && !display.window.is_key_down(Key::Escape) {
        let mut v: u32 = rng.gen();

        for (y, x) in iproduct!((0..360), (0..640)) {
            if (y*360+x) % 50000 == 0 {
                v = rng.gen();
            }

            frame_buffer.set_pixel(v, x, y);
            if display.window.is_key_down(Key::Up) {
                frame_buffer.set_pixel(0, x, y);
            }
        }



        display.show(&frame_buffer);

    }
}
