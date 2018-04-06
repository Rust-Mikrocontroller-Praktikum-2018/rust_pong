use minifb::{Key, WindowOptions, Window};

use pong_core::framebuffer::FrameBuffer;
use pong_core::display::Display;


pub struct DefaultDisplay {
    pub window: Window,
}

impl DefaultDisplay {
    pub fn new(name: &str, width: usize, height: usize) -> Self {
        let mut window = Window::new(name, width, height, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

        DefaultDisplay {
            window
        }
    }
}

impl Display for DefaultDisplay {
    fn show(&mut self, frame_buffer: &FrameBuffer) {
        let buffer = &frame_buffer.buffer;
        self.window.update_with_buffer(buffer).unwrap();
    }
}