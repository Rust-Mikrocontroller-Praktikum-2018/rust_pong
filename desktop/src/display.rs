use std::rc::Rc;
use std::cell::RefCell;

use minifb::{Key, WindowOptions, Window};

use pong_core::framebuffer::FrameBuffer;
use pong_core::display::Display;


pub struct DefaultDisplay {
    pub window: Rc<RefCell<Window>>,
    frame_buffer: FrameBuffer,
}

impl DefaultDisplay {
    pub fn new(name: &str, width: usize, height: usize, frame_buffer: FrameBuffer) -> Self {
        let window = Window::new(name, width, height, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        let rc_window = Rc::new(RefCell::new(window));

        DefaultDisplay {
            window: rc_window,
            frame_buffer
        }
    }

    pub fn show(&self) {
        let buffer = &self.frame_buffer.buffer;
        self.window.borrow_mut().update_with_buffer(buffer).unwrap();
    }

}

impl Display for DefaultDisplay {
    fn set_pixel(&mut self, x: usize, y: usize, hex_color: u32) {
        self.frame_buffer.set_pixel(hex_color, x, y);
    }

    fn show_score(&mut self, score_1: usize, score_2: usize, hex_color: u32) {

    }
}